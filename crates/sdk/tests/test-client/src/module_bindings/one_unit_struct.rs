// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

use super::unit_struct::UnitStruct;
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
pub struct OneUnitStruct {
    pub s: UnitStruct,
}

impl TableType for OneUnitStruct {
    const TABLE_NAME: &'static str = "OneUnitStruct";
    type ReducerEvent = super::ReducerEvent;
}

impl OneUnitStruct {
    #[allow(unused)]
    pub fn filter_by_s(s: UnitStruct) -> TableIter<Self> {
        Self::filter(|row| row.s == s)
    }
}
