use cosmwasm_std::{Addr, Uint128};
use cw20::Cw20ReceiveMsg;
use serde::{Serialize, Deserialize};

// ------------------------Enums that represent messages-----------------------
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
    pub total_funds: Uint128,
    pub coin_contract: String,
}


#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum  ExecuteMsg {
    AddAdmin{ admin_name: String },
    Leave{ },
    ReceiveCw20(Cw20ReceiveMsg),
    SendCw20{
        amount: Uint128,    
        to: Addr,
    },
}



#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum QueryMsg {
    // Represents a message that triggers
    // execution of the contract's query()
    // entry point function
    Greet{ greeting: String }, // Better to use struct inside variant, even if it has only one field
    Goodbye{ goodbye: String },
    ListAdmins{ },
}



// ------------------------Structs that represent responses to queries--------
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct GreetResp {
    pub greeting: String,
}



#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct GoodbyeResp {
    pub goodbye: String,
}


#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct AdminListResp {
    pub admin_list: Vec<Addr>,
}
