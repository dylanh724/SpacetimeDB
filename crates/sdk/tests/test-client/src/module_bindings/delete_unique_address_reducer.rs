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
pub struct DeleteUniqueAddressArgs {
    pub a: Address,
}

impl Reducer for DeleteUniqueAddressArgs {
    const REDUCER_NAME: &'static str = "delete_unique_address";
}

#[allow(unused)]
pub fn delete_unique_address(a: Address) {
    DeleteUniqueAddressArgs { a }.invoke();
}

#[allow(unused)]
pub fn on_delete_unique_address(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &Address) + Send + 'static,
) -> ReducerCallbackId<DeleteUniqueAddressArgs> {
    DeleteUniqueAddressArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let DeleteUniqueAddressArgs { a } = __args;
        __callback(__identity, __addr, __status, a);
    })
}

#[allow(unused)]
pub fn once_on_delete_unique_address(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &Address) + Send + 'static,
) -> ReducerCallbackId<DeleteUniqueAddressArgs> {
    DeleteUniqueAddressArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let DeleteUniqueAddressArgs { a } = __args;
        __callback(__identity, __addr, __status, a);
    })
}

#[allow(unused)]
pub fn remove_on_delete_unique_address(id: ReducerCallbackId<DeleteUniqueAddressArgs>) {
    DeleteUniqueAddressArgs::remove_on_reducer(id);
}
