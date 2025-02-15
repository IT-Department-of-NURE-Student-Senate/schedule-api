use sea_orm_migration::{prelude::*, schema::*};

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
                    .col(pk_auto(Building::Id))
                    .col(string(Building::FullName))
                    .col(string(Building::ShortName))
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
