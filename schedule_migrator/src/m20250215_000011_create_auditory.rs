use sea_orm_migration::prelude::*;

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
                    .col(
                        ColumnDef::new(Auditory::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Auditory::Name).string().not_null())
                    .col(ColumnDef::new(Auditory::BuildingId).integer().not_null())
                    .col(ColumnDef::new(Auditory::Floor).integer().not_null())
                    .col(ColumnDef::new(Auditory::HavePower).boolean().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_auditory_building")
                            .from(Auditory::Table, Auditory::BuildingId)
                            .to(Building::Table, Building::Id)
                            .on_delete(ForeignKeyAction::Cascade),
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
