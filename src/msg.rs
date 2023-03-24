use cosmwasm_std::Addr;
use serde::{Serialize, Deserialize};

// ------------------------Enums that represent messages-----------------------
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct InstantiateMsg {
    pub users: Vec<String>,
    pub total_funds: String,
}



pub enum  ExecuteMsg {
    AddUser{ username: String },
    Leave{ },
    CollectFunds{ },
}



#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum QueryMsg {
    // Represents a message that triggers
    // execution of the contract's query()
    // entry point function
    Greet{ greeting: String }, // question: do I have to use struct representation
                    // for the variant's content (e.g. Greet{ greeting: String })
                    // or will the variant be serialized correctly anyways?
                    // Answer: better to use struct repr.
    Goodbye{ goodbye: String },
    ListUsers{ },
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
pub struct UserListResp {
    pub user_list: Vec<Addr>,
}
