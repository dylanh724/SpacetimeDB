// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

use super::one_u_8::OneU8;
use super::vec_u_8::VecU8;
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
pub struct InsertTableHoldsTableArgs {
    pub a: OneU8,
    pub b: VecU8,
}

impl Reducer for InsertTableHoldsTableArgs {
    const REDUCER_NAME: &'static str = "insert_table_holds_table";
}

#[allow(unused)]
pub fn insert_table_holds_table(a: OneU8, b: VecU8) {
    InsertTableHoldsTableArgs { a, b }.invoke();
}

#[allow(unused)]
pub fn on_insert_table_holds_table(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &OneU8, &VecU8) + Send + 'static,
) -> ReducerCallbackId<InsertTableHoldsTableArgs> {
    InsertTableHoldsTableArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let InsertTableHoldsTableArgs { a, b } = __args;
        __callback(__identity, __addr, __status, a, b);
    })
}

#[allow(unused)]
pub fn once_on_insert_table_holds_table(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &OneU8, &VecU8) + Send + 'static,
) -> ReducerCallbackId<InsertTableHoldsTableArgs> {
    InsertTableHoldsTableArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let InsertTableHoldsTableArgs { a, b } = __args;
        __callback(__identity, __addr, __status, a, b);
    })
}

#[allow(unused)]
pub fn remove_on_insert_table_holds_table(id: ReducerCallbackId<InsertTableHoldsTableArgs>) {
    InsertTableHoldsTableArgs::remove_on_reducer(id);
}
