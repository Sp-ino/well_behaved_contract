use thiserror::Error;
use cosmwasm_std::{Addr, StdError};


#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),

    #[error("User is not registered")]
    AlreadyRegistered{ },

    #[error("User is already registered")]
    NotRegistered{ sender: Addr },
}