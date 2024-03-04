use crate::db_helpers::upsert_patient;
use crate::types::address::Address;
use crate::types::aggregate::PATIENT_AGGREGATE;
use crate::types::commands::{PatientCommand, StreamId};
use crate::types::events::PatientEvent;
use crate::types::patient::Patient;
use crate::types::patient_db::{AddressDB, PatientDB, PatientMeta};
use anyhow::{bail, Result};
use cosmo_store::common::i64_event_version::EventVersion;
use cosmo_store::traits::event_store::EventStore;
use cosmo_store::types::event_read::EventRead;
use cosmo_store::types::event_read_range::EventsReadRange;
use cosmo_store::types::event_write::EventWrite;
use cosmo_store::types::expected_version::ExpectedVersion;
use cosmo_store_sqlx_sqlite::event_store_sqlx_sqlite::EventStoreSQLXSqlite;
use cosmo_store_util::aggregate::Aggregate;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;


//TODO: Copy of original Make Handler function that I m trying to make it work
pub async fn make_handler<State, Command, Event, Meta, Version>(
    aggregate: &dyn Aggregate<State, Command, Event>,
    store: &dyn EventStore<Event, Meta, Version>,
    command: &Command,
    stream_id: &str,
    range: &EventsReadRange<Version>,
    expected_version: &ExpectedVersion<Version>,
) -> Result<Vec<EventRead<Event, Meta, Version>>>
where
    Version: Eq + PartialEq,
    Event: Into<EventWrite<Event, Meta>> + Clone + Serialize + for<'de> Deserialize<'de>,
    Meta: Clone + Serialize + for<'de> Deserialize<'de>,
{
    let events = store.get_events(stream_id, range).await?;
    let state = events
        .iter()
        .fold(aggregate.init(), |a, b| aggregate.apply(a, &b.data));
    let new_events = aggregate
        .execute(&state, command)?
        .iter()
        .map(|x| x.clone().into())
        .collect();
    store
        .append_events(stream_id, expected_version, new_events)
        .await
}

pub async fn process_patient_command(
    store: EventStoreSQLXSqlite,
    patient_command: &PatientCommand,
) -> Result<Vec<EventRead<PatientEvent, PatientEvent, EventVersion>>> {
    let stream_id = StreamId::from(patient_command.clone());

    let events_read_range = EventsReadRange::from(patient_command.clone());

    make_handler(
        &PATIENT_AGGREGATE,
        &store,
        &patient_command,
        &stream_id,
        &events_read_range,
        &ExpectedVersion::Any,
    )
    .await
}

pub async fn process_patient_events(
    read_pool: Pool<Sqlite>,
    patient_id: Uuid,
    patient_stream_id: String,
    read_events: Vec<EventRead<PatientEvent, PatientEvent, EventVersion>>,
) -> Result<()> {
    let patient_db = sqlx::query_as::<_, PatientDB>("SELECT * from Patient WHERE id = ? LIMIT 1")
        .bind(patient_id)
        .fetch_optional(&read_pool)
        .await?;
    let address_db =
        sqlx::query_as::<_, AddressDB>("SELECT * from Address WHERE patient_id = ? LIMIT 1")
            .bind(patient_id)
            .fetch_optional(&read_pool)
            .await?;

    let patient_state: Option<Patient> = match &patient_db {
        Some(p) => Some(Patient {
            id: p.clone().id,
            name: p.clone().name,
            address: Address {
                street: address_db.clone().unwrap().street,
                city: address_db.clone().unwrap().city,
                state: address_db.clone().unwrap().state,
                zip: address_db.clone().unwrap().zip,
            },
            age: p.clone().age,
            phone: p.clone().phone,
            email: p.clone().email,
        }),
        None => None,
    };
    let patient_updated_state = read_events
        .iter()
        .fold(patient_state, |a, b| PATIENT_AGGREGATE.apply(a, &b.data));

    println!(
        "version {:#?}",
        read_events
            .last()
            .map_or_else(|| 0, |event| event.version.0)
    );

    println!("patient_updated_state: {:#?}", patient_updated_state);
    match patient_updated_state {
        Some(p) => {
            upsert_patient(
                read_pool,
                p,
                read_events
                    .last()
                    .map_or_else(|| 0, |event| event.version.0),
                patient_stream_id,
            )
            .await?;
        }
        None => {
            bail!("Patient not found");
        }
    }
    Ok(())
}

pub async fn get_patient_meta(read_pool: Pool<Sqlite>) -> Result<Option<PatientMeta>> {
    let patient_meta =
        sqlx::query_as::<_, PatientMeta>("SELECT id, stream_id, version from Patient LIMIT 1")
            .fetch_optional(&read_pool)
            .await?;
    Ok(patient_meta)
}
