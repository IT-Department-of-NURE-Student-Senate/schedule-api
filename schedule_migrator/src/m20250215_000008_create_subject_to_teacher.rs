use sea_orm_migration::prelude::*;

use crate::{m20250215_000006_create_teacher::Teacher, m20250215_000007_create_subject::Subject};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20250215_000008_create_subject_to_teacher"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SubjectToTeacher::Table)
                    .col(
                        ColumnDef::new(SubjectToTeacher::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(SubjectToTeacher::EventType)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SubjectToTeacher::Hours)
                            .small_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SubjectToTeacher::SubjectId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SubjectToTeacher::TeacherId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_subject_to_teacher_subject")
                            .from(SubjectToTeacher::Table, SubjectToTeacher::SubjectId)
                            .to(Subject::Table, Subject::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_subject_to_teacher_teacher")
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
    Id,
    EventType,
    Hours,
    SubjectId,
    TeacherId,
}
