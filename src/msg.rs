use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum QueryMsg {
    Greet(String),
    Goodbye(String),
}



#[derive(Serialize, Deserialize)]
pub struct GreetResp {
    greeting: String,
}



#[derive(Serialize, Deserialize)]
pub struct GoodbyeResp {
    goodbye: String,
}