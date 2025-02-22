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
pub struct SetNameArgs {
    pub name: String,
}

impl Reducer for SetNameArgs {
    const REDUCER_NAME: &'static str = "set_name";
}

#[allow(unused)]
pub fn set_name(name: String) {
    SetNameArgs { name }.invoke();
}

#[allow(unused)]
pub fn on_set_name(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &String) + Send + 'static,
) -> ReducerCallbackId<SetNameArgs> {
    SetNameArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let SetNameArgs { name } = __args;
        __callback(__identity, __addr, __status, name);
    })
}

#[allow(unused)]
pub fn once_on_set_name(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &String) + Send + 'static,
) -> ReducerCallbackId<SetNameArgs> {
    SetNameArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let SetNameArgs { name } = __args;
        __callback(__identity, __addr, __status, name);
    })
}

#[allow(unused)]
pub fn remove_on_set_name(id: ReducerCallbackId<SetNameArgs>) {
    SetNameArgs::remove_on_reducer(id);
}
