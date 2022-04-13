use apollo_factory::msg::MigrateMsg;
use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use apollo_factory::msg::InstantiateMsg;
use apollo_protocol::factory::{
    Cw20HookMsg, ExecuteMsg, FactoryConfig as Config, FactoryStrategyInfoResponse,
    GetConfigResponse, GetExtensionTotalCollectedFeesResponse, GetStrategiesResponse,
    GetTotalCollectedFeesResponse, GetTotalRewardWeightResponse, GetTvlResponse,
    GetUserStrategiesResponse, QueryMsg, StakerInfoResponse,
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(MigrateMsg), &out_dir);
    export_schema(&schema_for!(Config), &out_dir);
    export_schema(&schema_for!(FactoryStrategyInfoResponse), &out_dir);
    export_schema(&schema_for!(GetStrategiesResponse), &out_dir);
    export_schema(&schema_for!(GetUserStrategiesResponse), &out_dir);
    export_schema(&schema_for!(GetConfigResponse), &out_dir);
    export_schema(&schema_for!(GetTvlResponse), &out_dir);
    export_schema(&schema_for!(Cw20HookMsg), &out_dir);
    export_schema(&schema_for!(GetTotalCollectedFeesResponse), &out_dir);
    export_schema(
        &schema_for!(GetExtensionTotalCollectedFeesResponse),
        &out_dir,
    );
    export_schema(&schema_for!(GetTotalRewardWeightResponse), &out_dir);
    export_schema(&schema_for!(StakerInfoResponse), &out_dir);
}
