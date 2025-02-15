use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250215_000002_create_department::Department;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20250215_000006_create_teacher"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Teacher::Table)
                    .col(pk_auto(Teacher::Id))
                    .col(string(Teacher::FullName))
                    .col(string(Teacher::ShortName))
                    .col(integer(Teacher::DepartmentId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Teacher::Table, Teacher::DepartmentId)
                            .to(Department::Table, Department::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Teacher::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Teacher {
    Table,
    Id,
    FullName,
    ShortName,
    DepartmentId,
}
