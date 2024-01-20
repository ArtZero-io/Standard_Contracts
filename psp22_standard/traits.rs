use ink::{
    prelude::{string::String, vec::Vec},
    primitives::AccountId,
};

use crate::errors::PSP22Error;

#[ink::trait_definition]
pub trait PSP22 {
    
    #[ink(message)]
    fn total_supply(&self) -> u128;

    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> u128;

    #[ink(message)]
    fn allowance(&self, owner: AccountId, spender: AccountId) -> u128;

    #[ink(message)]
    fn transfer(&mut self, to: AccountId, value: u128, data: Vec<u8>) -> Result<(), PSP22Error>;

    #[ink(message)]
    fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        value: u128,
        data: Vec<u8>,
    ) -> Result<(), PSP22Error>;

    #[ink(message)]
    fn approve(&mut self, spender: AccountId, value: u128) -> Result<(), PSP22Error>;

    #[ink(message)]
    fn increase_allowance(
        &mut self,
        spender: AccountId,
        delta_value: u128,
    ) -> Result<(), PSP22Error>;

    #[ink(message)]
    fn decrease_allowance(
        &mut self,
        spender: AccountId,
        delta_value: u128,
    ) -> Result<(), PSP22Error>;
}

#[ink::trait_definition]
pub trait PSP22Metadata {
    #[ink(message)]
    fn token_name(&self) -> Option<String>;
    #[ink(message)]
    fn token_symbol(&self) -> Option<String>;
    #[ink(message)]
    fn token_decimals(&self) -> u8;
}

#[ink::trait_definition]
pub trait PSP22Burnable {
    #[ink(message)]
    fn burn(&mut self, from: AccountId, value: u128) -> Result<(), PSP22Error>;
}

#[ink::trait_definition]
pub trait PSP22Mintable {
    #[ink(message)]
    fn mint(&mut self, account: AccountId, value: u128) -> Result<(), PSP22Error>;
}

#[ink::trait_definition]
pub trait PSP22Capped {
    #[ink(message)]
    fn cap(&mut self) -> u128;
}
