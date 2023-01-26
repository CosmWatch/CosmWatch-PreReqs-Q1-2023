#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::config::{Config, Poll, CONFIG, POLLS};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cosm-wasm-zero2-hero";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let admin = msg.admin.unwrap_or(info.sender.to_string());
    let validated_admin = deps.api.addr_validate(&admin)?;
    let config = Config {
        admin: validated_admin.clone(),
    };
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", validated_admin.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreatePoll {
            poll_id,
            question,
            options,
        } => execute_create_poll(deps, env, info, poll_id, question, options),
        ExecuteMsg::DeletePoll { poll_id } => unimplemented!(),
        ExecuteMsg::Vote { poll_id, vote } => unimplemented!(),
        ExecuteMsg::DeleteVote { poll_id } => unimplemented!(),
    }
}

fn execute_create_poll(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    poll_id: String,
    question: String,
    options: Vec<String>,
) -> Result<Response, ContractError> {
    // Ensure there are no more than 10 options
    if options.len() > 10 {
        return Err(ContractError::TooManyOptions {});
    }

    // Loop over options and add to options vector
    let mut opts: Vec<(String, u64)> = vec![];
    for opt in options {
        opts.push((opt, 0));
    }

    // Create poll and save it to config (aka state)
    let poll = Poll {
        creator: info.sender,
        question,
        options: opts,
    };
    POLLS.save(deps.storage, poll_id, &poll)?;

    Ok(Response::new()
        .add_attribute("action", "create_poll")
        .add_attribute("creator", &poll.creator)
        .add_attribute("question", &poll.question)
        .add_attribute(
            "options",
            &poll
                .options
                .iter()
                .map(|&(ref s, _)| s.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use crate::contract::instantiate;
    use crate::msg::InstantiateMsg;
    use cosmwasm_std::attr;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    pub const ADDR1: &str = "addr1";
    pub const ADDR2: &str = "addr2";

    #[test]
    fn test_instantiate() {
        // Define mock dependencies, env, and info
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);

        // Define message to instantiate contract and call instantiate
        let msg = InstantiateMsg { admin: None };
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();

        // Check response for success
        assert_eq!(
            res.attributes,
            vec![attr("action", "instantiate"), attr("admin", ADDR1)]
        )
    }

    #[test]
    fn test_instantiate_with_admin() {
        // Define mock dependencies, env, and info
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);

        // Define message to instantiate contract (with admin this time) and call instantiate
        let msg = InstantiateMsg {
            admin: Some(ADDR2.to_string()),
        };
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();

        // Check response for success
        assert_eq!(
            res.attributes,
            vec![attr("action", "instantiate"), attr("admin", ADDR2)]
        )
    }
}
