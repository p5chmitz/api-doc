pub use sea_orm_migration::prelude::*;

//mod m20220101_000001_create_table;
//mod m20250324_012019_add_test_column_to_table;
//mod m20250325_000311_fix_user_table;
//mod m20250327_025117_add_patient_schema;
//mod m20250401_031514_add_patient_metadata;
mod m20250407_035528_create_base_schema;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            //Box::new(m20220101_000001_create_table::Migration),
            //Box::new(m20250324_012019_add_test_column_to_table::Migration),
            //Box::new(m20250325_000311_fix_user_table::Migration),
            //Box::new(m20250327_025117_add_patient_schema::Migration),
            //Box::new(m20250401_031514_add_patient_metadata::Migration),
            Box::new(m20250407_035528_create_base_schema::Migration),
        ]
    }
}
