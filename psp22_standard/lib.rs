#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod data;
mod errors;
mod testing;
mod traits;
mod ownable;
mod access_control;

pub use data::{PSP22Data, PSP22Event};
pub use ownable::OwnableData;
pub use access_control::{AccessControlData, RoleType, DEFAULT_ADMIN_ROLE};
pub use errors::PSP22Error;
pub use traits::{PSP22Burnable, PSP22Metadata, PSP22Mintable, PSP22Capped, UpgradeableTrait, Ownable, AccessControl, AdminTrait, PSP22};

#[cfg(feature = "contract")]
#[ink::contract]
mod token {
    use crate::{
        PSP22Data,  
        PSP22Error, 
        PSP22Event, 
        PSP22Metadata, 
        PSP22Mintable, 
        PSP22Burnable, 
        PSP22Capped, 
        UpgradeableTrait,
        AdminTrait,
        Ownable,
        OwnableData,
        AccessControl, 
        AccessControlData,
        DEFAULT_ADMIN_ROLE,
        RoleType,
        PSP22
    };
    use ink::prelude::{string::String, vec::Vec};

    #[ink(storage)]
    pub struct Token {
        data: PSP22Data,
        ownable_data: OwnableData,
        access_control_data: AccessControlData,
        name: Option<String>,
        symbol: Option<String>,
        decimals: u8,
    }

    impl Token {
        #[ink(constructor)]
        pub fn new(
            cap: u128,
            name: Option<String>,
            symbol: Option<String>,
            decimals: u8,
        ) -> Self {
            Self {
                data: PSP22Data::new(cap),
                ownable_data: OwnableData::new(Some(Self::env().caller())),
                access_control_data: AccessControlData::new(),
                name,
                symbol,
                decimals,
            }
        }

        fn emit_events(&self, events: Vec<PSP22Event>) {
            for event in events {
                match event {
                    PSP22Event::Transfer { from, to, value } => {
                        self.env().emit_event(Transfer { from, to, value })
                    }
                    PSP22Event::Approval {
                        owner,
                        spender,
                        amount,
                    } => self.env().emit_event(Approval {
                        owner,
                        spender,
                        amount,
                    }),
                }
            }
        }
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        amount: u128,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: u128,
    }

