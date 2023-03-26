use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;
// use serde::{Serialize, Deserialize};
// use schemars::JsonSchema;



pub const COIN_CONTRACT: Item<Addr> = Item::new("coin_contract");
pub const ADMINS: Item<Vec<Addr>> = Item::new("users");
pub const BALANCE: Item<Uint128> = Item::new("total_funds");
