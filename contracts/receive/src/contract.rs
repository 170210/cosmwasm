use cosmwasm_std::{
    entry_point, DepsMut, Env, MessageInfo, Response, StdResult, 
};

use crate::msg::{ExecuteMsg, InstantiateMsg};

// A no-op, just empty data
#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Receive(_receive_msg) =>handle_receive(),
    }
}

fn handle_receive() -> StdResult<Response> {
    Ok(Response::default())
}
