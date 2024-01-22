use ink::{
    prelude::vec::Vec,
    primitives::AccountId,
};

use crate::data::Id;
use crate::errors::PSP34Error;

#[ink::trait_definition]
pub trait PSP34 {
    #[ink(message)]
    fn collection_id(&self) -> Id;

    #[ink(message)]
    fn total_supply(&self) -> u128;

    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> u32;

    #[ink(message)]
    fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> bool;

    #[ink(message)]
    fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error>;

    #[ink(message)]
    fn approve(
        &mut self,
        operator: AccountId,
        id: Option<Id>,
        approved: bool,
    ) -> Result<(), PSP34Error>;

    /// Returns the owner of the token if any.
    #[ink(message)]
    fn owner_of(&self, id: Id) -> Option<AccountId>;
}

#[ink::trait_definition]
pub trait PSP34Traits {
    // #[ink(message)]
    // fn set_base_uri(&mut self, uri: String) -> Result<(), Error>;
    
    // #[ink(message)]
    // fn set_multiple_attributes(
    //     &mut self,
    //     token_id: Id,
    //     metadata: Vec<(String, String)>
    // ) -> Result<(), Error>;
    
    // #[ink(message)]
    // fn get_attributes(&self, token_id: Id, attributes: Vec<String>) -> Vec<String>;
    
    // #[ink(message)]
    // fn get_attribute_count(&self) -> u32;
    
    // #[ink(message)]
    // fn get_attribute_name(&self, index: u32) -> String;
    
    // #[ink(message)]
    // fn token_uri(&self, token_id: u64) -> String;
    
    #[ink(message)]
    fn get_owner(&self) -> AccountId ;
    
    #[ink(message)]
    fn get_last_token_id(&self) -> u64;
    
    #[ink(message)]
    fn lock(&mut self, token_id: Id) -> Result<(), PSP34Error>;
    
    #[ink(message)]
    fn is_locked_nft(&self, token_id: Id) -> bool;
    
    #[ink(message)]
    fn get_locked_token_count(&self) -> u64;
}

#[ink::trait_definition]
pub trait PSP34Metadata {
    #[ink(message)]
    fn get_attribute(&self, id: Id, key: Vec<u8>) -> Option<Vec<u8>>;
}

#[cfg(feature = "enumerable")]
#[ink::trait_definition]
pub trait PSP34Enumerable {
    #[ink(message)]
    fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Result<Id, PSP34Error>;

    #[ink(message)]
    fn token_by_index(&self, index: u128) -> Result<Id, PSP34Error>;
}

#[ink::trait_definition]
pub trait Ownable {
    #[ink(message)]
    fn owner(&self) -> Option<AccountId>;

    #[ink(message)]
    fn renounce_ownership(&mut self) -> Result<(), PSP34Error>;

    #[ink(message)]
    fn transfer_ownership(&mut self, new_owner: Option<AccountId>) -> Result<(), PSP34Error>;
}
