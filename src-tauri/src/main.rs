// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db_helpers;
mod patient_helper;
mod types;
use std::sync::{Arc, Mutex};

use crate::patient_helper::{
    get_patient_meta, make_handler, process_patient_command, process_patient_events,
};
use crate::types::address::Address;
use crate::types::commands::{
    AddPatient, PatientCommand, StreamId, UpdatePatient, UpdatePatientAddress,
};
use anyhow::Result;
use cosmo_store::common::i64_event_version::EventVersion;
use cosmo_store::traits::event_store::EventStore;
use cosmo_store::types::event_read::EventRead;
use cosmo_store::types::event_read_range::EventsReadRange;
use cosmo_store::types::expected_version::ExpectedVersion;
use cosmo_store_sqlx_sqlite::event_store_sqlx_sqlite::EventStoreSQLXSqlite;
use cosmo_store_util::aggregate::Aggregate;
use sqlx::sqlite::SqlitePoolOptions;
use tauri::State;
use types::aggregate::PATIENT_AGGREGATE;
use types::events::PatientEvent;
use uuid::Uuid;

use crate::db_helpers::{recreate_database, setup_read_db};

struct AppState {
    read_db_pool: sqlx::SqlitePool,
    write_db_pool: sqlx::SqlitePool,
    store: EventStoreSQLXSqlite,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_patients() -> String {
    format!("patient list")
}

#[tauri::command]
async fn add_patient<'a>(state: State<'a, AppState>) -> Result<String, tauri::Error> {
    let store = &state.store;
    let store = store.clone();

    let new_patient_id = Uuid::new_v4();
    let new_patient_stream_id = format!("patient-{}", Uuid::new_v4().to_string());
    let patient_command = PatientCommand::AddPatient(AddPatient {
        id: new_patient_id.clone(),
        stream_id: new_patient_stream_id.clone(),
        name: "John Doe".to_string(),
        address: Address {
            street: "123 Main St".to_string(),
            city: "Anytown".to_string(),
            state: "NY".to_string(),
            zip: "12345".to_string(),
        },
        version: 0,
        age: 42,
        phone: "555-123-4567".to_string(),
        email: "".to_string(),
    });
    let stream_id = StreamId::from(patient_command.clone());

    let events_read_range = EventsReadRange::from(patient_command.clone());
    let aggregate = PATIENT_AGGREGATE.clone();
    let res = make_handler(
        &aggregate,
        &store,
        &patient_command,
        &stream_id,
        &events_read_range,
        &ExpectedVersion::Any,
    )
    .await?;

    // Simple code works as below. But generic is not working
    // let events: Vec<EventRead<PatientEvent, PatientEvent, EventVersion>> = store.get_events(&stream_id, &events_read_range).await?;
    // let state = events
    //     .iter()
    //     .fold(aggregate.init(), |a, b| aggregate.apply(a, &b.data));
    // let new_events = aggregate
    //     .execute(&state, &patient_command)?
    //     .iter()
    //     .map(|x| x.clone().into())
    //     .collect();
    // let res = store
    //     .append_events(&stream_id, &ExpectedVersion::Any, new_events)
    //     .await?;

    println!("res {:#?}", res);

    Ok(format!("patient added"))
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    println!("Hello, world!");

    let write_db_conn = format!("sqlite://{}", "write.db");
    let read_db_conn = format!("sqlite://{}", "read.db");

    recreate_database(&write_db_conn).await?;
    recreate_database(&read_db_conn).await?;

    let write_pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&write_db_conn)
        .await?;
    let read_pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&read_db_conn)
        .await?;

    // Create Patient and Address Read Table
    setup_read_db(read_pool.clone()).await?;
    let store = EventStoreSQLXSqlite::new(&write_pool, "tauri_store").await?;

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            read_db_pool: read_pool,
            write_db_pool: write_pool,
            store: store.clone(),
        })
        .invoke_handler(tauri::generate_handler![greet, add_patient, get_patients])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
