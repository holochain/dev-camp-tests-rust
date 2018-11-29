#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate holochain_core_types;
#[macro_use]
extern crate holochain_core_types_derive;
extern crate boolinator;

use boolinator::Boolinator;
use hdk::{
    holochain_core_types::{
        dna::zome::entry_types::Sharing,
        hash::HashString,
        json::JsonString,
        entry::Entry,
        entry::entry_type::EntryType,
        error::HolochainError,
    },
};

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
struct Person {
    name: String,
}

fn handle_add_person(name: String) -> JsonString {
    let person_entry = Entry::new(EntryType::App("person".into()), Person {
        name,
    });
    match hdk::commit_entry(&person_entry) {
        Ok(address) => json!({ "address": address }).into(),
        Err(hdk_error) => hdk_error.into(),
    }
}

define_zome! {
    entries: [

    ]

    genesis: || { Ok(()) }

    functions: {

    }
}
