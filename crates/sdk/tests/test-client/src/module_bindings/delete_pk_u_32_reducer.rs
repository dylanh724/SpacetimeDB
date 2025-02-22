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
pub struct DeletePkU32Args {
    pub n: u32,
}

impl Reducer for DeletePkU32Args {
    const REDUCER_NAME: &'static str = "delete_pk_u32";
}

#[allow(unused)]
pub fn delete_pk_u_32(n: u32) {
    DeletePkU32Args { n }.invoke();
}

#[allow(unused)]
pub fn on_delete_pk_u_32(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &u32) + Send + 'static,
) -> ReducerCallbackId<DeletePkU32Args> {
    DeletePkU32Args::on_reducer(move |__identity, __addr, __status, __args| {
        let DeletePkU32Args { n } = __args;
        __callback(__identity, __addr, __status, n);
    })
}

#[allow(unused)]
pub fn once_on_delete_pk_u_32(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &u32) + Send + 'static,
) -> ReducerCallbackId<DeletePkU32Args> {
    DeletePkU32Args::once_on_reducer(move |__identity, __addr, __status, __args| {
        let DeletePkU32Args { n } = __args;
        __callback(__identity, __addr, __status, n);
    })
}

#[allow(unused)]
pub fn remove_on_delete_pk_u_32(id: ReducerCallbackId<DeletePkU32Args>) {
    DeletePkU32Args::remove_on_reducer(id);
}
