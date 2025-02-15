use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20250215_000006_create_teacher::Teacher, m20250215_000007_create_subject::Subject};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20250215_000005_create_auditory_type_to_auditory"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SubjectToTeacher::Table)
                    .col(string(SubjectToTeacher::EventType))
                    .col(small_integer(SubjectToTeacher::Hours))
                    .col(integer(SubjectToTeacher::SubjectId))
                    .col(integer(SubjectToTeacher::TeacherId))
                    .primary_key(
                        Index::create()
                            .col(SubjectToTeacher::SubjectId)
                            .col(SubjectToTeacher::TeacherId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(SubjectToTeacher::Table, SubjectToTeacher::SubjectId)
                            .to(Subject::Table, Subject::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(SubjectToTeacher::Table, SubjectToTeacher::TeacherId)
                            .to(Teacher::Table, Teacher::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SubjectToTeacher::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum SubjectToTeacher {
    Table,
    EventType,
    Hours,
    SubjectId,
    TeacherId,
}
