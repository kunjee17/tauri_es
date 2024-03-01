use cosmo_store::common::i64_event_version::EventVersion;
use cosmo_store::types::event_read_range::EventsReadRange;
use crate::types::address::Address;
use uuid::Uuid;
use crate::types::patient_db::PatientMeta;

pub type StreamId = String;

#[derive(Clone, Debug)]
pub struct AddPatient {
    pub(crate) id: Uuid,
    pub(crate) stream_id: StreamId,
    pub(crate) name: String,
    pub(crate) address: Address,
    pub(crate) version: i64,
    pub(crate) age: i32,
    pub(crate) phone: String,
    pub(crate) email: String,
}

#[derive(Clone, Debug)]
pub struct UpdatePatient {
    pub(crate) id: Uuid,
    pub(crate) stream_id: StreamId,
    pub(crate) version: i64,
    pub(crate) name: String,
    pub(crate) age: i32,
    pub(crate) phone: String,
    pub(crate) email: String,
}

#[derive(Clone, Debug)]
pub struct UpdatePatientAddress {
    pub(crate) id: Uuid,
    pub(crate) stream_id: StreamId,
    pub(crate) version: i64,
    pub(crate) address: Address,
}

#[derive(Clone, Debug)]
pub enum PatientCommand {
    AddPatient(AddPatient),
    UpdatePatient(UpdatePatient),
    UpdatePatientAddress(UpdatePatientAddress),
}

impl From<PatientCommand> for PatientMeta {
    fn from(value: PatientCommand) -> Self {
        match value {
            PatientCommand::AddPatient(p) => PatientMeta {
                id: p.id,
                stream_id: p.stream_id,
                version: p.version,
            },
            PatientCommand::UpdatePatient(p) => PatientMeta {
                id: p.id,
                stream_id: p.stream_id,
                version: p.version,
            },
            PatientCommand::UpdatePatientAddress(p) => PatientMeta {
                id: p.id,
                stream_id: p.stream_id,
                version: p.version,
            },
        }
    }
}

impl From<PatientCommand> for StreamId {
    fn from(value: PatientCommand) -> Self {
        match value {
            PatientCommand::AddPatient(p) => p.stream_id,
            PatientCommand::UpdatePatient(p) => p.stream_id,
            PatientCommand::UpdatePatientAddress(p) => p.stream_id,
        }
    }
}

impl From<PatientCommand> for EventsReadRange<EventVersion> {
    fn from(value: PatientCommand) -> Self {
        match value {
            PatientCommand::AddPatient(p) => EventsReadRange::FromVersion(EventVersion(p.version)),
            PatientCommand::UpdatePatient(p) => EventsReadRange::FromVersion(EventVersion(p.version)),
            PatientCommand::UpdatePatientAddress(p) => {
                EventsReadRange::FromVersion(EventVersion(p.version))
            }
        }
    }
}

