use fields_of_mars::martian_field::msg::{
    Action, CallbackMsg, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg,
};
use fields_of_mars::martian_field::{
    AprResponse, ConfigUnchecked, Health, PositionUnchecked, Snapshot, State, StrategyInfoResponse,
    TvlResponse, UserInfoResponse,
};
use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(MigrateMsg), &out_dir);
    export_schema(&schema_for!(CallbackMsg), &out_dir);
    export_schema(&schema_for!(Action), &out_dir);
    export_schema(&schema_for!(ConfigUnchecked), &out_dir);
    export_schema(&schema_for!(Health), &out_dir);
    export_schema(&schema_for!(PositionUnchecked), &out_dir);
    export_schema(&schema_for!(Snapshot), &out_dir);
    export_schema(&schema_for!(State), &out_dir);
    export_schema(&schema_for!(StrategyInfoResponse), &out_dir);
    export_schema(&schema_for!(UserInfoResponse), &out_dir);
    export_schema(&schema_for!(AprResponse), &out_dir);
    export_schema(&schema_for!(TvlResponse), &out_dir);
}
