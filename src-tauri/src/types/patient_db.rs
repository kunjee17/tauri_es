use uuid::Uuid;

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct AddressDB {
    pub(crate) patient_id: Uuid,
    pub(crate) street: String,
    pub(crate) city: String,
    pub(crate) state: String,
    pub(crate) zip: String,
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct PatientDB {
    pub(crate) id: Uuid,
    pub(crate) stream_id: String,
    pub(crate) version: i64,
    pub(crate) name: String,
    pub(crate) age: i32,
    pub(crate) phone: String,
    pub(crate) email: String,
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct PatientMeta {
    pub(crate) id: Uuid,
    pub(crate) stream_id: String,
    pub(crate) version: i64,
}
