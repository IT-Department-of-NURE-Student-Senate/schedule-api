use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20250215_000009_create_building"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Building::Table)
                    .col(
                        ColumnDef::new(Building::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Building::FullName).string().not_null())
                    .col(ColumnDef::new(Building::ShortName).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Building::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Building {
    Table,
    Id,
    FullName,
    ShortName,
}
