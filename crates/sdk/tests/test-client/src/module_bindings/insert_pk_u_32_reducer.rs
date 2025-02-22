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
pub struct InsertPkU32Args {
    pub n: u32,
    pub data: i32,
}

impl Reducer for InsertPkU32Args {
    const REDUCER_NAME: &'static str = "insert_pk_u32";
}

#[allow(unused)]
pub fn insert_pk_u_32(n: u32, data: i32) {
    InsertPkU32Args { n, data }.invoke();
}

#[allow(unused)]
pub fn on_insert_pk_u_32(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &u32, &i32) + Send + 'static,
) -> ReducerCallbackId<InsertPkU32Args> {
    InsertPkU32Args::on_reducer(move |__identity, __addr, __status, __args| {
        let InsertPkU32Args { n, data } = __args;
        __callback(__identity, __addr, __status, n, data);
    })
}

#[allow(unused)]
pub fn once_on_insert_pk_u_32(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &u32, &i32) + Send + 'static,
) -> ReducerCallbackId<InsertPkU32Args> {
    InsertPkU32Args::once_on_reducer(move |__identity, __addr, __status, __args| {
        let InsertPkU32Args { n, data } = __args;
        __callback(__identity, __addr, __status, n, data);
    })
}

#[allow(unused)]
pub fn remove_on_insert_pk_u_32(id: ReducerCallbackId<InsertPkU32Args>) {
    InsertPkU32Args::remove_on_reducer(id);
}
