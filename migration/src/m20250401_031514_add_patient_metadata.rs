use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Alter the Patient table to add identifiers
        manager
            .alter_table(
                Table::alter()
                    .table(Patient::Table)
                    .add_column(
                        ColumnDef::new(Patient::PatientId)
                        .uuid()
                        //.not_null()
                    )
                    .add_column(
                        ColumnDef::new(Patient::CreatedAt)
                            // Ensures UTC storage
                            .timestamp_with_time_zone() 
                            //.not_null()
                            // Default to now (UTC)
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    .to_owned(),)
            .await

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Patient::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Patient {
    Table,
    PatientId,
    CreatedAt,
}
