use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Map;

// Enums
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum AccountType {
    Buyer,
    Seller,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum RequestLifecycle {
    Pending,
    AcceptedBySeller,
    AcceptedByBuyer,
    RequestLocked,
    Completed,
}

// Structs
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Location {
    pub latitude: i128,
    pub longitude: i128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Store {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub phone: String,
    pub location: Location,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub phone: String,
    pub location: Location,
    pub created_at: u64,
    pub updated_at: u64,
    pub account_type: AccountType,
    pub location_enabled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Request {
    pub id: u64,
    pub name: String,
    pub buyer_id: u64,
    pub seller_price_quote: i128,
    pub seller_ids: Vec<u64>,
    pub offer_ids: Vec<u64>,
    pub locked_seller_id: u64,
    pub description: String,
    pub images: Vec<String>,
    pub created_at: u64,
    pub lifecycle: RequestLifecycle,
    pub location: Location,
    pub updated_at: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Offer {
    pub id: u64,
    pub price: i128,
    pub images: Vec<String>,
    pub request_id: u64,
    pub store_name: String,
    pub seller_id: u64,
    pub is_accepted: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

// State
pub const USERS: Map<&[u8], User> = Map::new("users");
pub const REQUESTS: Map<u64, Request> = Map::new("requests");
pub const STORES: Map<u64, Store> = Map::new("stores");
pub const OFFERS: Map<u64, Offer> = Map::new("offers");
