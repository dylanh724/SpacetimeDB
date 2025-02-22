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
pub struct InsertVecI64Args {
    pub n: Vec<i64>,
}

impl Reducer for InsertVecI64Args {
    const REDUCER_NAME: &'static str = "insert_vec_i64";
}

#[allow(unused)]
pub fn insert_vec_i_64(n: Vec<i64>) {
    InsertVecI64Args { n }.invoke();
}

#[allow(unused)]
pub fn on_insert_vec_i_64(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &Vec<i64>) + Send + 'static,
) -> ReducerCallbackId<InsertVecI64Args> {
    InsertVecI64Args::on_reducer(move |__identity, __addr, __status, __args| {
        let InsertVecI64Args { n } = __args;
        __callback(__identity, __addr, __status, n);
    })
}

#[allow(unused)]
pub fn once_on_insert_vec_i_64(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &Vec<i64>) + Send + 'static,
) -> ReducerCallbackId<InsertVecI64Args> {
    InsertVecI64Args::once_on_reducer(move |__identity, __addr, __status, __args| {
        let InsertVecI64Args { n } = __args;
        __callback(__identity, __addr, __status, n);
    })
}

#[allow(unused)]
pub fn remove_on_insert_vec_i_64(id: ReducerCallbackId<InsertVecI64Args>) {
    InsertVecI64Args::remove_on_reducer(id);
}
