use crate::error::MarketplaceError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{
    AccountType, CoinPayment, Location, Offer, PaymentInfo, Request, RequestLifecycle, Store, User,
    COIN_DENOM, OFFERS, OFFER_COUNT, PAYMENT_INFO, REQUESTS, REQUEST_COUNT, STORES, STORE_COUNT,
    TIME_TO_LOCK, USDT_ADDR, USERS, USERS_BY_ID, USER_COUNT, USER_STORE_IDS,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128, WasmMsg,
};
use cw2::set_contract_version;
use cw20::Cw20ExecuteMsg;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:marketplace";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, MarketplaceError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    USER_COUNT.save(deps.storage, &1)?;
    STORE_COUNT.save(deps.storage, &1)?;
    REQUEST_COUNT.save(deps.storage, &1)?;
    OFFER_COUNT.save(deps.storage, &1)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, MarketplaceError> {
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
            _env,
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
            _env,
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
        } => create_store(
            deps,
            info,
            _env,
            name,
            description,
            phone,
            latitude,
            longitude,
        ),
        ExecuteMsg::CreateRequest {
            name,
            description,
            images,
            latitude,
            longitude,
        } => create_request(
            deps,
            info,
            _env,
            name,
            description,
            images,
            latitude,
            longitude,
        ),
        ExecuteMsg::CreateOffer {
            price,
            images,
            request_id,
            store_name,
        } => create_offer(deps, info, _env, price, images, request_id, store_name),
        ExecuteMsg::AcceptOffer { offer_id } => accept_offer(deps, info, _env, offer_id),
        ExecuteMsg::ToggleLocation { enabled } => toggle_location(deps, info, _env, enabled),
        ExecuteMsg::DeleteRequest { request_id } => delete_request(deps, info, _env, request_id),
        ExecuteMsg::MarkRequestAsCompleted { request_id } => {
            mark_request_as_completed(deps, info, _env, request_id)
        }
    }
}

pub fn create_user(
    deps: DepsMut,
    info: MessageInfo,
    _env: Env,
    username: String,
    phone: String,
    latitude: i128,
    longitude: i128,
    account_type: AccountType,
) -> Result<Response, MarketplaceError> {
    let user_count = USER_COUNT.load(deps.storage)?;
    let user = User {
        id: user_count,
        username,
        phone,
        location: Location {
            latitude,
            longitude,
        },
        created_at: _env.block.time.seconds(),
        updated_at: _env.block.time.seconds(),
        account_type,
        location_enabled: true,
        authority: info.sender.clone(),
    };

    USERS.save(deps.storage, info.sender.as_bytes(), &user)?;
    USERS_BY_ID.save(deps.storage, user_count, &user)?;
    USER_COUNT.save(deps.storage, &(user_count + 1))?;

    Ok(Response::new().add_attribute("method", "create_user"))
}

pub fn update_user(
    deps: DepsMut,
    info: MessageInfo,
    _env: Env,
    username: String,
    phone: String,
    latitude: i128,
    longitude: i128,
    account_type: AccountType,
) -> Result<Response, MarketplaceError> {
    let mut user = USERS.load(deps.storage, info.sender.as_bytes())?;

    user.username = username;
    user.phone = phone;
    user.location = Location {
        latitude,
        longitude,
    };
    user.account_type = account_type;
    user.updated_at = _env.block.time.seconds();

    USERS.save(deps.storage, info.sender.as_bytes(), &user)?;

    Ok(Response::new().add_attribute("method", "update_user"))
}

