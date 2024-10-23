use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
    Paid,
    Completed,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum CoinPayment {
    Cosmos,
    USDT,
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
    pub authority: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Request {
    pub id: u64,
    pub name: String,
    pub buyer_id: u64,
    pub seller_price_quote: u128,
    pub seller_ids: Vec<u64>,
    pub offer_ids: Vec<u64>,
    pub locked_seller_id: u64,
    pub description: String,
    pub images: Vec<String>,
    pub created_at: u64,
    pub lifecycle: RequestLifecycle,
    pub location: Location,
    pub updated_at: u64,
    pub paid: bool,
    pub accepted_offer_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Offer {
    pub id: u64,
    pub price: u128,
    pub images: Vec<String>,
    pub request_id: u64,
    pub store_name: String,
    pub seller_id: u64,
    pub is_accepted: bool,
    pub created_at: u64,
    pub updated_at: u64,
    pub authority: Addr,
}

// let mut new_payment_info = PaymentInfo {
//     buyer: info.sender.clone(),
//     request_id,
//     payer: info.sender.clone(),
//     authority: offer.authority.clone(),
//     amount: Uint128::zero(),
//     coin: coin.clone(),
//     created_at: env.block.time.seconds(),
//     updated_at: env.block.time.seconds(),
// };
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PaymentInfo {
    pub buyer: Addr,
    pub request_id: u64,
    pub payer: Addr,
    pub authority: Addr,
    pub amount: Uint128,
    pub coin: CoinPayment,
    pub created_at: u64,
    pub updated_at: u64,
}

// State
pub const USERS: Map<&[u8], User> = Map::new("users");
pub const USERS_BY_ID: Map<u64, User> = Map::new("users_by_id");
pub const REQUESTS: Map<u64, Request> = Map::new("requests");
pub const STORES: Map<u64, Store> = Map::new("stores");
pub const OFFERS: Map<u64, Offer> = Map::new("offers");
pub const USER_STORE_IDS: Map<&[u8], Vec<u64>> = Map::new("user_store_ids");

pub const REQUEST_COUNT: Item<u64> = Item::new("request_count");
pub const OFFER_COUNT: Item<u64> = Item::new("offer_count");
pub const USER_COUNT: Item<u64> = Item::new("user_count");
pub const STORE_COUNT: Item<u64> = Item::new("store_count");
pub const PAYMENT_INFO: Map<u64, PaymentInfo> = Map::new("payment_info");
pub const TIME_TO_LOCK: u64 = 900; // 15 minutes
pub const USDT_ADDR: &str = "terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v";
pub const COIN_DENOM: &str = "uosmo";
