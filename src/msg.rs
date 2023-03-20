use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum QueryMsg {
    Greet(String),
    Goodbye(String),
}



#[derive(Serialize, Deserialize)]
pub struct GreetResp {
    pub greeting: String,
}



#[derive(Serialize, Deserialize)]
pub struct GoodbyeResp {
    pub goodbye: String,
}