use std::collections::HashSet;

use schedule_migrator::Migrator;
use sea_orm::{
    ConnectionTrait, Database, DatabaseConnection, DbBackend, EntityTrait, Set, Statement,
    TransactionTrait, sea_query::OnConflict,
};
use sea_orm_migration::MigratorTrait;

use crate::{entities, models};

pub struct Repository {
    db: DatabaseConnection,
}

impl Repository {
    pub async fn new() -> Result<Self, Error> {
        let db_url = match std::env::var("DATABASE_URL") {
            Ok(db_url) => db_url,
            Err(_) => return Err(Error::DatabaseUrlNotFound),
        };

        let db_name = match std::env::var("DATABASE_NAME") {
            Ok(db_name) => db_name,
            Err(_) => return Err(Error::DatabaseNameNotFound),
        };

        let db = Database::connect(&db_url).await?;

        let db = match db.get_database_backend() {
            db_backend @ DbBackend::MySql => {
                db.execute(Statement::from_string(
                    db_backend,
                    format!("CREATE DATABASE IF NOT EXISTS `{}`;", db_name),
                ))
                .await?;

                let url = format!("{}/{}", db_url, db_name);
                Database::connect(&url).await?
            }
            db_backend => return Err(Error::UnsupportedDbBackend(db_backend)),
        };

        Ok(Self { db })
    }

    pub async fn run_migrations(&self) -> Result<(), Error> {
        // TODO: Add interactive prompt to confirm running migrations
        Ok(Migrator::fresh(&self.db).await?)
    }

    pub async fn update_from_podr(&self, data: models::PodrRoot) -> Result<(), Error> {
        let faculties = data.university.faculties;

        let mut faculty_ids = HashSet::new();
        let mut faculties_to_insert = Vec::new();

        let mut department_ids = HashSet::new();
        let mut departments_to_insert = Vec::new();

        let mut teacher_ids = HashSet::new();
        let mut teachers_to_insert = Vec::new();

        for faculty in faculties {
            if faculty_ids.insert(faculty.id) {
                faculties_to_insert.push(entities::faculty::ActiveModel {
                    id: Set(faculty.id),
                    full_name: Set(faculty.full_name),
                    short_name: Set(faculty.short_name),
                });
            }

            for department in faculty.departments {
                if department_ids.insert(department.id) {
                    departments_to_insert.push(entities::department::ActiveModel {
                        id: Set(department.id),
                        full_name: Set(department.full_name),
                        short_name: Set(department.short_name),
                        faculty_id: Set(faculty.id),
                    });
                }

                for teacher in department.teachers {
                    if teacher_ids.insert(teacher.id) {
                        teachers_to_insert.push(entities::teacher::ActiveModel {
                            id: Set(teacher.id),
                            full_name: Set(teacher.full_name),
                            short_name: Set(teacher.short_name),
                            department_id: Set(department.id),
                        });
                    }
                }
            }
        }

        let txn = self.db.begin().await?;

        if !faculties_to_insert.is_empty() {
            entities::faculty::Entity::insert_many(faculties_to_insert)
                .on_conflict(
                    OnConflict::column(entities::faculty::Column::Id)
                        .update_column(entities::faculty::Column::Id)
                        .to_owned(),
                )
                .exec(&txn)
                .await?;
        }

        if !departments_to_insert.is_empty() {
            entities::department::Entity::insert_many(departments_to_insert)
                .on_conflict(
                    OnConflict::column(entities::department::Column::Id)
                        .update_column(entities::department::Column::Id)
                        .to_owned(),
                )
                .exec(&txn)
                .await?;
        }

        if !teachers_to_insert.is_empty() {
            entities::teacher::Entity::insert_many(teachers_to_insert)
                .on_conflict(
                    OnConflict::column(entities::teacher::Column::Id)
                        .update_column(entities::teacher::Column::Id)
                        .to_owned(),
                )
                .exec(&txn)
                .await?;
        }

        txn.commit().await?;

        Ok(())
    }

}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unsupported database backend: {0:?}")]
    UnsupportedDbBackend(DbBackend),

    #[error("Error retrieving DATABASE_URL environment variable")]
    DatabaseUrlNotFound,

    #[error("Error retrieving DATABASE_NAME environment variable")]
    DatabaseNameNotFound,

    #[error(transparent)]
    SeaOrmDbErr(#[from] sea_orm::DbErr),
}
