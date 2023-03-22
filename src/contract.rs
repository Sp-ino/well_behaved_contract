use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary,
};
// use cw20::{Cw20ReceiveMsg};
use crate::msg::*;
use crate::state::USERS;




#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let users: StdResult<Vec<_>> = msg
                                    .users
                                    .into_iter()
                                    .map(|addr| deps.api.addr_validate(&addr))
                                    .collect();
    USERS.save(deps.storage, &users?)?;

    Ok(Response::new())
}


#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::AddUser{username: un} => add_user(un, deps, info),
        ExecuteMsg::Leave{} => leave(),
    }
}



pub fn add_user(username: String, deps: DepsMut, _info: MessageInfo) -> StdResult<Response> {
    let new_user_addr = deps.api.addr_validate(&username)?;

    let mut users = USERS.load(deps.storage)?;
    users.push(new_user_addr);
    
    USERS.save(deps.storage, &users)?;

    Ok(Response::new())
}



pub fn leave() -> StdResult<Response>{

    Ok(Response::new())
}




#[entry_point]
pub fn query(
    _deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {

    match msg {
        QueryMsg::Greet{ greeting: grtng } => to_binary(&greet(grtng)?),
        QueryMsg::Goodbye{ goodbye: gbye } => to_binary(&say_goodbye(gbye)?),        
    }
}



pub fn greet(gr: String) -> StdResult<GreetResp> {
    let greetresp = GreetResp {
        greeting: gr.to_owned(),
    };

    Ok(greetresp)
}



pub fn say_goodbye(gb: String)-> StdResult<GoodbyeResp> {
    let goodbyeresp = GoodbyeResp {
        goodbye: gb.to_owned(),
    };

    Ok(goodbyeresp)
}