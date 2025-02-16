use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20250215_000007_create_subject"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Subject::Table)
                    .col(
                        ColumnDef::new(Subject::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Subject::FullName).string().not_null())
                    .col(ColumnDef::new(Subject::ShortName).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Subject::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Subject {
    Table,
    Id,
    FullName,
    ShortName,
}
