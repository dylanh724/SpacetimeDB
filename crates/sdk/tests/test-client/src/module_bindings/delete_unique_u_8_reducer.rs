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
pub struct DeleteUniqueU8Args {
    pub n: u8,
}

impl Reducer for DeleteUniqueU8Args {
    const REDUCER_NAME: &'static str = "delete_unique_u8";
}

#[allow(unused)]
pub fn delete_unique_u_8(n: u8) {
    DeleteUniqueU8Args { n }.invoke();
}

#[allow(unused)]
pub fn on_delete_unique_u_8(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &u8) + Send + 'static,
) -> ReducerCallbackId<DeleteUniqueU8Args> {
    DeleteUniqueU8Args::on_reducer(move |__identity, __addr, __status, __args| {
        let DeleteUniqueU8Args { n } = __args;
        __callback(__identity, __addr, __status, n);
    })
}

#[allow(unused)]
pub fn once_on_delete_unique_u_8(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &u8) + Send + 'static,
) -> ReducerCallbackId<DeleteUniqueU8Args> {
    DeleteUniqueU8Args::once_on_reducer(move |__identity, __addr, __status, __args| {
        let DeleteUniqueU8Args { n } = __args;
        __callback(__identity, __addr, __status, n);
    })
}

#[allow(unused)]
pub fn remove_on_delete_unique_u_8(id: ReducerCallbackId<DeleteUniqueU8Args>) {
    DeleteUniqueU8Args::remove_on_reducer(id);
}