pub fn create_store(
    deps: DepsMut,
    info: MessageInfo,
    _env: Env,
    name: String,
    description: String,
    phone: String,
    latitude: i128,
    longitude: i128,
) -> Result<Response, MarketplaceError> {
    let user = USERS.load(deps.storage, info.sender.as_bytes())?;
    let store_count = STORE_COUNT.load(deps.storage)?;

    if user.account_type != AccountType::Seller {
        return Err(MarketplaceError::OnlySellersAllowed);
    }
    let store = Store {
        id: store_count, // Logic for unique ID
        name,
        description,
        phone,
        location: Location {
            latitude,
            longitude,
        },
    };

    STORES.save(deps.storage, store.id, &store)?;
    let mut store_ids = USER_STORE_IDS
        .load(deps.storage, info.sender.as_bytes())
        .unwrap_or_default();
    store_ids.push(store.id);
    USER_STORE_IDS.save(deps.storage, info.sender.as_bytes(), &store_ids)?;
    STORE_COUNT.save(deps.storage, &(store_count + 1))?;
    Ok(Response::new().add_attribute("method", "create_store"))
}
pub fn create_request(
    deps: DepsMut,
    info: MessageInfo,
    _env: Env,
    name: String,
    description: String,
    images: Vec<String>,
    latitude: i128,
    longitude: i128,
) -> Result<Response, MarketplaceError> {
    let request_count = REQUEST_COUNT.load(deps.storage)?;
    let user = USERS.load(deps.storage, info.sender.as_bytes())?;

    if user.account_type != AccountType::Buyer {
        return Err(MarketplaceError::OnlyBuyersAllowed);
    }
    let request = Request {
        id: request_count,
        name,
        buyer_id: user.id,
        seller_price_quote: 0,
        seller_ids: vec![],
        offer_ids: vec![],
        locked_seller_id: 0,
        description,
        images,
        created_at: _env.block.time.seconds(),
        lifecycle: RequestLifecycle::Pending,
        location: Location {
            latitude,
            longitude,
        },
        updated_at: _env.block.time.seconds(),
        paid: false,
        accepted_offer_id: 0,
    };

    REQUESTS.save(deps.storage, request.id, &request)?;
    REQUEST_COUNT.save(deps.storage, &(request_count + 1))?;

    Ok(Response::new().add_attribute("method", "create_request"))
}
pub fn create_offer(
    deps: DepsMut,
    info: MessageInfo,
    _env: Env,
    price: u128,
    images: Vec<String>,
    request_id: u64,
    store_name: String,
) -> Result<Response, MarketplaceError> {
    let offer_count = OFFER_COUNT.load(deps.storage)?;
    let user = USERS.load(deps.storage, info.sender.as_bytes())?;

    if user.account_type != AccountType::Seller {
        return Err(MarketplaceError::OnlySellersAllowed);
    }

    let mut request = REQUESTS.load(deps.storage, request_id)?;

    if request.lifecycle != RequestLifecycle::Pending {
        request.lifecycle = RequestLifecycle::AcceptedBySeller;
    }

    request.seller_ids.push(user.id);
    request.offer_ids.push(offer_count);

    REQUESTS.save(deps.storage, request_id, &request)?;

    let offer = Offer {
        id: offer_count,
        price,
        images,
        request_id,
        store_name,
        seller_id: user.id,
        is_accepted: false,
        created_at: _env.block.time.seconds(),
        updated_at: _env.block.time.seconds(),
        authority: info.sender.clone(),
    };

    OFFERS.save(deps.storage, offer.id, &offer)?;
    OFFER_COUNT.save(deps.storage, &(offer_count + 1))?;
    deps.api.debug("Offer saved successfully");

    Ok(Response::new().add_attribute("method", "create_offer"))
}
pub fn accept_offer(
    deps: DepsMut,
    info: MessageInfo,
    _env: Env,
    offer_id: u64,
) -> Result<Response, MarketplaceError> {
    let mut offer = OFFERS.load(deps.storage, offer_id)?;
    let buyer = USERS.load(deps.storage, info.sender.as_bytes())?;
    let mut request = REQUESTS.load(deps.storage, offer.request_id)?;

    if buyer.account_type != AccountType::Buyer {
        return Err(MarketplaceError::OnlyBuyersAllowed);
    }

    if offer.is_accepted {
        return Err(MarketplaceError::OfferAlreadyAccepted);
    }

    if _env.block.time.seconds() > request.updated_at + TIME_TO_LOCK
        && request.lifecycle == RequestLifecycle::AcceptedByBuyer
    {
        return Err(MarketplaceError::RequestLocked);
    }

    for offer_id in request.offer_ids.iter() {
        let mut offer = OFFERS.load(deps.storage, *offer_id)?;
        offer.is_accepted = false;
        OFFERS.save(deps.storage, offer.id, &offer)?;
    }

    offer.is_accepted = true;
    offer.updated_at = _env.block.time.seconds();
    request.lifecycle = RequestLifecycle::AcceptedByBuyer;
    request.locked_seller_id = offer.seller_id;
    request.seller_price_quote = offer.price;

    OFFERS.save(deps.storage, offer.id, &offer)?;
    REQUESTS.save(deps.storage, request.id, &request)?;

    Ok(Response::new().add_attribute("method", "accept_offer"))
}

