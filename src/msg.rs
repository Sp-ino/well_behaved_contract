use serde::{Serialize, Deserialize};

// ------------------------Enums that represent messages-----------------------
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct InstantiateMsg {
    pub users: Vec<String>,
}



pub enum  ExecuteMsg {
    AddUser{ username: String },
    Leave{},
}



#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum QueryMsg {
    // Represents a message that triggers
    // execution of the contract's query()
    // entry point function
    Greet{ greeting: String }, // Better to use struct inside variant, even if it has only one field
    Goodbye{ goodbye: String },
}



// ------------------------Structs that represent responses to queries--------
#[derive(Serialize, Deserialize)]
pub struct GreetResp {
    pub greeting: String,
}



#[derive(Serialize, Deserialize)]
pub struct GoodbyeResp {
    pub goodbye: String,
}