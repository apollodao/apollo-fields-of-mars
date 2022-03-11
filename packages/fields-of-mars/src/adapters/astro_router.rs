use cosmwasm_std::{to_binary, Addr, Api, Coin, CosmosMsg, StdResult, Uint128, WasmMsg};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use astroport::router::{Cw20HookMsg, ExecuteMsg, SwapOperation};

use cw_asset::{Asset, AssetInfo};
use std::convert::TryFrom;

//--------------------------------------------------------------------------------------------------
// Router
//--------------------------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RouterBase<T> {
    /// Address of the Astroport router contract
    pub contract_addr: T,
}

pub type RouterUnchecked = RouterBase<String>;
pub type Router = RouterBase<Addr>;

impl From<Router> for RouterUnchecked {
    fn from(router: Router) -> Self {
        RouterUnchecked {
            contract_addr: router.contract_addr.to_string(),
        }
    }
}

impl RouterUnchecked {
    pub fn check(&self, api: &dyn Api) -> StdResult<Router> {
        Ok(Router {
            contract_addr: api.addr_validate(&self.contract_addr)?,
        })
    }
}

impl Router {
    /// Create a new Router instance
    pub fn new(contract_addr: &Addr) -> Self {
        Self {
            contract_addr: contract_addr.clone(),
        }
    }

    /// Generate msg for swapping start_asset through the assets in the assets vec.
    /// start_asset will be swapped to the last asset in the assets vec.
    /// Note that start_asset should be included in the assets vec.
    pub fn swap_msg(
        &self,
        start_asset: Asset,
        assets: Vec<AssetInfo>,
        minimum_receive: Option<Uint128>,
        to: Option<Addr>,
    ) -> StdResult<CosmosMsg> {
        let operations = assets
            .windows(2)
            .flat_map(<&[AssetInfo; 2]>::try_from)
            .map(|[a, b]| SwapOperation::AstroSwap {
                offer_asset_info: a.into(),
                ask_asset_info: b.into(),
            })
            .collect();

        let msg = match &start_asset.info {
            AssetInfo::Cw20(_) => start_asset.send_msg(
                &self.contract_addr,
                to_binary(&Cw20HookMsg::ExecuteSwapOperations {
                    operations,
                    minimum_receive,
                    to: to.map(|x| x.into_string()),
                })?,
            )?,
            AssetInfo::Native(denom) => CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: self.contract_addr.to_string(),
                msg: to_binary(&ExecuteMsg::ExecuteSwapOperations {
                    operations,
                    minimum_receive,
                    to,
                })?,
                funds: vec![Coin {
                    denom: denom.clone(),
                    amount: start_asset.amount,
                }],
            }),
        };
        Ok(msg)
    }
}