    impl PSP22 for Token {
        #[ink(message)]
        fn total_supply(&self) -> u128 {
            self.data.total_supply()
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> u128 {
            self.data.balance_of(owner)
        }

        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> u128 {
            self.data.allowance(owner, spender)
        }

        #[ink(message)]
        fn transfer(
            &mut self,
            to: AccountId,
            value: u128,
            _data: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            let events = self.data.transfer(self.env().caller(), to, value)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: u128,
            _data: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            let events = self
                .data
                .transfer_from(self.env().caller(), from, to, value)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: u128) -> Result<(), PSP22Error> {
            let events = self.data.approve(self.env().caller(), spender, value)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn increase_allowance(
            &mut self,
            spender: AccountId,
            delta_value: u128,
        ) -> Result<(), PSP22Error> {
            let events = self
                .data
                .increase_allowance(self.env().caller(), spender, delta_value)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn decrease_allowance(
            &mut self,
            spender: AccountId,
            delta_value: u128,
        ) -> Result<(), PSP22Error> {
            let events = self
                .data
                .decrease_allowance(self.env().caller(), spender, delta_value)?;
            self.emit_events(events);
            Ok(())
        }
    }

    impl PSP22Metadata for Token {
        #[ink(message)]
        fn token_name(&self) -> Option<String> {
            self.name.clone()
        }
        #[ink(message)]
        fn token_symbol(&self) -> Option<String> {
            self.symbol.clone()
        }
        #[ink(message)]
        fn token_decimals(&self) -> u8 {
            self.decimals
        }
    }

    impl PSP22Mintable for Token {
        #[ink(message)]
        fn mint(&mut self, to: AccountId, value: u128) -> Result<(), PSP22Error> {
            let events = self.data.mint(to, value)?;
            self.emit_events(events);
            Ok(())
        }
    }

    impl PSP22Burnable for Token {
        #[ink(message)]
        fn burn(&mut self, from: AccountId, value: u128) -> Result<(), PSP22Error> {
            let events = self.data.burn(from, value)?;
            self.emit_events(events);
            Ok(())
        }
    }

    impl PSP22Capped for Token {
        #[ink(message)]
        fn cap(&mut self) -> u128 {
            self.data.cap()
        }
    }

    impl UpgradeableTrait for Token {
        #[ink(message)]
        fn set_code(&mut self, code_hash: [u8; 32]) -> Result<(), PSP22Error> {
            if self.ownable_data.owner() != Some(self.env().caller()) {
                return Err(PSP22Error::CallerIsNotOwner)
            }
            ink::env::set_code_hash(&code_hash).unwrap_or_else(|err| {
                panic!(
                    "Failed to `set_code_hash` to {:?} due to {:?}",
                    code_hash, err
                )
            });
            Ok(())
        }
    }

    impl Ownable for Token {
        #[ink(message)]
        fn owner(&self) -> Option<AccountId> {
            self.ownable_data.owner()
        }

        #[ink(message)]
        fn transfer_ownership(&mut self, new_owner: Option<AccountId>) -> Result<(), PSP22Error> {
            if self.owner() != Some(self.env().caller()) {
                return Err(PSP22Error::CallerIsNotOwner)
            }
            self.ownable_data.transfer_ownership(new_owner)
        }

        #[ink(message)]
        fn renounce_ownership(&mut self) -> Result<(), PSP22Error> {
            if self.owner() != Some(self.env().caller()) {
                return Err(PSP22Error::CallerIsNotOwner)
            }
            self.ownable_data.renounce_ownership()
        }
    }

    impl AccessControl for Token {
        #[ink(message)]
        fn has_role(&self, role: RoleType, address: Option<AccountId>) -> bool {
            self.access_control_data.has_role(role, address)
        }

        #[ink(message)]
        fn get_role_admin(&self, role: RoleType) -> RoleType {
            self.access_control_data.get_role_admin(role)
        }

        #[ink(message)]
        fn init_admin_role(&mut self) -> Result<(), PSP22Error> {
            if self.ownable_data.owner() != Some(self.env().caller()) {
                return Err(PSP22Error::CallerIsNotOwner)
            }
            self.access_control_data.grant_role(DEFAULT_ADMIN_ROLE, Some(self.env().caller()))
        }

        #[ink(message)]
        fn grant_role(&mut self, role: RoleType, account: Option<AccountId>) -> Result<(), PSP22Error> {
            if !self.access_control_data.only_role(self.get_role_admin(role), Some(self.env().caller())) {
                return Err(PSP22Error::MissingRole)
            }
            self.access_control_data.grant_role(role, account)
        }

        #[ink(message)]
        fn revoke_role(&mut self, role: RoleType, account: Option<AccountId>) -> Result<(), PSP22Error> {
            if !self.access_control_data.only_role(self.get_role_admin(role), Some(self.env().caller())) {
                return Err(PSP22Error::MissingRole)
            }
            self.access_control_data.revoke_role(role, account)
        }

        #[ink(message)]
        fn renounce_role(&mut self, role: RoleType, account: Option<AccountId>) -> Result<(), PSP22Error> {
            if account != Some(self.env().caller()) {
                return Err(PSP22Error::InvalidCaller)
            }
            self.access_control_data.revoke_role(role, account)
        }
    }

    impl AdminTrait for Token {
        #[ink(message)]
        fn withdraw_fee(&mut self, value: u128, receiver: AccountId) -> Result<(), PSP22Error> {
            if self.ownable_data.owner() != Some(self.env().caller()) {
                return Err(PSP22Error::CallerIsNotOwner)
            }
            if value > self.env().balance() {
                return Err(PSP22Error::NotEnoughBalance);
            }
            if self.env().transfer(receiver, value).is_err() {
                return Err(PSP22Error::WithdrawFeeError);
            }
            Ok(())
        }

        #[ink(message)]
        fn get_balance(&mut self) -> Result<u128, PSP22Error> {
            Ok(self.env().balance())
        }
    }

    #[cfg(test)]
    mod tests {
        crate::tests!(Token, (|supply| Token::new(supply, None, None, 0)));
    }
}