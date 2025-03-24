use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Alter the User table to add the Test column
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_column(User::Testerino) // Drops the iden
                    .add_column(ColumnDef::new(User::Test).string().not_null())
                    .to_owned(),
            )
            .await

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum User {
    Table,
    Testerino,
    Test,
}
