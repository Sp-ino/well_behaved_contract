use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult, to_binary,
};
use crate::msg::*;




#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}



#[entry_point]
pub fn query(
    _deps: DepsMut,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {

    match msg {
        QueryMsg::Greet(greeting) => to_binary(&)
    }

}



pub fn greet()
