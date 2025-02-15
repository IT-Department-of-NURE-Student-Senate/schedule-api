use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20250215_000010_create_auditory_type"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AuditoryType::Table)
                    .col(pk_auto(AuditoryType::Id))
                    .col(string(AuditoryType::Name))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AuditoryType::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum AuditoryType {
    Table,
    Id,
    Name,
}
