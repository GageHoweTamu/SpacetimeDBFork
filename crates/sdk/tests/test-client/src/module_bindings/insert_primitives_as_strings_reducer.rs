// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused_imports)]
use super::every_primitive_struct::EveryPrimitiveStruct;
use spacetimedb_sdk::{
    anyhow::{anyhow, Result},
    identity::Identity,
    reducer::{Reducer, ReducerCallbackId, Status},
    sats::{de::Deserialize, i256, ser::Serialize, u256},
    spacetimedb_lib,
    table::{TableIter, TableType, TableWithPrimaryKey},
    Address, ScheduleAt,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct InsertPrimitivesAsStringsArgs {
    pub s: EveryPrimitiveStruct,
}

impl Reducer for InsertPrimitivesAsStringsArgs {
    const REDUCER_NAME: &'static str = "insert_primitives_as_strings";
}

#[allow(unused)]
pub fn insert_primitives_as_strings(s: EveryPrimitiveStruct) {
    InsertPrimitivesAsStringsArgs { s }.invoke();
}

#[allow(unused)]
pub fn on_insert_primitives_as_strings(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &EveryPrimitiveStruct) + Send + 'static,
) -> ReducerCallbackId<InsertPrimitivesAsStringsArgs> {
    InsertPrimitivesAsStringsArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let InsertPrimitivesAsStringsArgs { s } = __args;
        __callback(__identity, __addr, __status, s);
    })
}

#[allow(unused)]
pub fn once_on_insert_primitives_as_strings(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &EveryPrimitiveStruct) + Send + 'static,
) -> ReducerCallbackId<InsertPrimitivesAsStringsArgs> {
    InsertPrimitivesAsStringsArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let InsertPrimitivesAsStringsArgs { s } = __args;
        __callback(__identity, __addr, __status, s);
    })
}

#[allow(unused)]
pub fn remove_on_insert_primitives_as_strings(id: ReducerCallbackId<InsertPrimitivesAsStringsArgs>) {
    InsertPrimitivesAsStringsArgs::remove_on_reducer(id);
}
