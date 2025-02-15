use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250215_000003_create_direction::Direction;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20250215_000004_create_speciality"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Speciality::Table)
                    .col(pk_auto(Speciality::Id))
                    .col(string(Speciality::FullName))
                    .col(string(Speciality::ShortName))
                    .col(integer(Speciality::DirectionId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Speciality::Table, Speciality::DirectionId)
                            .to(Direction::Table, Direction::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Speciality::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Speciality {
    Table,
    Id,
    FullName,
    ShortName,
    DirectionId,
}
