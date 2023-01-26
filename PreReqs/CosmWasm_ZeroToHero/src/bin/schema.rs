use std::env::current_dir;

use cosmwasm_schema::{export_schema, schema_for, write_api};

use cosm_wasm_zero2_hero::config::{Ballot, Config, Poll};
use cosm_wasm_zero2_hero::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    // Define the path where the schema will be saved
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");

    // Export schema for messages
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }

    // Export schema for config (aka state)
    export_schema(&schema_for!(Config), &out_dir);
    export_schema(&schema_for!(Ballot), &out_dir);
    export_schema(&schema_for!(Poll), &out_dir);
}
