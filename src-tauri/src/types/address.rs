use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Address {
    pub(crate) street: String,
    pub(crate) city: String,
    pub(crate) state: String,
    pub(crate) zip: String,
}
