use crate::types::address::Address;
use cosmo_store::types::event_write::EventWrite;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatientAdded {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) version: i64,
    pub(crate) address: Address,
    pub(crate) age: i32,
    pub(crate) phone: String,
    pub(crate) email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatientUpdated {
    pub(crate) name: String,
    pub(crate) age: i32,
    pub(crate) phone: String,
    pub(crate) email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatientAddressUpdated {
    pub(crate) address: Address,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PatientEvent {
    PatientAdded(PatientAdded),
    PatientUpdated(PatientUpdated),
    PatientAddressUpdated(PatientAddressUpdated),
}

impl From<PatientEvent> for EventWrite<PatientEvent, PatientEvent> {
    fn from(value: PatientEvent) -> Self {
        EventWrite {
            id: Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            name: "patient_event".to_string(),
            data: value,
            metadata: None,
        }
    }
}
