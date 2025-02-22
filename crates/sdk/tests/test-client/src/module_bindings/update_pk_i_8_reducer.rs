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
pub struct UpdatePkI8Args {
    pub n: i8,
    pub data: i32,
}

impl Reducer for UpdatePkI8Args {
    const REDUCER_NAME: &'static str = "update_pk_i8";
}

#[allow(unused)]
pub fn update_pk_i_8(n: i8, data: i32) {
    UpdatePkI8Args { n, data }.invoke();
}

#[allow(unused)]
pub fn on_update_pk_i_8(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &i8, &i32) + Send + 'static,
) -> ReducerCallbackId<UpdatePkI8Args> {
    UpdatePkI8Args::on_reducer(move |__identity, __addr, __status, __args| {
        let UpdatePkI8Args { n, data } = __args;
        __callback(__identity, __addr, __status, n, data);
    })
}

#[allow(unused)]
pub fn once_on_update_pk_i_8(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &i8, &i32) + Send + 'static,
) -> ReducerCallbackId<UpdatePkI8Args> {
    UpdatePkI8Args::once_on_reducer(move |__identity, __addr, __status, __args| {
        let UpdatePkI8Args { n, data } = __args;
        __callback(__identity, __addr, __status, n, data);
    })
}

#[allow(unused)]
pub fn remove_on_update_pk_i_8(id: ReducerCallbackId<UpdatePkI8Args>) {
    UpdatePkI8Args::remove_on_reducer(id);
}
