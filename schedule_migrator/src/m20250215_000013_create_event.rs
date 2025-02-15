use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20250215_000007_create_subject::Subject, m20250215_000011_create_auditory::Auditory};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20250215_000013_create_event"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Event::Table)
                    .col(pk_auto(Event::Id))
                    .col(timestamp(Event::StartTime))
                    .col(timestamp(Event::EndTime))
                    .col(tiny_integer(Event::NumberPair))
                    .col(string(Event::Type))
                    .col(integer(Event::AuditoryId))
                    .col(integer(Event::SubjectId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Event::Table, Event::SubjectId)
                            .to(Subject::Table, Subject::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Event::Table, Event::AuditoryId)
                            .to(Auditory::Table, Auditory::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Event::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Event {
    Table,
    Id,
    StartTime,
    EndTime,
    NumberPair,
    Type,
    AuditoryId,
    SubjectId,
}
