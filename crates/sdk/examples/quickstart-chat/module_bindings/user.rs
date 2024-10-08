// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused_imports)]
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
pub struct User {
    pub identity: Identity,
    pub name: Option<String>,
    pub online: bool,
}

impl TableType for User {
    const TABLE_NAME: &'static str = "user";
    type ReducerEvent = super::ReducerEvent;
}

impl TableWithPrimaryKey for User {
    type PrimaryKey = Identity;
    fn primary_key(&self) -> &Self::PrimaryKey {
        &self.identity
    }
}

impl User {
    #[allow(unused)]
    pub fn filter_by_identity(identity: Identity) -> TableIter<Self> {
        Self::filter(|row| row.identity == identity)
    }
    #[allow(unused)]
    pub fn find_by_identity(identity: Identity) -> Option<Self> {
        Self::find(|row| row.identity == identity)
    }
    #[allow(unused)]
    pub fn filter_by_online(online: bool) -> TableIter<Self> {
        Self::filter(|row| row.online == online)
    }
}
