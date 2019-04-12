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

// Defines a struct which acts as the fundamental data schema for the "person" entry type
// EVERY struct that will be used as an entry type needs to have the following line (#[derive]...), as is,
// implementing important functions for deserializing/serializing to and from JSON data
#[derive(Serialize, Deserialize, Clone, Debug, DefaultJson)]
struct Person {
    name: String,
}

// this is the handler for the Zome "add_person" function
fn handle_add_person(name: String) -> ZomeApiResult<Address> {
    // Entry::App() expects two arguments, entry type, and entry value
    // Entry::App() instantiates an "app entry", with the given 
    // "person" is the entry type, it should match the "name" property of an entry type given in define_zome!
    // .into() converts this string automatically into an AppEntryType, which is the type Entry::App expects as the first argument
    // Person { name: name } is the entry value. It is constructed with the `name` argument that was given to the function call
    // as the name property of the Person.
    // .into() converts this Person into a value of type 
    let post_entry = Entry::App("person".into(), Person { name: name }.into());
    // call the hdk::commit_entry function with our entry
    // this will cause the HDK to communicate with Holochain core
    // to attempt to write this data to the source chain
    // this can fail, if the validation callback fails, for the entry
    // -> calling a function without using a semicolon (;) at the end
    // -> means the result will be used as the return value for this function call
    // -> since the ZomeApiResult<Address> type we named as the return type
    // -> for this function is the same as the return type for hdk::commit_entry
    // -> we can do this directly
    hdk::commit_entry(&post_entry)
}

// this is the handler for the Zome "update_person" function
fn handle_update_person(address: Address, name: String) -> ZomeApiResult<Address> {
    // this does the same thing as in `handle_add_person`, constructs an app entry
    let post_entry = Entry::App("person".into(), Person { name: name }.into());
    // update_entry takes a new app entry, and then the address of the initial entry
    // which the new one will be marked as replacing
    // -> when requests for the initial entry are made (dependent on configuration options)
    // -> Holochain will automatically fetch the latest version for that, instead
    // -> of the initial
    hdk::update_entry(post_entry, &address)
}

// this is the handler for the Zome "remove_person" function
fn handle_remove_person(address: Address) -> ZomeApiResult<()> {
    // remove_entry simply takes the address of an entry, which it will
    // write a new entry, marking the old one as deleted
    // it only returns the "unit type" of Rust () which is like a null value
    hdk::remove_entry(&address)
}

// this is the handler for the Zome "get_person" function
// the return type is ZomeApiResult<Option<Entry>>
// the ZomeApiResult indicates at outer level whether the function call succeeded or not
// can be "Ok" or an "Err"
// the Option indicates that when calling get_entry the result can either be 
// "Some" if there was an Entry at that address, or "None" if there wasn't. 
// If "Some", then there is an Entry
fn handle_get_person(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}

// this is the handler for the Zome "link_people" function
fn handle_link_people(base: Address, target: Address, tag: String) -> ZomeApiResult<()> {
    // note that when you do link_entries, you can only do get_entries
    // from the "base" to the "target"
    // if you want to retrieve in the other direction, you have to create two links,
    // one in each direction
    hdk::link_entries(&base, &target, tag)
}

// this is the handler for the Zome "get_relationships" function
fn handle_get_relationships(address: Address, tag: String) -> ZomeApiResult<GetLinksResult> {
    // get_links takes a "tag", which should be the same as the one that was
    // used during "link_entries", in order for it to be found
    hdk::get_links(&address, tag)
}

// define_zome! is an HDK macro required for every Zome
// it does a lot of magic stuff internally, to keep the code you have
// to write and think about simpler, and Holochain specific
define_zome! {
    entries: [
        entry!(
            // this is the name of the entry type
            name: "person",
            // this is just for humans to read, to know what this entry type is for
            description: "a human with a name",
            // whether entries of this type should get shared to the DHT or not
            // -> note that Private entries are not yet enabled, but will be soon
            sharing: Sharing::Public,
            // specify which data from the originator of an entry (during validation)
            // is needed to perform validation. In this case, just the entry itself
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            // the validation callback to perform, when an agent attempts to commit
            // an entry of this type, or if a node receives a request to hold
            // this entry in its shard of the DHT
            // 'person' is the entry value itself, '_validation_data' is
            // extra contextual information, plus the "validation package" if it was 
            // requested, that can be used to validate against
            validation: |validation_data: hdk::EntryValidationData<Person>| {
                // this line uses the 'boolinator' import to take a boolean value
                // (person.name.len() >= 2) and convert it into a Rust "Result" type.
                // It should provide a string if it fails validation, which will be
                // given as the error value back to the caller
                match validation_data {
                    EntryValidationData::Create{entry:person,validation_data:_} => {
                        (person.name.len() > 2).ok_or_else(|| String::from("Name must be at least 2 characters"))
                    },
                    _ => Ok(()),
                }
            },
            links: [
                // to! is another HDK macro, used for defining links types
                to!(
                    // the entry type this links between
                    "person",
                    // note that for the time being (in the future it may change)
                    // the only tag that can be used is the tag defined here
                    tag: "is friends with",
                    // similar to validation_package callback for an entry type
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    // similar to validation callback for an entry type
                    validation: |_validation_data: hdk::LinkValidationData| {
                        // for now, perform no interesting validation, just return Ok
                        Ok(())
                    }
                )
            ]
        )
    ]

    // genesis is a callback that happens when a new initializes a DNA instance
    // for the first time.
    genesis: || { Ok(()) }

    // these are function declarations
    // they're required in order to expose extra information to Holochain about the function signatures
    // of the functions in this Zome.
    // Holochain will use that information internally in a variety of ways, that aren't all defined yet.
    functions: [
        // add_person is the publicly callable name of the function
        add_person: {
            // inputs must match the arguments of the function named as the 'handler' below
            // -> the handler can't have the same name as the publicly callable function
            inputs: |name: String|,
            // outputs should always only have one argument, call it what you want
            // but it must be listed as being of the same type as the return type
            // of the function named as the 'handler' below
            outputs: |result: ZomeApiResult<Address>|,
            // handler is a reference to the name of the private function to call
            // to actual handle inbound requests for the function being declared
            handler: handle_add_person
        }
        update_person: {
            inputs: |address: Address, name: String|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_update_person
        }
        remove_person: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<()>|,
            handler: handle_remove_person
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

    // make sure to list every function that you want accessible via the API here
    // use the same name that you use in the `functions` declarations immediately above
    // Note: traits always has to come after `functions` in `define_zome!`
    traits: {
        hc_public [add_person, update_person, remove_person, get_person, link_people, get_relationships]
    }
}
