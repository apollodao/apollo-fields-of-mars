use cosmwasm_std::{to_binary, Addr, Api, CosmosMsg, StdResult, WasmMsg};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Partial Apollo Factory ExecuteMsg. Just what we need here.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ApolloFactoryExecuteMsg {
    UpdateUserRewards {
        user: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ApolloFactoryBase<T> {
    pub contract_addr: T,
}

pub type ApolloFactoryUnchecked = ApolloFactoryBase<String>;
pub type ApolloFactory = ApolloFactoryBase<Addr>;

impl From<ApolloFactory> for ApolloFactoryUnchecked {
    fn from(apollo_factory: ApolloFactory) -> Self {
        ApolloFactoryUnchecked {
            contract_addr: apollo_factory.contract_addr.to_string(),
        }
    }
}

impl ApolloFactoryUnchecked {
    pub fn check(&self, api: &dyn Api) -> StdResult<ApolloFactory> {
        Ok(ApolloFactory {
            contract_addr: api.addr_validate(&self.contract_addr)?,
        })
    }
}

impl ApolloFactory {
    /// Create a message for updating the user's apollo rewards
    /// This needs to be done before every deposit and withdrawal (change in shares)
    pub fn update_rewards_msg(&self, user: &Addr) -> StdResult<CosmosMsg> {
        Ok(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: self.contract_addr.to_string(),
            msg: to_binary(&ApolloFactoryExecuteMsg::UpdateUserRewards {
                user: user.to_string(),
            })?,
            funds: vec![],
        }))
    }
}
