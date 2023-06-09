use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo,
    Response, StdResult, to_binary, Addr, CosmosMsg, WasmMsg, Uint128};
use cw20::{Cw20ReceiveMsg, Cw20ExecuteMsg};
use crate::msg::*;
use crate::state::{State, Config, store_state, read_state, store_config, read_config};
use crate::error::ContractError;



#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let admins: StdResult<Vec<_>> = msg
                                    .admins
                                    .into_iter()
                                    .map(|addr| deps.api.addr_validate(&addr))
                                    .collect();
    let token_contract = deps.api.addr_validate(&msg.token_contract)?;

    let state = State {
        admins: admins?,
        balance: msg.balance,
    };
    store_state(deps.storage, &state)?;

    let config = Config {
        token_contract: token_contract,
    };
    store_config(deps.storage, &config)?;
    
    Ok(Response::new())
}


#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddAdmin{admin_name: un} => add_user(un, deps, info),
        ExecuteMsg::Leave{  } => leave(deps, info),
        ExecuteMsg::ReceiveCw20(msg) => receive_cw20(deps, msg),
        ExecuteMsg::SendCw20{ amount, to} => send_cw20(deps, info, amount, to),
    }
}



pub fn send_cw20(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
    to: Addr
) -> Result<Response, ContractError> {

    let msg_sender = info.sender;
    let state = read_state(deps.storage)?;
    let config = read_config(deps.storage)?;

    if !state.admins.contains(&msg_sender) {
        return Err(ContractError::NotRegistered { sender: msg_sender })
    }

    // There are two ways in which a smart contract can send tokens/coins
    // to an address. The first one consists in simply sending the currency without
    // triggering any actions. The second one allows send funds AND trigger
    // an execution on the receiving address (which must be a smart contract).
    // This is done differently depending on the fact the the sending contract
    // is transferring a coin or a token.
    // 
    // Consequently, the following cases can be considered:
    // - The currency is a coin -> if the currency is a coin, the transfer can be
    //   performed with the two following approaches, depending on whether or not
    //   a message must be sent to the receiving address:
    //      * If no action needs to be triggered the coin is transferred
    //        by producing a Response that contains a message of type BankMsg.
    //        BankMsg is a struct defined by CosmWasm that contains the receiving
    //        address and the amount to be sent to that address.
    //      * If an action needs to be triggered (which means the receiving
    //        address is a smart contract) then the coin is transferred by producing
    //        a Response that contains a message of type Execute (which must be wrapped
    //        in the Wasm variant of the CosmosMsg struct). The contract_addr field of
    //        the Execute message represents the receiving contract's address. The
    //        msg field indicates the action that will be triggered by the Execute
    //        message. The funds field contains a vector of Coin variables. In turn,
    //        Coin contains a denom field (the name of the coin) and an amount field. 
    // - The currency is a Cw20 token -> in CosmWasm, Cw20 tokens are actually smart
    //   contracts. More precisely, all Cw20 token transfers occur by sending an execute
    //   message to a smart contract that handles these operations. As in the previous
    //   case, two approaches are possible:
    //      * If no action needs to be triggered the token is transferred by sending
    //        an execute message to the token contract (i.e. by producing a Response
    //        that contains an Execute message whose contract_address is the token
    //        contract's address). Note the the contract_address _does not represent
    //        the address to which the token is being sent, but rather the address of
    //        the contract that handles the transfer_. The funds field can be left empty
    //        if no coin transfer is involved. The msg field, instead, must contain a
    //        Cw20ExecuteMsg::Transfer variable. The Cw20ExecuteMsg::Transfer struct has
    //        a recipient field (the address that actually receives the token) and an
    //        amount field.
    //      * If an action needs to be triggered the logic will be similar to the
    //        previous case, except for the fact that the msg field of the Execute
    //        message must contain a Cw20ExecuteMsg::Send variable. This struct contains
    //        a contract field (the contract that receives the token), an amount field
    //        and a msg field (the message that specifies the action to be executed).
    let resp = Response::new()
                            .add_message(CosmosMsg::Wasm(
                                WasmMsg::Execute {
                                    contract_addr: config.token_contract.to_string(),
                                    msg: to_binary(&Cw20ExecuteMsg::Transfer {
                                        recipient: to.to_string(),
                                        amount: amount,
                                    })?,
                                    funds: vec![],
                                }
                            ));

    Ok(resp)
}



pub fn receive_cw20(
    deps: DepsMut,
    cw20_receive_msg: Cw20ReceiveMsg
) -> Result<Response, ContractError> { 

    // Actually, there is no need for a state variable that
    // keeps track of the balance, as the contract can query its
    // own balance.
    // 
    // --------------------IMPORTANT!!!!!------------------------
    // When a smart contract handles a Cw20ReceiveMsg, its balance
    // is not updated until the contract's execute function returns
    // an Ok() value. Therefore, is the contract needs to know the
    // updated balance, it must query its current balance and then
    // add the quantity Cw20ReceiveMsg.amount to it.
    // 
    // It's also important that what is stated above DOES NOT APPLY
    // TO NATIVE COIN TRANSFERS. For example, if an ExecuteMsg with
    // non-empty funds field is received by the contract, its update
    // balance will be available from the moment the execute() entry
    // point is triggered.
    let amount: Uint128 = cw20_receive_msg.amount.into();

    let mut state = read_state(deps.storage)?;
    state.balance = state.balance + amount;
    store_state(deps.storage, &state)?;

    Ok(Response::new())
}



pub fn add_user(admin_name: String, deps: DepsMut, _info: MessageInfo) -> Result<Response, ContractError> {
    let mut state = read_state(deps.storage)?;
    let new_user_addr = deps.api.addr_validate(&admin_name)?;

    if state.admins.contains(&new_user_addr) {
        return Err(ContractError::AlreadyRegistered{ })
    }

    state
        .admins
        .push(new_user_addr);

    store_state(deps.storage, &state)?;

    Ok(Response::new())
}



pub fn leave(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let mut state = read_state(deps.storage)?;
    let msg_sender = info.sender;

    if !state.admins.contains(&msg_sender) {
        return Err(ContractError::NotRegistered { sender: msg_sender })
    }

    let user_idx = state.admins.iter()
                                    .position(|x| *x == msg_sender)
                                    .unwrap();
    state
        .admins
        .remove(user_idx);

    store_state(deps.storage, &state)?;

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
        QueryMsg::ListAdmins{ } => to_binary(&list_admins(deps)?),
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


pub fn list_admins(deps: Deps) -> StdResult<AdminListResp> {
    let state = read_state(deps.storage)?;

    let admlist = AdminListResp {
        admin_list: state.admins,
    };

    Ok(admlist)
}