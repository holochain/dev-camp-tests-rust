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
        cas::content::Address,
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

fn handle_get_person(address: HashString) -> JsonString {
    match hdk::get_entry(address) {
        Ok(result) => result.and_then(|entry| Some(entry.serialize())).into(),
        Err(hdk_error) => hdk_error.into(),
    }
}

fn handle_link_people(base: HashString, target: HashString, tag: String) -> JsonString {
    match hdk::link_entries(
        &base,
        &target,
        tag
    ) {
        Ok(_) => json!({"success": true}).into(),
        Err(hdk_error) => hdk_error.into(),
    }
}

fn handle_get_relationships(address: HashString, tag: String) -> JsonString {
    match hdk::get_links(&address, tag) {
        Ok(result) => json!({"addresses": result.addresses().clone()}).into(),
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
            },
            links: [
                to!(
                    "person",
                    tag: "is friends with",
                    validation_package: || hdk::ValidationPackageDefinition::Entry,
                    validation: |_base: Address, _target: Address, _ctx: hdk::ValidationData| {
                        Ok(())
                    }
                )
            ]
        )
    ]

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
            add_person: {
                inputs: |name: String|,
                outputs: |result: JsonString|,
                handler: handle_add_person
            }
            get_person: {
                inputs: |address: HashString|,
                outputs: |result: JsonString|,
                handler: handle_get_person
            }
            link_people: {
                inputs: |base: HashString, target: HashString, tag: String|,
                outputs: |result: JsonString|,
                handler: handle_link_people
            }
            get_relationships: {
                inputs: |address: HashString, tag: String|,
                outputs: |result: JsonString|,
                handler: handle_get_relationships
            }
        }
    }
}
