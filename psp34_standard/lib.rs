#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod balances;
mod data;
mod errors;
pub mod metadata;
mod traits;
mod unit_tests;
mod ownable;

pub use data::{Id, PSP34Data, PSP34Event};
pub use errors::PSP34Error;
pub use traits::{PSP34Metadata, PSP34, PSP34Traits, Ownable};
pub use ownable::OwnableData;

#[cfg(feature = "enumerable")]
pub use traits::PSP34Enumerable;

#[cfg(feature = "contract")]
#[ink::contract]
mod token {
    use crate::{
        metadata, 
        Id, 
        PSP34Data, 
        PSP34Error, 
        PSP34Event, 
        PSP34Metadata, 
        PSP34,
        PSP34Traits,
        Ownable,
        OwnableData,
    };
    use ink::prelude::{string::String, vec::Vec};

    #[cfg(feature = "enumerable")]
    use crate::PSP34Enumerable;

    #[ink(storage)]
    pub struct Token {
        data: PSP34Data,
        metadata: metadata::Data,
        ownable_data: OwnableData
    }

    impl Token {
        #[ink(constructor)]
        pub fn new(contract_owner: AccountId) -> Self {
            Self {
                data: PSP34Data::new(),
                metadata: metadata::Data::default(),
                ownable_data: OwnableData::new(Some(contract_owner))
            }
        }

        fn emit_events(&self, events: ink::prelude::vec::Vec<PSP34Event>) {
            for event in events {
                match event {
                    PSP34Event::Approval {
                        owner,
                        operator,
                        id,
                        approved,
                    } => self.env().emit_event(Approval {
                        owner,
                        operator,
                        id,
                        approved,
                    }),
                    PSP34Event::Transfer { from, to, id } => {
                        self.env().emit_event(Transfer { from, to, id })
                    }
                    PSP34Event::AttributeSet { id, key, data } => {
                        self.env().emit_event(AttributeSet { id, key, data })
                    }
                }
            }
        }
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        operator: AccountId,
        #[ink(topic)]
        id: Option<Id>,
        approved: bool,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        id: Id,
    }

    #[ink(event)]
    pub struct AttributeSet {
        id: Id,
        key: Vec<u8>,
        data: Vec<u8>,
    }

    impl PSP34 for Token {
        #[ink(message)]
        fn collection_id(&self) -> Id {
            self.data.collection_id(self.env().account_id())
        }

        #[ink(message)]
        fn total_supply(&self) -> u128 {
            self.data.total_supply()
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> u32 {
            self.data.balance_of(owner)
        }

        #[ink(message)]
        fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> bool {
            self.data.allowance(owner, operator, id.as_ref())
        }

        #[ink(message)]
        fn transfer(
            &mut self,
            to: AccountId,
            id: Id,
            data: ink::prelude::vec::Vec<u8>,
        ) -> Result<(), PSP34Error> {
            let events = self.data.transfer(self.env().caller(), to, id, data)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn approve(
            &mut self,
            operator: AccountId,
            id: Option<Id>,
            approved: bool,
        ) -> Result<(), PSP34Error> {
            let events = self
                .data
                .approve(self.env().caller(), operator, id, approved)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn owner_of(&self, id: Id) -> Option<AccountId> {
            self.data.owner_of(&id)
        }
    }

    impl PSP34Traits for Token {
        #[ink(message)]
        fn get_owner(&self) -> AccountId {
            self.ownable_data.owner().unwrap()
        }
        
        #[ink(message)]
        fn get_last_token_id(&self) -> u64 {
            self.data.get_last_token_id()
        }

        #[ink(message)]
        fn lock(&mut self, token_id: Id) -> Result<(), PSP34Error> {
            if Some(self.env().caller()) != self.data.owner_of(&token_id) {
                return Err(PSP34Error::NotTokenOwner);
            }
            self.data.lock(token_id)
        }
        
        #[ink(message)]
        fn is_locked_nft(&self, token_id: Id) -> bool {
            self.data.is_locked_nft(token_id)
        }
        
        #[ink(message)]
        fn get_locked_token_count(&self) -> u64 {
            self.data.get_locked_token_count()
        }
    }

    impl PSP34Metadata for Token {
        #[ink(message)]
        fn get_attribute(&self, id: Id, key: Vec<u8>) -> Option<Vec<u8>> {
            self.metadata.get_attribute(id, key)
        }
    }

    impl Ownable for Token {
        #[ink(message)]
        fn owner(&self) -> Option<AccountId> {
            self.ownable_data.owner()
        }

        #[ink(message)]
        fn transfer_ownership(&mut self, new_owner: Option<AccountId>) -> Result<(), PSP34Error> {
            if self.owner() != Some(self.env().caller()) {
                return Err(PSP34Error::CallerIsNotOwner)
            }
            self.ownable_data.transfer_ownership(new_owner)
        }

        #[ink(message)]
        fn renounce_ownership(&mut self) -> Result<(), PSP34Error> {
            if self.owner() != Some(self.env().caller()) {
                return Err(PSP34Error::CallerIsNotOwner)
            }
            self.ownable_data.renounce_ownership()
        }
    }
    
    #[cfg(test)]
    mod tests {
        crate::tests!(Token, Token::new);
    }
}
