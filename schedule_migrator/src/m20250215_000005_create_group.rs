use sea_orm_migration::{prelude::*, schema::*};

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
                    .col(pk_auto(Group::Id))
                    .col(string(Group::FullName))
                    .col(string(Group::ShortName))
                    .col(integer(Group::DirectionId))
                    .col(integer_null(Group::SpecialityId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Group::Table, Group::DirectionId)
                            .to(Direction::Table, Direction::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Group::Table, Group::SpecialityId)
                            .to(Speciality::Table, Speciality::Id),
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
