use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub const USERS: Item<Vec<Addr>> = Item::new("users");
pub const FUNDS: Item<String> = Item::new("total_funds");