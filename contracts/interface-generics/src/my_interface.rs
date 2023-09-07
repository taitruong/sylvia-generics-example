use cosmwasm_std::{Response, StdError, StdResult};
use sylvia::{
    interface,
    types::{ExecCtx, QueryCtx},
};

#[interface]
pub trait MyInterface<T> {
    type Error: From<StdError>;

    #[msg(exec)]
    fn save_data(&self, ctx: ExecCtx, data: T) -> StdResult<Response>;

    #[msg(query)]
    fn get_data(&self, ctx: QueryCtx) -> StdResult<T>;
}
