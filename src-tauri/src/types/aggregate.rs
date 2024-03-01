use crate::types::commands::PatientCommand;
use crate::types::events::{PatientAdded, PatientAddressUpdated, PatientEvent, PatientUpdated};
use crate::types::patient::Patient;
use cosmo_store_util::aggregate::Aggregate;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct PatientAggregate {}

impl Aggregate<Option<Patient>, PatientCommand, PatientEvent> for PatientAggregate {
    fn init(&self) -> Option<Patient> {
        None
    }

    fn apply(&self, state: Option<Patient>, event: &PatientEvent) -> Option<Patient> {
        match event {
            PatientEvent::PatientAdded(p) => Some(Patient {
                id: p.id.clone(),
                name: p.name.clone(),
                address: p.address.clone(),
                age: p.age,
                phone: p.phone.clone(),
                email: p.email.clone(),
            }),
            PatientEvent::PatientUpdated(p) => match state {
                None => return None,
                Some(state) => Some(Patient {
                    id: state.id.clone(),
                    name: p.name.clone(),
                    address: state.address.clone(),
                    age: p.age,
                    phone: p.phone.clone(),
                    email: p.email.clone(),
                }),
            },
            PatientEvent::PatientAddressUpdated(a) => match state {
                None => return None,
                Some(state) => Some(Patient {
                    id: state.id.clone(),
                    name: state.name.clone(),
                    address: a.address.clone(),
                    age: state.age,
                    phone: state.phone.clone(),
                    email: state.email.clone(),
                }),
            },
        }
    }

    fn execute(
        &self,
        state: &Option<Patient>,
        command: &PatientCommand,
    ) -> anyhow::Result<Vec<PatientEvent>> {
        match command {
            PatientCommand::AddPatient(p) => Ok(vec![PatientEvent::PatientAdded(PatientAdded {
                id: Uuid::new_v4(),
                name: p.name.clone(),
                version: i64::default(),
                address: p.address.clone(),
                age: p.age,
                phone: p.phone.clone(),
                email: p.email.clone(),
            })]),
            PatientCommand::UpdatePatient(p) => match state {
                None => return Err(anyhow::anyhow!("Patient not found")),
                Some(state) => {
                    if p.name == state.name
                        && p.age == state.age
                        && p.phone == state.phone
                        && p.email == state.email
                    {
                        return Err(anyhow::anyhow!("Patient not updated"));
                    }
                    Ok(vec![PatientEvent::PatientUpdated(PatientUpdated {
                        name: p.name.clone(),
                        age: p.age,
                        phone: p.phone.clone(),
                        email: p.email.clone(),
                    })])
                }
            },
            PatientCommand::UpdatePatientAddress(a) => match state {
                None => return Err(anyhow::anyhow!("Patient not found")),
                Some(state) => {
                    if a.address == state.address {
                        return Err(anyhow::anyhow!("Patient address not updated"));
                    }
                    Ok(vec![PatientEvent::PatientAddressUpdated(
                        PatientAddressUpdated {
                            address: a.address.clone(),
                        },
                    )])
                }
            },
        }
    }
}

pub const PATIENT_AGGREGATE: PatientAggregate = PatientAggregate {};
