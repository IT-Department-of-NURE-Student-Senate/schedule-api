use sea_orm_migration::prelude::*;

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
                    .col(
                        ColumnDef::new(Department::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Department::FullName).string().not_null())
                    .col(ColumnDef::new(Department::ShortName).string().not_null())
                    .col(ColumnDef::new(Department::FacultyId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_department_faculty")
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
