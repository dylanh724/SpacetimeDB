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
pub struct InsertVecI8Args {
    pub n: Vec<i8>,
}

impl Reducer for InsertVecI8Args {
    const REDUCER_NAME: &'static str = "insert_vec_i8";
}

#[allow(unused)]
pub fn insert_vec_i_8(n: Vec<i8>) {
    InsertVecI8Args { n }.invoke();
}

#[allow(unused)]
pub fn on_insert_vec_i_8(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &Vec<i8>) + Send + 'static,
) -> ReducerCallbackId<InsertVecI8Args> {
    InsertVecI8Args::on_reducer(move |__identity, __addr, __status, __args| {
        let InsertVecI8Args { n } = __args;
        __callback(__identity, __addr, __status, n);
    })
}

#[allow(unused)]
pub fn once_on_insert_vec_i_8(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &Vec<i8>) + Send + 'static,
) -> ReducerCallbackId<InsertVecI8Args> {
    InsertVecI8Args::once_on_reducer(move |__identity, __addr, __status, __args| {
        let InsertVecI8Args { n } = __args;
        __callback(__identity, __addr, __status, n);
    })
}

#[allow(unused)]
pub fn remove_on_insert_vec_i_8(id: ReducerCallbackId<InsertVecI8Args>) {
    InsertVecI8Args::remove_on_reducer(id);
}
