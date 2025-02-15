use sea_orm_migration::prelude::*;

mod m20250215_000001_create_faculty;
mod m20250215_000002_create_department;
mod m20250215_000003_create_direction;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250215_000001_create_faculty::Migration),
            Box::new(m20250215_000002_create_department::Migration),
            Box::new(m20250215_000003_create_direction::Migration),
        ]
    }
}
