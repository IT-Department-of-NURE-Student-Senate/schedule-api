use sea_orm_migration::{prelude::*, schema::*};

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
                    .col(integer(AuditoryTypeToAuditory::AuditoryId))
                    .col(integer(AuditoryTypeToAuditory::AuditoryTypeId))
                    .primary_key(
                        Index::create()
                            .col(AuditoryTypeToAuditory::AuditoryId)
                            .col(AuditoryTypeToAuditory::AuditoryTypeId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                AuditoryTypeToAuditory::Table,
                                AuditoryTypeToAuditory::AuditoryId,
                            )
                            .to(Auditory::Table, Auditory::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                AuditoryTypeToAuditory::Table,
                                AuditoryTypeToAuditory::AuditoryTypeId,
                            )
                            .to(AuditoryType::Table, AuditoryType::Id),
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
    AuditoryId,
    AuditoryTypeId,
}
