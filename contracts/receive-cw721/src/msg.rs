use cosmwasm_schema::cw_serde;
use cw721::Cw721ReceiveMsg;

#[cw_serde]
pub enum ExecuteMsg {
    Receive(Cw721ReceiveMsg),
}

#[cw_serde]
pub struct InstantiateMsg {}
