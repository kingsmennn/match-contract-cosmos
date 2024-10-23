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
        price: u128,
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
        request_id: u64,
    },
    MarkRequestAsCompleted {
        request_id: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetUser { address: String },

    GetRequest { request_id: u64 },

    GetAllRequests {},

    GetUserRequests { address: String },

    GetOffer { offer_id: u64 },

    GetOffersByRequest { request_id: u64 },

    GetUserById { user_id: u64 },

    GetLocationPreference { address: String },
    GetUserStores { address: String },

    GetSellerOffers { address: String },
}