pub fn delete_request(
    deps: DepsMut,
    info: MessageInfo,
    _env: Env,
    request_id: u64,
) -> Result<Response, MarketplaceError> {
    let request = REQUESTS.load(deps.storage, request_id)?;
    let user = USERS.load(deps.storage, info.sender.as_bytes())?;

    if user.id != request.buyer_id {
        return Err(MarketplaceError::UnauthorizedBuyer);
    }

    if request.lifecycle != RequestLifecycle::Pending {
        return Err(MarketplaceError::RequestLocked);
    }

    REQUESTS.remove(deps.storage, request_id);

    Ok(Response::new().add_attribute("method", "delete_request"))
}

pub fn toggle_location(
    deps: DepsMut,
    info: MessageInfo,
    _env: Env,
    enabled: bool,
) -> Result<Response, MarketplaceError> {
    let mut user = USERS.load(deps.storage, info.sender.as_bytes())?;
    user.location_enabled = enabled;

    USERS.save(deps.storage, info.sender.as_bytes(), &user)?;

    Ok(Response::new().add_attribute("method", "toggle_location"))
}

pub fn mark_request_as_completed(
    deps: DepsMut,
    info: MessageInfo,
    _env: Env,
    request_id: u64,
) -> Result<Response, MarketplaceError> {
    let mut request = REQUESTS.load(deps.storage, request_id)?;
    let user = USERS.load(deps.storage, info.sender.as_bytes())?;
    let payment_info = PAYMENT_INFO.load(deps.storage, request_id)?;

    if user.id != request.buyer_id {
        return Err(MarketplaceError::UnauthorizedBuyer);
    }

    if request.lifecycle != RequestLifecycle::Paid {
        return Err(MarketplaceError::RequestNotAccepted);
    }

    if request.updated_at.checked_add(TIME_TO_LOCK).unwrap() > _env.block.time.seconds() {
        return Err(MarketplaceError::RequestNotLocked);
    }

    if user.account_type != AccountType::Buyer {
        return Err(MarketplaceError::OnlyBuyersAllowed);
    }

    request.lifecycle = RequestLifecycle::Completed;
    request.updated_at = _env.block.time.seconds();

    REQUESTS.save(deps.storage, request_id, &request)?;

    let amount: u128 = payment_info.amount.into();

    if amount > 0 {
        if payment_info.coin == CoinPayment::USDT {
            // Transfer USDT
            let transfer_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: USDT_ADDR.to_string(),
                msg: to_json_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: payment_info.seller.to_string(),
                    amount: payment_info.amount.into(),
                })?,
                funds: vec![],
            });

            return Ok(Response::new()
                .add_message(transfer_msg)
                .add_attribute("method", "mark_request_as_completed"));
        } else if payment_info.coin == CoinPayment::Cosmos {
            // Transfer native tokens
            let transfer_msg: CosmosMsg = CosmosMsg::Bank(cosmwasm_std::BankMsg::Send {
                to_address: payment_info.seller.to_string(),
                amount: vec![Coin {
                    denom: COIN_DENOM.to_string(), // Replace with the correct denom
                    amount: payment_info.amount,
                }],
            });

            return Ok(Response::new()
                .add_message(transfer_msg)
                .add_attribute("method", "mark_request_as_completed"));
        } else {
            return Err(MarketplaceError::UnknownPaymentType);
        }
    }

    Ok(Response::new().add_attribute("method", "mark_request_as_completed"))
}

