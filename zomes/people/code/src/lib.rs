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

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
struct Person {
    name: String,
}

fn handle_add_person(name: String) -> ZomeApiResult<Address> {
    let post_entry = Entry::App("person".into(), Person {
        name,
    }.into());

   let address = hdk::commit_entry(&post_entry)?;

   Ok(address)
}

fn handle_get_person(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}

fn handle_link_people(base: Address, target: Address, tag: String) -> ZomeApiResult<()> {
    hdk::link_entries(&base, &target, tag)
}

fn handle_get_relationships(address: Address, tag: String) -> ZomeApiResult<GetLinksResult> {
    hdk::get_links(&address, tag)
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
            validation: |person: Person, _validation_data: hdk::ValidationData| {
                (person.name.len() >= 2)
                    .ok_or_else(|| String::from("Name must be at least 2 characters"))
            },
            links: [
                to!(
                    "person",
                    tag: "is friends with",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: |_base: Address, _target: Address, _ctx: hdk::ValidationData| {
                        Ok(())
                    }
                )
            ]
        )
    ]

    genesis: || { Ok(()) }

    functions: [
        add_person: {
            inputs: |name: String|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_add_person
        }
        get_person: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<Option<Entry>>|,
            handler: handle_get_person
        }
        link_people: {
            inputs: |base: Address, target: Address, tag: String|,
            outputs: |result: ZomeApiResult<()>|,
            handler: handle_link_people
        }
        get_relationships: {
            inputs: |address: Address, tag: String|,
            outputs: |result: ZomeApiResult<GetLinksResult>|,
            handler: handle_get_relationships
        }
    ]

    capabilities: {
        main (Public) [add_person, get_person, link_people, get_relationships]
    }
}
