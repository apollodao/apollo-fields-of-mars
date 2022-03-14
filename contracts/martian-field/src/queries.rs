use cosmwasm_std::{Deps, Env, StdResult, Uint128};

use fields_of_mars::martian_field::msg::QueryMsg;
use fields_of_mars::martian_field::{
    AprResponse, ConfigUnchecked, Health, PositionUnchecked, Snapshot, State, StrategyInfoResponse,
    TvlResponse, UserInfoResponse,
};

use crate::health::compute_health;
use crate::state::{CONFIG, POSITION, SNAPSHOT, STATE};

#[allow(clippy::all)]
mod uints {
    uint::construct_uint! {
        pub struct U256(4);
    }
}
use uints::U256;

pub fn query_config(deps: Deps, _env: Env) -> StdResult<ConfigUnchecked> {
    let config = CONFIG.load(deps.storage)?;
    Ok(config.into())
}

pub fn query_state(deps: Deps, _env: Env) -> StdResult<State> {
    STATE.load(deps.storage)
}

pub fn query_position(deps: Deps, _env: Env, user: String) -> StdResult<PositionUnchecked> {
    let user_addr = deps.api.addr_validate(&user)?;
    let position = POSITION.load(deps.storage, &user_addr).unwrap_or_default();
    Ok(position.into())
}

pub fn query_health(deps: Deps, env: Env, user: String) -> StdResult<Health> {
    let user_addr = deps.api.addr_validate(&user)?;
    let config = CONFIG.load(deps.storage)?;
    let state = STATE.load(deps.storage)?;
    let position = POSITION.load(deps.storage, &user_addr).unwrap_or_default();
    compute_health(&deps.querier, &env, &config, &state, &position)
}

pub fn query_snapshot(deps: Deps, user: String) -> StdResult<Snapshot> {
    let user_addr = deps.api.addr_validate(&user)?;
    Ok(SNAPSHOT.load(deps.storage, &user_addr).unwrap_or_default())
}

pub fn query_user_info(deps: Deps, env: Env, user: String) -> StdResult<UserInfoResponse> {
    let user_addr = deps.api.addr_validate(&user)?;
    let position = POSITION.load(deps.storage, &user_addr).unwrap_or_default();
    let config = CONFIG.load(deps.storage)?;
    let state = STATE.load(deps.storage)?;

    let total_bonded_amount = config.astro_generator.query_bonded_amount(
        &deps.querier,
        &env.contract.address,
        &config.primary_pair.liquidity_token,
    )?;

    let user_bonds =
        total_bonded_amount.multiply_ratio(position.bond_units, state.total_bond_units);

    Ok(UserInfoResponse {
        shares: position.bond_units,
        base_token_balance: user_bonds,
    })
}

pub fn query_strategy_info(deps: Deps, env: Env) -> StdResult<StrategyInfoResponse> {
    let config = CONFIG.load(deps.storage)?;
    let state = STATE.load(deps.storage)?;

    let total_bonded_amount = config.astro_generator.query_bonded_amount(
        &deps.querier,
        &env.contract.address,
        &config.primary_pair.liquidity_token,
    )?;

    Ok(StrategyInfoResponse {
        total_bond_amount: total_bonded_amount,
        total_shares: state.total_bond_units,
    })
}

pub fn query_tvl(deps: Deps, env: Env) -> StdResult<TvlResponse> {
    let config = CONFIG.load(deps.storage)?;

    let total_bonded_amount = config.astro_generator.query_bonded_amount(
        &deps.querier,
        &env.contract.address,
        &config.primary_pair.liquidity_token,
    )?;

    let (primary_depth, secondary_depth, total_shares) = config.primary_pair.query_pool(
        &deps.querier,
        &config.primary_asset_info,
        &config.secondary_asset_info,
    )?;

    let primary_price = config.oracle.query_price(&deps.querier, &config.primary_asset_info)?;
    let secondary_price = config.oracle.query_price(&deps.querier, &config.secondary_asset_info)?;

    // RE the calculation of the value of liquidity token, see:
    // https://blog.alphafinance.io/fair-lp-token-pricing/
    // this formulation avoids a potential sandwich attack that distorts asset prices by a flashloan
    //
    // NOTE: we need to use U256 here, because Uint128 * Uint128 may overflow the 128-bit limit
    let primary_value = U256::from(u128::from(primary_depth * primary_price));
    let secondary_value = U256::from(u128::from(secondary_depth * secondary_price));
    let pool_value = U256::from(2) * (primary_value * secondary_value).integer_sqrt();

    let pool_value_u128 = Uint128::new(pool_value.as_u128());

    let total_bonded_value = if total_shares.is_zero() {
        Uint128::zero()
    } else {
        pool_value_u128.multiply_ratio(total_bonded_amount, total_shares)
    };

    Ok(TvlResponse {
        tvl: total_bonded_value,
    })
}

pub fn query_apr(deps: Deps) -> StdResult<AprResponse> {
    let config = CONFIG.load(deps.storage)?;

    deps.querier.query_wasm_smart(config.apr_query_adapter, &QueryMsg::Apr {})
}
