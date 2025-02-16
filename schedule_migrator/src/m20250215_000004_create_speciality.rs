use sea_orm_migration::prelude::*;

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
                    .col(
                        ColumnDef::new(Speciality::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Speciality::FullName).string().not_null())
                    .col(ColumnDef::new(Speciality::ShortName).string().not_null())
                    .col(ColumnDef::new(Speciality::DirectionId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_speciality_direction")
                            .from(Speciality::Table, Speciality::DirectionId)
                            .to(Direction::Table, Direction::Id)
                            .on_delete(ForeignKeyAction::Cascade),
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
