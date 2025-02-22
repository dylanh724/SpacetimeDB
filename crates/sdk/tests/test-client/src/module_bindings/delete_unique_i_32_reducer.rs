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
pub struct DeleteUniqueI32Args {
    pub n: i32,
}

impl Reducer for DeleteUniqueI32Args {
    const REDUCER_NAME: &'static str = "delete_unique_i32";
}

#[allow(unused)]
pub fn delete_unique_i_32(n: i32) {
    DeleteUniqueI32Args { n }.invoke();
}

#[allow(unused)]
pub fn on_delete_unique_i_32(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &i32) + Send + 'static,
) -> ReducerCallbackId<DeleteUniqueI32Args> {
    DeleteUniqueI32Args::on_reducer(move |__identity, __addr, __status, __args| {
        let DeleteUniqueI32Args { n } = __args;
        __callback(__identity, __addr, __status, n);
    })
}

#[allow(unused)]
pub fn once_on_delete_unique_i_32(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &i32) + Send + 'static,
) -> ReducerCallbackId<DeleteUniqueI32Args> {
    DeleteUniqueI32Args::once_on_reducer(move |__identity, __addr, __status, __args| {
        let DeleteUniqueI32Args { n } = __args;
        __callback(__identity, __addr, __status, n);
    })
}

#[allow(unused)]
pub fn remove_on_delete_unique_i_32(id: ReducerCallbackId<DeleteUniqueI32Args>) {
    DeleteUniqueI32Args::remove_on_reducer(id);
}
