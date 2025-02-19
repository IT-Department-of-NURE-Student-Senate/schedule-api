use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement};

const DATABASE_URL: &str = "mysql://schedule:schedule_pass@localhost";
const DB_NAME: &str = "schedule";

pub async fn get_connection() -> Result<DatabaseConnection, Error> {
    let db = Database::connect(DATABASE_URL).await?;

    let db_backend = db.get_database_backend();

    let db = match db_backend {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db_backend,
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME),
            ))
            .await?;

            let url = format!("{}/{}", DATABASE_URL, DB_NAME);
            Database::connect(&url).await?
        }
        _ => return Err(Error::UnsupportedDbBackend(db_backend)),
    };

    Ok(db)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unsupported database backend: {0:?}")]
    UnsupportedDbBackend(DbBackend),

    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),
}
