use cosmwasm_std::{Addr, Uint128, Storage, StdResult};
use cosmwasm_storage::{ReadonlySingleton, Singleton};
use serde::{Serialize, Deserialize};

const KEY_CONFIG: &[u8] = b"config";
const KEY_STATE: &[u8] = b"state";



#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct State {
    pub admins: Vec<Addr>,
    pub balance: Uint128,
}



#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Config {
    pub token_contract: Addr,
}



pub fn store_config(storage: &mut dyn Storage, data: &Config) -> StdResult<()> {
    Singleton::new(storage, KEY_CONFIG).save(data)
}



pub fn read_config(storage: &dyn Storage) -> StdResult<Config> {
    ReadonlySingleton::new(storage, KEY_CONFIG).load()
}



pub fn store_state(storage: &mut dyn Storage, data: &State) -> StdResult<()> {
    Singleton::new(storage, KEY_STATE).save(data)
}



pub fn read_state(storage: &dyn Storage) -> StdResult<State> {
    ReadonlySingleton::new(storage, KEY_STATE).load()
}