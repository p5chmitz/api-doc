use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
//use chrono::NaiveDate;

// Name Entity
pub mod name {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "name")]
    pub struct Model {
        #[sea_orm(primary_key)]
        #[serde(skip_deserializing)]
        pub id: i32,
        pub first: String,
        pub middle: String,
        pub surname: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

// Address Entity
pub mod address {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "address")]
    pub struct Model {
        #[sea_orm(primary_key)]
        #[serde(skip_deserializing)]
        pub id: i32,
        pub address_lines: Vec<String>, 
        pub sublocality: String,
        pub locality: String,
        pub administrative_area: String,
        pub postal_code: String,
        pub country_region: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

// Birth date Entity
pub mod birthdate {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "birthdate")]
    pub struct Model {
        #[sea_orm(primary_key)]
        #[serde(skip_deserializing)]
        pub id: i32,
        pub day: i32,
        pub month: i32,
        pub year: i32,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}

}

// Patient Entity
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "patient")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,

    #[sea_orm(belongs_to = "name::Model", from = "Column::NameId", to = "name::Column::Id")]
    pub name_id: i32,

    #[sea_orm(belongs_to = "address::Model", from = "Column::AddressId", to = "address::Column::Id")]
    pub address_id: i32,

    #[sea_orm(belongs_to = "birthdate::Model", from = "Column::BirthdateId", to = "birthdate::Column::Id")]
    pub birthdate_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "name::Entity", from = "Column::NameId", to = "name::Column::Id")]
    Name,

    #[sea_orm(belongs_to = "address::Entity", from = "Column::AddressId", to = "address::Column::Id")]
    Address,
}

impl ActiveModelBehavior for ActiveModel {}

// Used for POST requests
//#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
//pub struct CreatePatientRequest {
//    pub name: NameRequest,
//    pub address: AddressRequest,
//    pub birth_date: NaiveDate,
//}
//#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
//pub struct NameRequest {
//    pub first: String,
//    pub middle: String,
//    pub surname: String,
//}
//#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
//pub struct AddressRequest {
//    pub address_lines: String, 
//    pub sublocality: String,
//    pub locality: String,
//    pub administrative_area: String,
//    pub postal_code: String,
//    pub country_region: String,
//}
