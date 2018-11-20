#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate holochain_core_types;
#[macro_use]
extern crate holochain_core_types_derive;
extern crate boolinator;

use boolinator::Boolinator;
use hdk::{
    error::ZomeApiError,
    holochain_wasm_utils::api_serialization::get_entry::{
        GetEntryOptions,
    },
    holochain_core_types::{
        dna::zome::entry_types::Sharing,
        hash::HashString,
        json::JsonString,
        entry::Entry,
        entry::entry_type::EntryType,
        error::HolochainError,
    },
    AGENT_ADDRESS,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
struct Person {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
struct AddPersonResult {
    address: HashString,
}

fn handle_add_person(name: String) -> JsonString {
    let person_entry = Entry::new(EntryType::App("person".into()), Person {
        name,
    });
    match hdk::commit_entry(&person_entry) {
        Ok(address) => AddPersonResult { address: address }.into(),
        Err(hdk_error) => hdk_error.into(),
    }
}

define_zome! {
    entries: [
        entry!(
            name: "person",
            description: "",
            sharing: Sharing::Public,
            native_type: Person,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |person: Person, _ctx: hdk::ValidationData| {
                (person.name.len() >= 2)
                    .ok_or_else(|| String::from("Name must be at least 2 characters"))
            }
        )
    ]

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
            add_person: {
                inputs: |name: String|,
                outputs: |result: serde_json::Value|,
                handler: handle_add_person
            }
        }
    }
}
