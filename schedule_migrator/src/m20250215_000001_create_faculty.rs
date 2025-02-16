use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20250215_000001_create_faculty"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Faculty::Table)
                    .col(
                        ColumnDef::new(Faculty::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Faculty::FullName).string().not_null())
                    .col(ColumnDef::new(Faculty::ShortName).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Faculty::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Faculty {
    Table,
    Id,
    FullName,
    ShortName,
}
