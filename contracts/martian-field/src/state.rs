use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

use fields_of_mars::martian_field::{Config, Position, Snapshot, State};

pub const CONFIG: Item<Config> = Item::new("config");
pub const STATE: Item<State> = Item::new("state");
pub const POSITION: Map<&Addr, Position> = Map::new("position");

pub const CACHED_USER_ADDR: Item<Addr> = Item::new("cached_user_addr");
pub const CACHED_ASSET_BALANCE: Item<Uint128> = Item::new("cached_asset_balance");

// TODO: delete this
pub const SNAPSHOT: Map<&Addr, Snapshot> = Map::new("snapshot");
