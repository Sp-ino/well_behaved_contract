use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary, StdError,
};
use crate::msg::*;
use crate::state::{USERS, FUNDS};




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
    FUNDS.save(deps.storage, &msg.total_funds)?;
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
        ExecuteMsg::Leave{  } => leave(deps, info),
        ExecuteMsg::CollectFunds{  } => collect_funds(deps, info),
    }
}


pub fn collect_funds(deps: DepsMut, info: MessageInfo) -> StdResult<Response> { 

    Ok(Response::new())
}



pub fn add_user(username: String, deps: DepsMut, _info: MessageInfo) -> StdResult<Response> {
    let mut users = USERS.load(deps.storage)?;
    let new_user_addr = deps.api.addr_validate(&username)?;

    if users.contains(&new_user_addr) {
        return Err(StdError::GenericErr { msg: "User is already inside the user list".to_string() })
    }

    users.push(new_user_addr);
    USERS.save(deps.storage, &users)?;

    Ok(Response::new())
}



pub fn leave(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    let mut users = USERS.load(deps.storage)?;
    let sender = info.sender;

    // This part should be improved by removing the following check
    // and handling the wrapped output of .position with a if let
    if !users.contains(&sender) {
        return Err(StdError::GenericErr { msg: "User to be removed is not in user list".to_string() })
    }

    let user_idx = users.iter()
                                .position(|x| *x == sender)
                                .unwrap();
    users.remove(user_idx);
    USERS.save(deps.storage, &users)?;

    Ok(Response::new())
}




#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {

    match msg {
        QueryMsg::Greet{ greeting: grtng } => to_binary(&greet(grtng)?),
        QueryMsg::Goodbye{ goodbye: gbye } => to_binary(&say_goodbye(gbye)?),
        QueryMsg::ListUsers{ } => to_binary(&list_users(deps)?),
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


pub fn list_users(deps: Deps) -> StdResult<UserListResp> {
    let usrs = USERS.load(deps.storage)?;

    let userlst = UserListResp {
        user_list: usrs,
    };

    Ok(userlst)
}