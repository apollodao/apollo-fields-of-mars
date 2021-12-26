//--------------------------------------------------------------------------------------------------
// Astroport pair types
//--------------------------------------------------------------------------------------------------

export type AssetInfo = { cw20: string } | { native: string };

export type Asset = {
  info?: AssetInfo;
  amount: string;
};

export type PoolResponse = {
  assets: Asset[];
  total_share: string;
};

export type SimulationResponse = {
  return_amount: string;
  spread_amount: string;
  commission_amount: string;
};

//--------------------------------------------------------------------------------------------------
// Astro generator types
//--------------------------------------------------------------------------------------------------

export type PendingTokenResponse = {
  pending: string;
  pending_on_proxy?: string;
};

//--------------------------------------------------------------------------------------------------
// Red Bank types
//--------------------------------------------------------------------------------------------------

export type UserAssetDebtResponse = {
  amount: string;
  // amount is the only parameter we care about. set others to optional
  denom?: string;
  asset_label?: string;
  asset_reference?: number[];
  asset_type?: { native: {} } | { cw20: {} };
  amount_scaled?: string;
};

//--------------------------------------------------------------------------------------------------
// Martian Field types
//--------------------------------------------------------------------------------------------------

export type Config = {
  red_bank: {
    contract_addr: string;
  };
  pair: {
    contract_addr: string;
    liquidity_token: string;
  };
  staking: {
    [key: string]: {
      contract_addr: string;
      asset_token: string;
      staking_token: string;
    };
  };
  [key: string]: string | object;
};

export type StateResponse = {
  total_bond_units: string;
  total_debt_units: string;
  pending_rewards: Asset[];
};

export type PositionResponse = {
  bond_units: string;
  debt_units: string;
  unlocked_assets: Asset[];
};

export type HealthResponse = {
  bond_value: string;
  debt_value: string;
  ltv: string | null;
};