pub fn pay_for_request_token(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    request_id: u64,
    coin: CoinPayment,
) -> Result<Response, MarketplaceError> {
    let mut request = REQUESTS.load(deps.storage, request_id)?;
    let offer_id = request.accepted_offer_id;
    let offer = OFFERS.load(deps.storage, offer_id)?;

    let user = USERS.load(deps.storage, info.sender.as_bytes())?;

    if request.paid {
        return Err(MarketplaceError::RequestAlreadyPaid);
    }

    if request.buyer_id != user.id {
        return Err(MarketplaceError::UnauthorizedBuyer);
    }
    if request.lifecycle != RequestLifecycle::AcceptedByBuyer {
        return Err(MarketplaceError::RequestNotAccepted);
    }
    if request.updated_at + TIME_TO_LOCK > env.block.time.seconds() {
        return Err(MarketplaceError::RequestNotLocked);
    }

    if !offer.is_accepted {
        return Err(MarketplaceError::RequestNotAccepted);
    }

    request.paid = true;
    request.lifecycle = RequestLifecycle::Paid;

    let mut new_payment_info = PaymentInfo {
        buyer: info.sender.clone(),
        request_id,
        seller: offer.authority.clone(),
        authority: info.sender.clone(),
        amount: Uint128::zero(),
        coin: coin.clone(),
        created_at: env.block.time.seconds(),
        updated_at: env.block.time.seconds(),
    };

    if coin == CoinPayment::USDT {
        //TODO: Calculate amount based on a price feed or some other logic
        let usdt_amount = offer.price; // Assuming price calculation logic is done elsewhere
        new_payment_info.amount = Uint128::from(usdt_amount);

        // Transfer USDT
        let transfer_msg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: USDT_ADDR.to_string(),
            msg: to_json_binary(&Cw20ExecuteMsg::TransferFrom {
                owner: info.sender.to_string(),
                recipient: env.contract.address.to_string(),
                amount: usdt_amount.into(),
            })?,
            funds: vec![],
        });

        PAYMENT_INFO.save(deps.storage, request_id, &new_payment_info)?;

        Ok(Response::new().add_message(transfer_msg).add_event(
            cosmwasm_std::Event::new("request_payment_transacted")
                .add_attribute("amount", new_payment_info.amount.to_string())
                .add_attribute("coin", format!("{:?}", coin))
                .add_attribute("request_id", request_id.to_string())
                .add_attribute("authority", offer.authority.to_string())
                .add_attribute("buyer", info.sender.to_string()),
        ))
    } else {
        Err(MarketplaceError::UnknownPaymentType)
    }
}

