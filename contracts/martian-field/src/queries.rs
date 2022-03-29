use cosmwasm_std::{Deps, Env, StdResult};

use fields_of_mars::martian_field::msg::QueryMsg;
use fields_of_mars::martian_field::{
    AprResponse, ConfigUnchecked, Health, PositionUnchecked, Snapshot, State, StrategyInfoResponse,
    TvlResponse, UserInfoResponse,
};

use crate::health::{compute_health, compute_value_per_lp_token};
use crate::state::{CONFIG, POSITION, SNAPSHOT, STATE};

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

    let lp_value = compute_value_per_lp_token(&deps.querier, &config, None, None)?;
    let total_bonded_value = total_bonded_amount * lp_value;

    Ok(TvlResponse {
        tvl: total_bonded_value,
    })
}

pub fn query_apr(deps: Deps) -> StdResult<AprResponse> {
    let config = CONFIG.load(deps.storage)?;

    deps.querier.query_wasm_smart(config.apr_query_adapter, &QueryMsg::Apr {})
}
