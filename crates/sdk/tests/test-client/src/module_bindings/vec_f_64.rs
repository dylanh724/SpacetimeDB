// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

use spacetimedb_sdk::{
    anyhow::{anyhow, Result},
    identity::Identity,
    reducer::{Reducer, ReducerCallbackId, Status},
    sats::{de::Deserialize, ser::Serialize},
    spacetimedb_lib,
    table::{TableIter, TableType, TableWithPrimaryKey},
    Address,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct VecF64 {
    pub f: Vec<f64>,
}

impl TableType for VecF64 {
    const TABLE_NAME: &'static str = "VecF64";
    type ReducerEvent = super::ReducerEvent;
}

impl VecF64 {
    #[allow(unused)]
    pub fn filter_by_f(f: Vec<f64>) -> TableIter<Self> {
        Self::filter(|row| row.f == f)
    }
}
