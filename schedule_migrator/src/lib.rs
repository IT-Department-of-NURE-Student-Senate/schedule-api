use sea_orm_migration::prelude::*;

mod m20250215_000001_create_faculty;
mod m20250215_000002_create_department;
mod m20250215_000003_create_direction;
mod m20250215_000004_create_speciality;
mod m20250215_000005_create_group;
mod m20250215_000006_create_teacher;
mod m20250215_000007_create_subject;
mod m20250215_000008_create_subject_to_teacher;
mod m20250215_000009_create_building;
mod m20250215_000010_create_auditory_type;
mod m20250215_000011_create_auditory;
mod m20250215_000012_create_auditory_type_to_auditory;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250215_000001_create_faculty::Migration),
            Box::new(m20250215_000002_create_department::Migration),
            Box::new(m20250215_000003_create_direction::Migration),
            Box::new(m20250215_000004_create_speciality::Migration),
            Box::new(m20250215_000005_create_group::Migration),
            Box::new(m20250215_000006_create_teacher::Migration),
            Box::new(m20250215_000007_create_subject::Migration),
            Box::new(m20250215_000008_create_subject_to_teacher::Migration),
            Box::new(m20250215_000009_create_building::Migration),
            Box::new(m20250215_000010_create_auditory_type::Migration),
            Box::new(m20250215_000011_create_auditory::Migration),
            Box::new(m20250215_000012_create_auditory_type_to_auditory::Migration),
        ]
    }
}
