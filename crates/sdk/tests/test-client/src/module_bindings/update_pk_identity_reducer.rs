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
pub struct UpdatePkIdentityArgs {
    pub i: Identity,
    pub data: i32,
}

impl Reducer for UpdatePkIdentityArgs {
    const REDUCER_NAME: &'static str = "update_pk_identity";
}

#[allow(unused)]
pub fn update_pk_identity(i: Identity, data: i32) {
    UpdatePkIdentityArgs { i, data }.invoke();
}

#[allow(unused)]
pub fn on_update_pk_identity(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &Identity, &i32) + Send + 'static,
) -> ReducerCallbackId<UpdatePkIdentityArgs> {
    UpdatePkIdentityArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let UpdatePkIdentityArgs { i, data } = __args;
        __callback(__identity, __addr, __status, i, data);
    })
}

#[allow(unused)]
pub fn once_on_update_pk_identity(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &Identity, &i32) + Send + 'static,
) -> ReducerCallbackId<UpdatePkIdentityArgs> {
    UpdatePkIdentityArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let UpdatePkIdentityArgs { i, data } = __args;
        __callback(__identity, __addr, __status, i, data);
    })
}

#[allow(unused)]
pub fn remove_on_update_pk_identity(id: ReducerCallbackId<UpdatePkIdentityArgs>) {
    UpdatePkIdentityArgs::remove_on_reducer(id);
}
