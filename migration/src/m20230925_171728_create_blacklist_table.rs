use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Blacklist::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Blacklist::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Blacklist::Token).string().not_null())
                    .col(ColumnDef::new(Blacklist::BlacklistedOn).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Blacklist::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Blacklist {
    Table,
    Id,
    Token,
    BlacklistedOn,
}
