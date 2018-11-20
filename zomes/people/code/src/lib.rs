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

// see https://holochain.github.io/rust-api/0.0.1/hdk/ for info on using the hdk library

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Post {
    content: String,
    date_created: String,
}

fn handle_post_address(content: String) -> JsonString {

    let post_entry = Entry::new(EntryType::App("post".into()), Post {
        content,
        date_created: "now".into(),
    });

    match hdk::entry_address(&post_entry) {
        Ok(address) => address.into(),
        Err(hdk_error) => hdk_error.into(),
    }

}

define_zome! {
    entries: [
        entry!(
            name: "post",
            description: "",
            sharing: Sharing::Public,
            native_type: Post,

            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },

            validation: |post: Post, _ctx: hdk::ValidationData| {
                (post.content.len() < 280)
                    .ok_or_else(|| String::from("Content too long"))
            }
        )
    ]

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
            // the name of this function, "hash_post" is the
            // one to give while performing a `call` method to this function.
            // the name of the handler function must be different than the
            // name of the Zome function.
            hash_post: {
                inputs: |content: String|,
                outputs: |post: serde_json::Value|,
                handler: handle_post_address
            }
        }
    }
}
