use sea_orm_migration::prelude::*;

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
                    .col(
                        ColumnDef::new(EventToGroup::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(EventToGroup::EventId).integer().not_null())
                    .col(ColumnDef::new(EventToGroup::GroupId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_event_to_group_event")
                            .from(EventToGroup::Table, EventToGroup::EventId)
                            .to(Event::Table, Event::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_event_to_group_group")
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
    Id,
    EventId,
    GroupId,
}
