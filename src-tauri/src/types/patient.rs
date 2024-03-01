use crate::types::address::Address;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Patient {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) address: Address,
    pub(crate) age: i32,
    pub(crate) phone: String,
    pub(crate) email: String,
}
