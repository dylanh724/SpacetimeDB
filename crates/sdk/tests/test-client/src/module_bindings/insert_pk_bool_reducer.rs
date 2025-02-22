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
pub struct InsertPkBoolArgs {
    pub b: bool,
    pub data: i32,
}

impl Reducer for InsertPkBoolArgs {
    const REDUCER_NAME: &'static str = "insert_pk_bool";
}

#[allow(unused)]
pub fn insert_pk_bool(b: bool, data: i32) {
    InsertPkBoolArgs { b, data }.invoke();
}

#[allow(unused)]
pub fn on_insert_pk_bool(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &bool, &i32) + Send + 'static,
) -> ReducerCallbackId<InsertPkBoolArgs> {
    InsertPkBoolArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let InsertPkBoolArgs { b, data } = __args;
        __callback(__identity, __addr, __status, b, data);
    })
}

#[allow(unused)]
pub fn once_on_insert_pk_bool(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &bool, &i32) + Send + 'static,
) -> ReducerCallbackId<InsertPkBoolArgs> {
    InsertPkBoolArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let InsertPkBoolArgs { b, data } = __args;
        __callback(__identity, __addr, __status, b, data);
    })
}

#[allow(unused)]
pub fn remove_on_insert_pk_bool(id: ReducerCallbackId<InsertPkBoolArgs>) {
    InsertPkBoolArgs::remove_on_reducer(id);
}
