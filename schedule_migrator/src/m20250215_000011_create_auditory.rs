use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250215_000009_create_building::Building;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20250215_000011_create_auditory"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Auditory::Table)
                    .col(pk_auto(Auditory::Id))
                    .col(string(Auditory::Name))
                    .col(integer(Auditory::BuildingId))
                    .col(integer(Auditory::Floor))
                    .col(boolean(Auditory::HavePower))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Auditory::Table, Auditory::BuildingId)
                            .to(Building::Table, Building::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Auditory::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Auditory {
    Table,
    Id,
    Name,
    BuildingId,
    Floor,
    HavePower,
}
