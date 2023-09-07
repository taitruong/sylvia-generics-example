use cosmwasm_std::{Response, StdError, StdResult};
use cw_storage_plus::Item;
use sylvia::{contract, entry_points, types::InstantiateCtx};
use thiserror::Error;

pub struct MyContract<'a> {
    /// Map of sender's last callback.
    pub(crate) data: Item<'a, String>,
}

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
}

#[entry_points]
#[contract]
#[error(ContractError)]
#[messages(crate::my_interface as MyInterface)]
impl MyContract<'_> {
    pub const fn new() -> Self {
        Self {
            data: Item::new("data"),
        }
    }

    #[msg(instantiate)]
    pub fn instantiate(&self, _ctx: InstantiateCtx) -> StdResult<Response> {
        Ok(Response::default())
    }
}
