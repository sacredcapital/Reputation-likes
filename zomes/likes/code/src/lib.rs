#![feature(proc_macro_hygiene)]

use hdk::{error::ZomeApiResult, prelude::*, AGENT_ADDRESS};
use hdk_proc_macros::zome;
extern crate holochain_entry_utils;

//Adapted from patterns developed by @pospigos in the Holo-threaded-comments DNA

mod base;
mod like;
use base::entry::Base;

#[zome]
mod likes_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[zome_fn("hc_public")]
    fn get_my_address() -> ZomeApiResult<Address> {
        Ok(AGENT_ADDRESS.clone())
    }

    // Likes_Entry Definition and functions

    #[entry_def]
    fn handle_get_like() -> ValidatingEntryType {
        crate::like::entry::like_def()
    }

    #[zome_fn("hc_public")]
    fn create(base: Base, author: Address, timestamp: u64) -> ZomeApiResult<Address> {
        crate::like::handlers::create(base, author, timestamp)
    }

    // Base_Entry Definition and functions

    #[entry_def]
    fn handle_get_base() -> ValidatingEntryType {
        crate::base::entry::base_def()
    }
}

/*


use hdk::holochain_core_types::{
    cas::content::Address,
    error::HolochainError,
    json::JsonString,
};

use likes_entry::{
    LikeData,
    Like,
    like_def,
    handle_create_like,
    handle_get_like,
};


use base_entry::{
    base_def,
    handle_get_like,
};



*/

/*
define_zome! {
    entries: [
        like_def(),
        base_def().
    ]


    genesis: || { Ok(())}

    functions:  [
        new: {
            inputs: | like: Likes|,
            outputs: | result: ZomeApiResult<Address>,
            handler: handle_create_like,
        }

    ]

        traits: {
            hc_public [
                create_comment,
                get_like,
                get_likes,
            ]
        }

}

*/

//////////////////////////////////////
/*
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct MyEntry {
    content: String,
}

#[zome]
mod my_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn my_entry_def() -> ValidatingEntryType {
        entry!(
            name: "my_entry",
            description: "this is a same entry defintion",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<MyEntry>| {
                Ok(())
            }
        )
    }

    #[zome_fn("hc_public")]
    fn create_my_entry(entry: MyEntry) -> ZomeApiResult<Address> {
        let entry = Entry::App("my_entry".into(), entry.into());
        let address = hdk::commit_entry(&entry)?;
        Ok(address)
    }

    #[zome_fn("hc_public")]
    fn get_my_entry(address: Address) -> ZomeApiResult<Option<Entry>> {
        hdk::get_entry(&address)
    }
}
*/
