#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;
use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_wasm_utils::api_serialization::{
        get_links::GetLinksResult,
    },
};
use hdk::holochain_core_types::{
    cas::content::Address, entry::Entry, dna::entry_types::Sharing, error::HolochainError, json::JsonString,
};

define_zome! {
    entries: []

    genesis: || { Ok(()) }

    functions: []

    capabilities: {}
}
