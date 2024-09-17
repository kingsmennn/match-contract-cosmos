use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128,
};
use cw2::set_contract_version;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Contract Info
const CONTRACT_NAME: &str = "crates.io:marketplace";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

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
const USERS: Map<&[u8], User> = Map::new("users");
const REQUESTS: Map<u64, Request> = Map::new("requests");
const OFFERS: Map<u64, Offer> = Map::new("offers");

// Instantiate
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

// Execute Msg
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
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
    RemoveOffer {
        offer_id: u64,
    },
}

// Instantiate Message
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

// Execute
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::CreateUser {
            username,
            phone,
            latitude,
            longitude,
            account_type,
        } => create_user(
            deps,
            info,
            username,
            phone,
            latitude,
            longitude,
            account_type,
        ),
        ExecuteMsg::UpdateUser {
            username,
            phone,
            latitude,
            longitude,
            account_type,
        } => update_user(
            deps,
            info,
            username,
            phone,
            latitude,
            longitude,
            account_type,
        ),
        ExecuteMsg::CreateStore {
            name,
            description,
            phone,
            latitude,
            longitude,
        } => create_store(deps, info, name, description, phone, latitude, longitude),
        ExecuteMsg::CreateRequest {
            name,
            description,
            images,
            latitude,
            longitude,
        } => create_request(deps, info, name, description, images, latitude, longitude),
        ExecuteMsg::CreateOffer {
            price,
            images,
            request_id,
            store_name,
        } => create_offer(deps, info, price, images, request_id, store_name),
        ExecuteMsg::AcceptOffer { offer_id } => accept_offer(deps, info, offer_id),
        ExecuteMsg::ToggleLocation { enabled } => toggle_location(deps, info, enabled),
        ExecuteMsg::RemoveOffer { offer_id } => remove_offer(deps, info, offer_id),
    }
}
