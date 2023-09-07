use cosmwasm_std::{Response, StdResult};
use sylvia::{
    contract,
    types::{ExecCtx, QueryCtx},
};

use crate::{
    contract::{ContractError, MyContract},
    my_interface::MyInterface,
};

#[contract(module=crate::contract)]
#[messages(crate::my_interface: exec<String>, query<String> as MyInterface)]
impl MyInterface for MyContract<'_> {
    type Error = ContractError;

    #[msg(exec)]
    fn save_data(&self, ctx: ExecCtx, data: String) -> StdResult<Response> {
        self.data.save(ctx.deps.storage, &data)?;
        Ok(Response::default())
    }

    #[msg(query)]
    fn get_data(&self, ctx: QueryCtx) -> StdResult<String> {
        self.data.load(ctx.deps.storage)
    }
}
