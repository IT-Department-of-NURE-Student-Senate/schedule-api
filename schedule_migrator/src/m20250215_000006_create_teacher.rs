use sea_orm_migration::prelude::*;

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
                    .col(
                        ColumnDef::new(Teacher::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Teacher::FullName).string().not_null())
                    .col(ColumnDef::new(Teacher::ShortName).string().not_null())
                    .col(ColumnDef::new(Teacher::DepartmentId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_teacher_department")
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
