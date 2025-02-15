use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20250215_000005_create_group::Group, m20250215_000013_create_event::Event};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20250215_000014_create_event_to_group"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(EventToGroup::Table)
                    .col(integer(EventToGroup::EventId))
                    .col(integer(EventToGroup::GroupId))
                    .primary_key(
                        Index::create()
                            .col(EventToGroup::EventId)
                            .col(EventToGroup::GroupId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(EventToGroup::Table, EventToGroup::EventId)
                            .to(Event::Table, Event::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(EventToGroup::Table, EventToGroup::GroupId)
                            .to(Group::Table, Group::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(EventToGroup::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum EventToGroup {
    Table,
    EventId,
    GroupId,
}
