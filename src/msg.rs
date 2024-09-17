use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::AccountType;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateUser {
        username: String,
        phone: String,
        latitude: i128,
        longitude: i128,
        account_type: AccountType,
    },
    UpdateUser {
        username: String,
        phone: String,
        latitude: i128,
        longitude: i128,
        account_type: AccountType,
    },
    CreateStore {
        name: String,
        description: String,
        phone: String,
        latitude: i128,
        longitude: i128,
    },
    CreateRequest {
        name: String,
        description: String,
        images: Vec<String>,
        latitude: i128,
        longitude: i128,
    },
    CreateOffer {
        price: i128,
        images: Vec<String>,
        request_id: u64,
        store_name: String,
    },
    AcceptOffer {
        offer_id: u64,
    },
    ToggleLocation {
        enabled: bool,
    },
    DeleteRequest {
        offer_id: u64,
    },
    MarkRequestAsCompleted {
        request_id: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetUser { address: String },

    GetAllUsers {},

    GetRequest { request_id: u64 },

    GetAllRequests {},

    GetOffer { offer_id: u64 },

    GetOffersForRequest { request_id: u64 },

    GetAllOffers {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}
