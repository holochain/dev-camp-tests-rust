// import external Rust crates
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

// create references for type definitions, for simpler use of externally
// defined types within the file (Address, instead of hdk::holochain_core_types::cas::content::Address)
use boolinator::Boolinator;
use hdk::{
    error::ZomeApiResult,
    holochain_core_types::{
        cas::content::Address, dna::entry_types::Sharing, entry::Entry, error::HolochainError,
        json::JsonString,
        validation::EntryValidationData,
    },
};
use holochain_wasm_utils::api_serialization::get_links::GetLinksResult;

define_zome! {
    entries: []

    genesis: || { Ok(()) }

    functions: []

    traits: {
        hc_public []
    }
}
