use sea_orm_migration::prelude::*;

use crate::m20250215_000001_create_faculty::Faculty;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20250215_000003_create_direction"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Direction::Table)
                    .col(
                        ColumnDef::new(Direction::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Direction::FullName).string().not_null())
                    .col(ColumnDef::new(Direction::ShortName).string().not_null())
                    .col(ColumnDef::new(Direction::FacultyId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_direction_faculty")
                            .from(Direction::Table, Direction::FacultyId)
                            .to(Faculty::Table, Faculty::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Direction::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Direction {
    Table,
    Id,
    FullName,
    ShortName,
    FacultyId,
}
