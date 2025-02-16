use sea_orm_migration::prelude::*;

use crate::{
    m20250215_000003_create_direction::Direction, m20250215_000004_create_speciality::Speciality,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20250215_000005_create_group"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Group::Table)
                    .col(ColumnDef::new(Group::Id).integer().not_null().primary_key())
                    .col(ColumnDef::new(Group::FullName).string().not_null())
                    .col(ColumnDef::new(Group::ShortName).string().not_null())
                    .col(ColumnDef::new(Group::DirectionId).integer().not_null())
                    .col(ColumnDef::new(Group::SpecialityId).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_group_direction")
                            .from(Group::Table, Group::DirectionId)
                            .to(Direction::Table, Direction::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_group_speciality")
                            .from(Group::Table, Group::SpecialityId)
                            .to(Speciality::Table, Speciality::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Group::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Group {
    Table,
    Id,
    FullName,
    ShortName,
    DirectionId,
    SpecialityId,
}