pub fn pay_for_request(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    request_id: u64,
    coin: CoinPayment,
) -> Result<Response, MarketplaceError> {
    let mut request = REQUESTS.load(deps.storage, request_id)?;
    let offer_id = request.accepted_offer_id;
    let offer = OFFERS.load(deps.storage, offer_id)?;
    let user = USERS.load(deps.storage, info.sender.as_bytes())?;

    if request.paid {
        return Err(MarketplaceError::RequestAlreadyPaid);
    }
    if request.buyer_id != user.id {
        return Err(MarketplaceError::UnauthorizedBuyer);
    }
    if request.lifecycle != RequestLifecycle::AcceptedByBuyer {
        return Err(MarketplaceError::RequestNotAccepted);
    }
    if request.updated_at + TIME_TO_LOCK > env.block.time.seconds() {
        return Err(MarketplaceError::RequestNotLocked);
    }

    if !offer.is_accepted {
        return Err(MarketplaceError::RequestNotAccepted);
    }

    request.paid = true;
    request.lifecycle = RequestLifecycle::Paid;

    let mut new_payment_info = PaymentInfo {
        buyer: info.sender.clone(),
        request_id,
        authority: info.sender.clone(),
        seller: offer.authority.clone(),
        amount: Uint128::zero(),
        coin: coin.clone(),
        created_at: env.block.time.seconds(),
        updated_at: env.block.time.seconds(),
    };

    if coin == CoinPayment::Cosmos {
        // Check if the correct amount of native tokens was sent
        let amount_sent = info
            .funds
            .iter()
            .find(|c| c.denom == COIN_DENOM)
            .map(|c| c.amount)
            .unwrap_or_default();
        if amount_sent < Uint128::from(offer.price) {
            return Err(MarketplaceError::InsufficientFunds);
        }

        new_payment_info.amount = Uint128::from(offer.price);
        PAYMENT_INFO.save(deps.storage, request_id, &new_payment_info)?;

        Ok(Response::new().add_event(
            cosmwasm_std::Event::new("request_payment_transacted")
                .add_attribute("amount", new_payment_info.amount.to_string())
                .add_attribute("coin", format!("{:?}", coin))
                .add_attribute("request_id", request_id.to_string())
                .add_attribute("authority", offer.authority.to_string())
                .add_attribute("buyer", info.sender.to_string()),
        ))
    } else {
        Err(MarketplaceError::UnknownPaymentType)
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetUser { address } => to_json_binary(&query_user(deps, address)?),
        QueryMsg::GetRequest { request_id } => to_json_binary(&query_request(deps, request_id)?),
        QueryMsg::GetAllRequests {} => to_json_binary(&query_all_requests(deps)?),
        QueryMsg::GetOffer { offer_id } => to_json_binary(&query_offer(deps, offer_id)?),
        QueryMsg::GetOffersByRequest { request_id } => {
            to_json_binary(&query_offers_by_request(deps, request_id)?)
        }

        QueryMsg::GetLocationPreference { address } => {
            let user = USERS.load(deps.storage, deps.api.addr_validate(&address)?.as_bytes())?;
            to_json_binary(&user.location_enabled)
        }

        QueryMsg::GetUserStores { address } => to_json_binary(&get_user_stores(deps, address)?),

        QueryMsg::GetUserRequests { address } => to_json_binary(&get_user_requests(deps, address)?),
        QueryMsg::GetUserPaymentHistory { address } => {
            to_json_binary(&get_user_payment_history(deps, address)?)
        }

        QueryMsg::GetSellerOffers { address } => to_json_binary(&get_seller_offers(deps, address)?),

        QueryMsg::GetUserById { user_id } => to_json_binary(&get_user_by_id(deps, user_id)?),
    }
}

pub fn query_user(deps: Deps, address: String) -> StdResult<User> {
    let addr = deps.api.addr_validate(&address)?;
    let user = USERS.load(deps.storage, addr.as_bytes())?;
    Ok(user)
}

pub fn get_user_by_id(deps: Deps, user_id: u64) -> StdResult<User> {
    let user = USERS_BY_ID.load(deps.storage, user_id)?;
    Ok(user)
}

pub fn get_user_stores(deps: Deps, address: String) -> StdResult<Vec<Store>> {
    let addr = deps.api.addr_validate(&address)?;
    let stores = USER_STORE_IDS.may_load(deps.storage, addr.as_bytes());

    if stores.is_err() {
        return Ok(vec![]);
    }

    let stored_id = stores.unwrap();

    if stored_id.is_none() {
        return Ok(vec![]);
    }

    let store_ids = stored_id.unwrap();
    let stores: Vec<Store> = store_ids
        .iter()
        .map(|store_id| STORES.load(deps.storage, *store_id))
        .collect::<StdResult<Vec<Store>>>()?;

    Ok(stores)
}

pub fn get_seller_offers(deps: Deps, address: String) -> StdResult<Vec<Offer>> {
    let addr = deps.api.addr_validate(&address)?;
    let user = USERS.load(deps.storage, addr.as_bytes())?;

    let offers: Vec<Offer> = OFFERS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .filter_map(|item| {
            match item {
                Ok((_, offer)) => {
                    // Filter based on the user's ID
                    if offer.seller_id == user.id {
                        Some(Ok(offer))
                    } else {
                        None
                    }
                }
                Err(_) => None,
                // Err(e) => Some(Err(e)),
            }
        })
        .collect::<StdResult<Vec<Offer>>>()?;

    Ok(offers)
}

