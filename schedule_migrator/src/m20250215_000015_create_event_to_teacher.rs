use sea_orm_migration::prelude::*;

use crate::{m20250215_000006_create_teacher::Teacher, m20250215_000013_create_event::Event};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20250215_000015_create_event_to_teacher"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(EventToTeacher::Table)
                    .col(
                        ColumnDef::new(EventToTeacher::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(EventToTeacher::EventId).integer().not_null())
                    .col(
                        ColumnDef::new(EventToTeacher::TeacherId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_event_to_teacher_event")
                            .from(EventToTeacher::Table, EventToTeacher::EventId)
                            .to(Event::Table, Event::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_event_to_teacher_teacher")
                            .from(EventToTeacher::Table, EventToTeacher::TeacherId)
                            .to(Teacher::Table, Teacher::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(EventToTeacher::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum EventToTeacher {
    Table,
    Id,
    EventId,
    TeacherId,
}
