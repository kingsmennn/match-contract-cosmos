use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MarketplaceError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("User already exists.")]
    UserAlreadyExists,
    #[error("Invalid account type.")]
    InvalidAccountType,
    #[error("Invalid user.")]
    InvalidUser,
    #[error("Only sellers allowed.")]
    OnlySellersAllowed,
    #[error("Only buyers allowed.")]
    OnlyBuyersAllowed,
    #[error("Unauthorized buyer.")]
    UnauthorizedBuyer,
    #[error("Offer already accepted.")]
    OfferAlreadyAccepted,
    #[error("Request locked.")]
    RequestLocked,
    #[error("Incorrect number of sellers.")]
    IncorrectNumberOfSellers,
    #[error("Request not accepted.")]
    RequestNotAccepted,
    #[error("Request not locked.")]
    RequestNotLocked,
    #[error("Offer not found.")]
    OfferNotFound,
    #[error("Request already paid.")]
    RequestAlreadyPaid,
    #[error("Unknown payment type")]
    UnknownPaymentType,
    #[error("Insufficient funds")]
    InsufficientFunds,
}
