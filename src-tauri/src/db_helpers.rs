use crate::types::patient::Patient;
use anyhow::Result;
use sqlx::migrate::MigrateDatabase;
use sqlx::{Error, Pool, Sqlite};

pub async fn recreate_database(conn: &str) -> anyhow::Result<()> {
    if Sqlite::database_exists(conn).await? {
        Sqlite::drop_database(conn).await?;
        match Sqlite::create_database(conn).await {
            Ok(_) => println!("Create db success for {}", conn),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        match Sqlite::create_database(conn).await {
            Ok(_) => println!("Create db success for {}", conn),
            Err(error) => panic!("error: {}", error),
        }
    }
    Ok(())
}

pub async fn upsert_patient(
    read_pool: Pool<Sqlite>,
    p: Patient,
    version: i64,
    stream_id: String,
) -> std::result::Result<(), Error> {
    //Insert or Update into Patient and Address table with transaction for read model
    println!("upsert_version: {:#?}", version);
    println!("upsert_patient: {:#?}", p);
    let mut tx = read_pool.begin().await?;
    let patient = sqlx::query("INSERT INTO Patient (id, stream_id, version,name, age, phone, email) VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT(id) DO UPDATE SET version = $3, name = $4, age = $5, phone = $6, email = $7")
        .bind(p.id)
        .bind(stream_id)
        .bind(version)
        .bind(p.name)
        .bind(p.age)
        .bind(p.phone)
        .bind(p.email)
        .execute(&mut *tx)
        .await?;

    let address = sqlx::query("INSERT INTO Address (patient_id, street, city, state, zip) VALUES ($1, $2, $3, $4, $5) ON CONFLICT(patient_id) DO UPDATE SET street = $2, city = $3, state = $4, zip = $5")
        .bind(p.id)
        .bind(p.address.street)
        .bind(p.address.city)
        .bind(p.address.state)
        .bind(p.address.zip)
        .execute(&mut *tx)
        .await?;

    println!("patient: {:#?}", patient);
    println!("address: {:#?}", address);

    tx.commit().await
}

pub async fn setup_read_db(read_pool: Pool<Sqlite>) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS Patient (
                id TEXT PRIMARY KEY,
                stream_id TEXT NOT NULL,
                version INTEGER NOT NULL,
                name TEXT NOT NULL,
                age INTEGER NOT NULL,
                phone TEXT NOT NULL,
                email TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS Address (
                patient_id TEXT PRIMARY KEY,
                street TEXT NOT NULL,
                city TEXT NOT NULL,
                state TEXT NOT NULL,
                zip TEXT NOT NULL,
                FOREIGN KEY (patient_id) REFERENCES Patient(id)
            );
        "#,
    )
    .execute(&read_pool)
    .await?;
    Ok(())
}
