use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Username).string().not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Name::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Name::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Name::First)
                        .string().not_null())
                    .col(ColumnDef::new(Name::Middle)
                        .string())
                    .col(ColumnDef::new(Name::Surname)
                        .string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Address::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Address::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    // NOTE: Only Postgres supports string arrays
                    .col(
                        ColumnDef::new(Address::AddressLines)
                            .array(ColumnType::String(StringLen::N(60)))
                            .not_null(),
                    )
                    .col(ColumnDef::new(Address::Sublocality)
                        .string())
                    .col(ColumnDef::new(Address::Locality)
                        .string())
                    .col(ColumnDef::new(Address::AdministrativeArea)
                        .string())
                    .col(ColumnDef::new(Address::PostalCode)
                        .string())
                    .col(ColumnDef::new(Address::CountryRegion)
                        .string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Birthdate::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Birthdate::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Birthdate::Day)
                        .integer().not_null())
                    .col(ColumnDef::new(Birthdate::Month)
                        .integer().not_null())
                    .col(ColumnDef::new(Birthdate::Year)
                        .integer().not_null())
                    .to_owned(),
            )
            .await?;

        manager
        .create_table(
            Table::create()
                .table(Patient::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Patient::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(
                    ColumnDef::new(Patient::NameId)
                        .integer()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(Patient::AddressId)
                        .integer()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(Patient::BirthdateId)
                        .integer()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(Patient::PatientId)
                        .uuid()
                        .not_null()
                )
                .col(
                    ColumnDef::new(Patient::CreatedAt)
                        // Ensures UTC storage
                        .timestamp_with_time_zone() 
                        .not_null()
                        .default(Expr::cust("CURRENT_TIMESTAMP")),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_patient_name")
                        .from(Patient::Table, Patient::NameId)
                        .to(Name::Table, Name::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_patient_address")
                        .from(Patient::Table, Patient::AddressId)
                        .to(Address::Table, Address::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_patient_birthdate")
                        .from(Patient::Table, Patient::BirthdateId)
                        .to(Birthdate::Table, Birthdate::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await


  }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop()
            .table(Patient::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop()
            .table(Birthdate::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop()
            .table(Address::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop()
            .table(Name::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop()
            .table(User::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum User {
    Table,
    Id,
    Username,
    Password,
}

#[derive(Iden)]
enum Name {
    Table,
    Id,
    First,
    Middle,
    Surname,
}

#[derive(Iden)]
enum Address {
    Table,
    Id,
    AddressLines,
    Sublocality,
    Locality,
    AdministrativeArea,
    PostalCode,
    CountryRegion,
}

#[derive(Iden)]
enum Birthdate {
    Table,
    Id,
    Day,
    Month,
    Year,
}

#[derive(Iden)]
enum Patient {
    Table,
    Id,
    PatientId,
    CreatedAt,
    NameId,
    AddressId,
    BirthdateId,
}
