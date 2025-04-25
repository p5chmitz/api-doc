use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
/// The full legal name of the patient
pub struct Name {
    #[schema(read_only)]
    pub first: Option<String>,

    /// The middle name of the patient
    #[schema(example = "Rutherford")]
    pub middle: Option<String>,

    //#[serde(skip)]
    #[schema(read_only)]
    pub surname: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct Address {
    /// Address lines consist of the street number, street name, unit number, or suite number of an address
    ///
    /// Unstructured address lines describing the lower levels of an address. Because values in address_lines do not have type information and may sometimes contain multiple values in a single field (e.g. "Austin, TX"), it is important that the line order is clear. The order of address lines should be "envelope order" for the country/region of the address. In places where this can vary (e.g. Japan), address_language is used to make it explicit (e.g. "ja" for large-to-small ordering and "ja-Latn" or "en" for small-to-large). This way, the most specific line of an address can be selected based on the language. The minimum permitted structural representation of an address consists of a region_code with all remaining information placed in the address_lines. It would be possible to format such an address very approximately without geocoding, but no semantic reasoning could be made about any of the address components until it was at least partially resolved. Creating an address only containing a region_code and address_lines, and then geocoding is the recommended way to handle completely unstructured addresses (as opposed to guessing which parts of the address should be localities or administrative areas).
    #[schema(example = json!(["123 Fake St.", "suite 1300"]))]
    pub address_lines: Option<Vec<String>>,

    /// Sublocality of the address.
    ///
    /// For example, this can be neighborhoods, boroughs, districts.
    #[schema(example = "Brooklyn")]
    pub sublocality: Option<String>,

    /// Generally refers to the city/town portion of the address.
    ///
    /// Examples: US city, IT comune, UK post town. In regions of the world where localities are not well defined or do not fit into this structure well, leave locality empty and use address_lines.
    /// Generally refers to the city/town portion of the address. Examples: US city, IT comune, UK post town. In regions of the world where localities are not well defined or do not fit into this structure well, leave locality empty and use address_lines.
    #[schema(example = "Portland")]
    pub locality: Option<String>,

    /// Highest administrative subdivision which is used for postal addresses of a country or region.
    ///
    /// For example, this can be a state, a province, an oblast, or a prefecture. Specifically, for Spain this is the province and not the autonomous community (e.g. "Barcelona" and not "Catalonia"). Many countries don't use an administrative area in postal addresses. E.g. in Switzerland this should be left unpopulated.
    #[schema(example = "OR")]
    pub administrative_area: Option<String>,

    /// Postal code of the address. Not all countries use or require postal codes to be present, but where they are used, they may trigger additional validation with other parts of the address (e.g. state/zip validation in the U.S.A.).
    #[schema(example = "97211")]
    pub postal_code: Option<String>,

    /// Highest administrative subdivision which is used for postal addresses of a country or region
    #[schema(example = "US")]
    pub country_region: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct BirthDate {
    pub day: Option<i32>,
    pub month: Option<i32>,
    pub year: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
/// Patient information
pub struct UpdatePatientRequest {
    pub name: Option<Name>,
    pub address: Option<Address>,
    pub birthdate: Option<BirthDate>,
}
// Dummy struct for OAS generation
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
/// Patient information
pub struct UpdatePatientRequestOas {
    pub name: Option<Name>,
    pub address: Option<Address>,
}

