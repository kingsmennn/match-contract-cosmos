#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::MarketplaceError;
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{
    AccountType, Location, Offer, Request, RequestLifecycle, Store, User, OFFERS, REQUESTS, USERS,
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

// Create User Function
pub fn create_user(
    deps: DepsMut,
    info: MessageInfo,
    username: String,
    phone: String,
    latitude: i128,
    longitude: i128,
    account_type: AccountType,
) -> StdResult<Response> {
    let user = User {
        id: 1, // Increment logic
        username,
        phone,
        location: Location {
            latitude,
            longitude,
        },
        created_at: deps.api.block_time().seconds(),
        updated_at: deps.api.block_time().seconds(),
        account_type,
        location_enabled: true,
    };

    USERS.save(deps.storage, info.sender.as_bytes(), &user)?;
    Ok(Response::new().add_attribute("method", "create_user"))
}

pub fn update_user(
    deps: DepsMut,
    info: MessageInfo,
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
    user.updated_at = deps.api.block_time().seconds();

    USERS.save(deps.storage, info.sender.as_bytes(), &user)?;

    Ok(Response::new().add_attribute("method", "update_user"))
}
pub fn create_store(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
    description: String,
    phone: String,
    latitude: i128,
    longitude: i128,
) -> StdResult<Response> {
    let store = Store {
        id: 1, // Logic for unique ID
        name,
        description,
        phone,
        location: Location {
            latitude,
            longitude,
        },
    };

    // Save the store data in a map or add to user's profile as needed.

    Ok(Response::new().add_attribute("method", "create_store"))
}
pub fn create_request(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
    description: String,
    images: Vec<String>,
    latitude: i128,
    longitude: i128,
) -> StdResult<Response> {
    let request = Request {
        id: 1, // Logic for unique ID
        name,
        buyer_id: 1, // Assume buyer_id is fetched from user profile
        seller_price_quote: 0,
        seller_ids: vec![],
        offer_ids: vec![],
        locked_seller_id: 0,
        description,
        images,
        created_at: deps.api.block_time().seconds(),
        lifecycle: RequestLifecycle::Pending,
        location: Location {
            latitude,
            longitude,
        },
        updated_at: deps.api.block_time().seconds(),
    };

    // Save request in the REQUESTS map.
    REQUESTS.save(deps.storage, request.id, &request)?;

    Ok(Response::new().add_attribute("method", "create_request"))
}
pub fn create_offer(
    deps: DepsMut,
    info: MessageInfo,
    price: i128,
    images: Vec<String>,
    request_id: u64,
    store_name: String,
) -> StdResult<Response> {
    let offer = Offer {
        id: 1, // Logic for unique ID
        price,
        images,
        request_id,
        store_name,
        seller_id: 1, // Assume seller_id is fetched from user profile
        is_accepted: false,
        created_at: deps.api.block_time().seconds(),
        updated_at: deps.api.block_time().seconds(),
    };

    // Save the offer in the OFFERS map
    OFFERS.save(deps.storage, offer.id, &offer)?;

    Ok(Response::new().add_attribute("method", "create_offer"))
}
pub fn accept_offer(deps: DepsMut, info: MessageInfo, offer_id: u64) -> StdResult<Response> {
    let mut offer = OFFERS.load(deps.storage, offer_id)?;
    offer.is_accepted = true;
    offer.updated_at = deps.api.block_time().seconds();

    // Update the associated request lifecycle
    let mut request = REQUESTS.load(deps.storage, offer.request_id)?;
    request.lifecycle = RequestLifecycle::AcceptedByBuyer;
    request.locked_seller_id = offer.seller_id;

    OFFERS.save(deps.storage, offer.id, &offer)?;
    REQUESTS.save(deps.storage, request.id, &request)?;

    Ok(Response::new().add_attribute("method", "accept_offer"))
}

pub fn toggle_location(deps: DepsMut, info: MessageInfo, enabled: bool) -> StdResult<Response> {
    let mut user = USERS.load(deps.storage, info.sender.as_bytes())?;
    user.location_enabled = enabled;

    USERS.save(deps.storage, info.sender.as_bytes(), &user)?;

    Ok(Response::new().add_attribute("method", "toggle_location"))
}
pub fn remove_offer(deps: DepsMut, info: MessageInfo, offer_id: u64) -> StdResult<Response> {
    let offer = OFFERS.load(deps.storage, offer_id)?;

    if offer.is_accepted {
        return Err(StdError::generic_err("Cannot remove accepted offer"));
    }

    OFFERS.remove(deps.storage, offer_id);

    Ok(Response::new().add_attribute("method", "remove_offer"))
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
    let offers: Vec<Offer> = OFFERS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|item| {
            let (_, offer) = item?;
            if offer.request_id == request_id {
                Ok(offer)
            } else {
                Err(StdError::generic_err("Offer not related to this request"))
            }
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }
}
