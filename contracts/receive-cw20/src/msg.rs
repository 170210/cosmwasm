use cosmwasm_schema::{cw_serde};
use cw20::Cw20ReceiveMsg;

#[cw_serde]
pub enum ExecuteMsg {
    Receive(Cw20ReceiveMsg),
}

#[cw_serde]
pub struct InstantiateMsg {}