pub fn query_request(deps: Deps, request_id: u64) -> StdResult<Request> {
    let request = REQUESTS.load(deps.storage, request_id)?;
    Ok(request)
}

pub fn query_all_requests(deps: Deps) -> StdResult<Vec<Request>> {
    let requests: Vec<Request> = REQUESTS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|item| {
            let (_, request) = item?;
            Ok(request)
        })
        .collect::<StdResult<Vec<Request>>>()?;

    Ok(requests)
}

pub fn query_offer(deps: Deps, offer_id: u64) -> StdResult<Offer> {
    let offer = OFFERS.load(deps.storage, offer_id)?;
    Ok(offer)
}

pub fn query_offers_by_request(deps: Deps, request_id: u64) -> StdResult<Vec<Offer>> {
    let request = REQUESTS.load(deps.storage, request_id)?;
    let offers: Vec<Offer> = request
        .offer_ids
        .iter()
        .map(|offer_id| OFFERS.load(deps.storage, *offer_id))
        .collect::<StdResult<Vec<Offer>>>()?;
    Ok(offers)
}

pub fn get_user_requests(deps: Deps, address: String) -> StdResult<Vec<Request>> {
    let addr: cosmwasm_std::Addr = deps.api.addr_validate(&address)?;
    let user = USERS.load(deps.storage, addr.as_bytes())?;

    let requests: Vec<Request> = REQUESTS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .filter_map(|item| {
            match item {
                Ok((_, request)) => {
                    // Filter based on the user's ID
                    if request.buyer_id == user.id {
                        Some(Ok(request))
                    } else {
                        None
                    }
                }
                Err(_) => None,
                // Err(e) => Some(Err(e)),
            }
        })
        .collect::<StdResult<Vec<Request>>>()?;

    Ok(requests)
}
pub fn get_user_payment_history(deps: Deps, address: String) -> StdResult<Vec<PaymentInfo>> {
    let addr: cosmwasm_std::Addr = deps.api.addr_validate(&address)?;
    let user = USERS.load(deps.storage, addr.as_bytes())?;

    let payments: Vec<PaymentInfo> = PAYMENT_INFO
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .filter_map(|item| {
            match item {
                Ok((_, payment_info)) => {
                    // Filter based on the user's ID
                    if payment_info.authority == user.authority {
                        Some(Ok(payment_info))
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        })
        .collect::<StdResult<Vec<PaymentInfo>>>()?;

    Ok(payments)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
//     use cosmwasm_std::{coins, from_binary};

//     #[test]
//     fn proper_initialization() {
//         let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

//         let msg = InstantiateMsg { count: 17 };
//         let info = mock_info("creator", &coins(1000, "earth"));

//         // we can just call .unwrap() to assert this was a success
//         let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
//         assert_eq!(0, res.messages.len());

//         // it worked, let's query the state
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount ).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(17, value.count);
//     }

//     #[test]
//     fn increment() {
//         let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

//         let msg = InstantiateMsg { count: 17 };
//         let info = mock_info("creator", &coins(2, "token"));
//         let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // beneficiary can release it
//         let info = mock_info("anyone", &coins(2, "token"));
//         let msg = ExecuteMsg::Increment {};
//         let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // should increase counter by 1
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount ).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(18, value.count);
//     }

//     #[test]
//     fn reset() {
//         let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

//         let msg = InstantiateMsg { count: 17 };
//         let info = mock_info("creator", &coins(2, "token"));
//         let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // beneficiary can release it
//         let unauth_info = mock_info("anyone", &coins(2, "token"));
//         let msg = ExecuteMsg::Reset { count: 5 };
//         let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
//         match res {
//             Err(MarketplaceError::Unauthorized ) => {}
//             _ => panic!("Must return unauthorized error"),
//         }

//         // only the original creator can reset the counter
//         let auth_info = mock_info("creator", &coins(2, "token"));
//         let msg = ExecuteMsg::Reset { count: 5 };
//         let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

//         // should now be 5
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount ).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(5, value.count);
//     }
// }
