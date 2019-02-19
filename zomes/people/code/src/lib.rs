#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;
extern crate boolinator;
use boolinator::Boolinator;
use hdk::{
    holochain_core_types::{
        dna::entry_types::Sharing,
        json::JsonString,
        entry::Entry,
        error::HolochainError,
        cas::content::Address,
    },
    error::ZomeApiResult,
};
use holochain_wasm_utils::api_serialization::get_links::GetLinksResult;

define_zome! {
    entries: []

    genesis: || { Ok(()) }

    functions: []

    traits: {}
}
