use sea_orm_migration::{prelude::*, schema::*};

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
                    .col(pk_auto(Direction::Id))
                    .col(string(Direction::FullName))
                    .col(string(Direction::ShortName))
                    .col(integer(Direction::FacultyId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Direction::Table, Direction::FacultyId)
                            .to(Faculty::Table, Faculty::Id),
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
