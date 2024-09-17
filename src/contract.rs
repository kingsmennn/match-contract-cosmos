#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::MarketplaceError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{
    AccountType, Location, Offer, Request, RequestLifecycle, Store, User, OFFERS, OFFER_COUNT,
    REQUESTS, REQUEST_COUNT, STORES, STORE_COUNT, TIME_TO_LOCK, USERS, USER_COUNT, USER_STORE_IDS,
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:marketplace";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
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
        ExecuteMsg::DeleteRequest { offer_id } => delete_request(deps, info, _env, offer_id),
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
) -> StdResult<Response> {
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
    };

    USERS.save(deps.storage, info.sender.as_bytes(), &user)?;
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
) -> StdResult<Response> {
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
) -> StdResult<Response> {
    let user = USERS.load(deps.storage, info.sender.as_bytes())?;
    let store_count = STORE_COUNT.load(deps.storage)?;

    if user.account_type != AccountType::Seller {
        // return Err(MarketplaceError::OnlySellersAllowed.into());
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
    // USER_STORE_IDS.update(
    //     deps.storage,
    //     info.sender.as_bytes(),
    //     |existing| match existing {
    //         Some(mut stores) => {
    //             stores.push(store.id);
    //             Ok(stores)
    //         }
    //         None => Ok(vec![store.id]),
    //     },
    // )?;

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
) -> StdResult<Response> {
    let request_count = REQUEST_COUNT.load(deps.storage)?;
    let user = USERS.load(deps.storage, info.sender.as_bytes())?;

    if user.account_type != AccountType::Buyer {
        // return Err(MarketplaceError::OnlyBuyersAllowed.into());
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
    };

    REQUESTS.save(deps.storage, request.id, &request)?;
    REQUEST_COUNT.save(deps.storage, &(request_count + 1))?;

    Ok(Response::new().add_attribute("method", "create_request"))
}
pub fn create_offer(
    deps: DepsMut,
    info: MessageInfo,
    _env: Env,
    price: i128,
    images: Vec<String>,
    request_id: u64,
    store_name: String,
) -> StdResult<Response> {
    let offer_count = OFFER_COUNT.load(deps.storage)?;
    let user = USERS.load(deps.storage, info.sender.as_bytes())?;

    if user.account_type != AccountType::Seller {
        // return Err(MarketplaceError::OnlySellersAllowed.into());
    }

    let mut request = REQUESTS.load(deps.storage, request_id)?;

    if request.lifecycle != RequestLifecycle::Pending {
        request.lifecycle = RequestLifecycle::AcceptedBySeller;
    }

    request.seller_ids.push(user.id);
    request.offer_ids.push(offer_count);

    REQUESTS.save(deps.storage, request.id, &request)?;

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
    };

    OFFERS.save(deps.storage, offer.id, &offer)?;
    OFFER_COUNT.save(deps.storage, &(offer_count + 1))?;

    Ok(Response::new().add_attribute("method", "create_offer"))
}
pub fn accept_offer(
    deps: DepsMut,
    info: MessageInfo,
    _env: Env,
    offer_id: u64,
) -> StdResult<Response> {
    let mut offer = OFFERS.load(deps.storage, offer_id)?;
    let buyer = USERS.load(deps.storage, info.sender.as_bytes())?;
    let mut request = REQUESTS.load(deps.storage, offer.request_id)?;

    if buyer.account_type != AccountType::Buyer {
        // return Err(MarketplaceError::UnauthorizedBuyer);
    }

    if offer.is_accepted {
        // return Err(MarketplaceError::OfferAlreadyAccepted);
    }

    // Update the associated request lifecycle

    if _env.block.time.seconds() > request.updated_at + TIME_TO_LOCK
        && request.lifecycle == RequestLifecycle::AcceptedByBuyer
    {
        // return Err(MarketplaceError::RequestLocked);
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
) -> StdResult<Response> {
    let request = REQUESTS.load(deps.storage, request_id)?;
    let user = USERS.load(deps.storage, info.sender.as_bytes())?;

    if user.id != request.buyer_id {
        // return Err(MarketplaceError::UnauthorizedBuyer);
    }

    if request.lifecycle != RequestLifecycle::Pending {
        // return Err(MarketplaceError::RequestLocked);
    }

    REQUESTS.remove(deps.storage, request_id);

    Ok(Response::new().add_attribute("method", "delete_request"))
}

pub fn toggle_location(
    deps: DepsMut,
    info: MessageInfo,
    _env: Env,
    enabled: bool,
) -> StdResult<Response> {
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
) -> StdResult<Response> {
    let mut request = REQUESTS.load(deps.storage, request_id)?;
    let user = USERS.load(deps.storage, info.sender.as_bytes())?;

    if user.id != request.buyer_id {
        // return Err(MarketplaceError::UnauthorizedBuyer);
    }

    if request.lifecycle != RequestLifecycle::AcceptedByBuyer {
        // return Err(MarketplaceError::RequestNotAccepted);
    }

    if request.updated_at.checked_add(TIME_TO_LOCK).unwrap() > _env.block.time.seconds() {
        // return Err(MarketplaceError::RequestNotLocked);
    }

    if user.account_type != AccountType::Buyer {
        // return Err(MarketplaceError::OnlyBuyersAllowed);
    }

    request.lifecycle = RequestLifecycle::Completed;
    request.updated_at = _env.block.time.seconds();

    REQUESTS.save(deps.storage, request_id, &request)?;

    Ok(Response::new().add_attribute("method", "mark_request_as_completed"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetUser { address } => to_binary(&query_user(deps, address)?),
        QueryMsg::GetAllUsers {} => to_binary(&query_all_users(deps)?),
        QueryMsg::GetRequest { request_id } => to_binary(&query_request(deps, request_id)?),
        QueryMsg::GetAllRequests {} => to_binary(&query_all_requests(deps)?),
        QueryMsg::GetOffer { offer_id } => to_binary(&query_offer(deps, offer_id)?),
        QueryMsg::GetOffersForRequest { request_id } => {
            to_binary(&query_offers_for_request(deps, request_id)?)
        }
        QueryMsg::GetAllOffers {} => to_binary(&query_all_offers(deps)?),
    }
}

pub fn query_user(deps: Deps, address: String) -> StdResult<User> {
    let addr = deps.api.addr_validate(&address)?;
    let user = USERS.load(deps.storage, addr.as_bytes())?;
    Ok(user)
}

pub fn query_all_users(deps: Deps) -> StdResult<Vec<User>> {
    let users: Vec<User> = USERS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|item| {
            let (_, user) = item?;
            Ok(user)
        })
        .collect::<StdResult<Vec<User>>>()?;

    Ok(users)
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

pub fn query_offers_for_request(deps: Deps, request_id: u64) -> StdResult<Vec<Offer>> {
    let request = REQUESTS.load(deps.storage, request_id)?;
    let offers: Vec<Offer> = request
        .offer_ids
        .iter()
        .map(|offer_id| OFFERS.load(deps.storage, *offer_id))
        .collect::<StdResult<Vec<Offer>>>()?;
    Ok(offers)
}

pub fn query_all_offers(deps: Deps) -> StdResult<Vec<Offer>> {
    let offers: Vec<Offer> = OFFERS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|item| {
            let (_, offer) = item?;
            Ok(offer)
        })
        .collect::<StdResult<Vec<Offer>>>()?;

    Ok(offers)
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
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
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
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
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
//             Err(ContractError::Unauthorized {}) => {}
//             _ => panic!("Must return unauthorized error"),
//         }

//         // only the original creator can reset the counter
//         let auth_info = mock_info("creator", &coins(2, "token"));
//         let msg = ExecuteMsg::Reset { count: 5 };
//         let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

//         // should now be 5
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(5, value.count);
//     }
// }
