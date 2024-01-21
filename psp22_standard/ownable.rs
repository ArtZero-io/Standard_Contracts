use crate::PSP22Error;
use ink::{
    primitives::AccountId,
};

#[ink::storage_item]
#[derive(Debug, Default)]
pub struct OwnableData {
    owner: Option<AccountId>
}

impl OwnableData {
    pub fn new(owner: Option<AccountId>) -> OwnableData {
        let data = OwnableData {
            owner: owner,
        };
        data
    }

    pub fn renounce_ownership(&mut self) -> Result<(), PSP22Error> {
        self.owner = None;
        Ok(())
    }

    pub fn transfer_ownership(&mut self, new_owner: Option<AccountId>) -> Result<(), PSP22Error> {
        if new_owner == None {
            return Err(PSP22Error::NewOwnerIsNotSet)
        }
        self.owner = new_owner;
        Ok(())
    }

    pub fn owner(&self) -> Option<AccountId> {
        self.owner
    }
}
