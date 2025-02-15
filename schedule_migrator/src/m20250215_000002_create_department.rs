use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250215_000001_create_faculty::Faculty;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20250215_000002_create_department"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Department::Table)
                    .col(pk_auto(Department::Id))
                    .col(string(Department::FullName))
                    .col(string(Department::ShortName))
                    .col(integer(Department::FacultyId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Department::Table, Department::FacultyId)
                            .to(Faculty::Table, Faculty::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Department::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Department {
    Table,
    Id,
    FullName,
    ShortName,
    FacultyId,
}
