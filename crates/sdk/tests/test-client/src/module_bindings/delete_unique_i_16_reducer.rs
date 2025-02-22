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
pub struct DeleteUniqueI16Args {
    pub n: i16,
}

impl Reducer for DeleteUniqueI16Args {
    const REDUCER_NAME: &'static str = "delete_unique_i16";
}

#[allow(unused)]
pub fn delete_unique_i_16(n: i16) {
    DeleteUniqueI16Args { n }.invoke();
}

#[allow(unused)]
pub fn on_delete_unique_i_16(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &i16) + Send + 'static,
) -> ReducerCallbackId<DeleteUniqueI16Args> {
    DeleteUniqueI16Args::on_reducer(move |__identity, __addr, __status, __args| {
        let DeleteUniqueI16Args { n } = __args;
        __callback(__identity, __addr, __status, n);
    })
}

#[allow(unused)]
pub fn once_on_delete_unique_i_16(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &i16) + Send + 'static,
) -> ReducerCallbackId<DeleteUniqueI16Args> {
    DeleteUniqueI16Args::once_on_reducer(move |__identity, __addr, __status, __args| {
        let DeleteUniqueI16Args { n } = __args;
        __callback(__identity, __addr, __status, n);
    })
}

#[allow(unused)]
pub fn remove_on_delete_unique_i_16(id: ReducerCallbackId<DeleteUniqueI16Args>) {
    DeleteUniqueI16Args::remove_on_reducer(id);
}
