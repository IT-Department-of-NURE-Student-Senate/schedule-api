use sea_orm_migration::prelude::*;

use crate::{
    m20250215_000010_create_auditory_type::AuditoryType, m20250215_000011_create_auditory::Auditory,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20250215_000012_create_auditory_type_to_auditory"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AuditoryTypeToAuditory::Table)
                    .col(
                        ColumnDef::new(AuditoryTypeToAuditory::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(AuditoryTypeToAuditory::AuditoryId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AuditoryTypeToAuditory::AuditoryTypeId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_auditory_type_to_auditory_auditory")
                            .from(
                                AuditoryTypeToAuditory::Table,
                                AuditoryTypeToAuditory::AuditoryId,
                            )
                            .to(Auditory::Table, Auditory::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_auditory_type_to_auditory_auditory_type")
                            .from(
                                AuditoryTypeToAuditory::Table,
                                AuditoryTypeToAuditory::AuditoryTypeId,
                            )
                            .to(AuditoryType::Table, AuditoryType::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(AuditoryTypeToAuditory::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
pub enum AuditoryTypeToAuditory {
    Table,
    Id,
    AuditoryId,
    AuditoryTypeId,
}
