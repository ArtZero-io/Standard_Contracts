use crate::balances::balance_manager::Balances;
use crate::PSP34Error;
use ink::{
    prelude::{string::String, vec, vec::Vec},
    primitives::AccountId,
    storage::Mapping,
};

#[cfg(feature = "std")]
use ink::storage::traits::StorageLayout;

/// Type for a PSP34 token id.
/// Contains all the possible permutations of id according to the standard.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub enum Id {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Bytes(Vec<u8>),
}

/// Temporary type for events emitted during operations that change the
/// state of PSP34Data struct.
/// This is meant to be replaced with proper ink! events as soon as the
/// language allows for event definitions outside contracts.
pub enum PSP34Event {
    Transfer {
        from: Option<AccountId>,
        to: Option<AccountId>,
        id: Id,
    },
    Approval {
        owner: AccountId,
        operator: AccountId,
        id: Option<Id>,
        approved: bool,
    },
    AttributeSet {
        id: Id,
        key: Vec<u8>,
        data: Vec<u8>,
    },
}

#[ink::storage_item]
#[derive(Debug, Default)]
pub struct PSP34Data {
    token_owner: Mapping<Id, AccountId>,
    operator_approvals: Mapping<(AccountId, AccountId, Option<Id>), ()>,
    balance: Balances,
    last_token_id: u64,
    locked_tokens: Mapping<Id, bool>,
    locked_token_count: u64,
}

impl PSP34Data {
    pub fn new() -> PSP34Data {
        Default::default()
    }

    pub fn total_supply(&self) -> u128 {
        self.balance.total_supply()
    }

    pub fn balance_of(&self, owner: AccountId) -> u32 {
        self.balance.balance_of(&owner)
    }

    pub fn owner_of(&self, id: &Id) -> Option<AccountId> {
        self.token_owner.get(id)
    }

    pub fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<&Id>) -> bool {
        self.operator_approvals
            .get((owner, operator, &None))
            .is_some()
            || id.is_some() && self.operator_approvals.get((owner, operator, id)).is_some()
    }

    pub fn collection_id(&self, account_id: AccountId) -> Id {
        Id::Bytes(<_ as AsRef<[u8; 32]>>::as_ref(&account_id).to_vec())
    }

    pub fn approve(
        &mut self,
        mut caller: AccountId,
        operator: AccountId,
        id: Option<Id>,
        approved: bool,
    ) -> Result<Vec<PSP34Event>, PSP34Error> {
        if let Some(id) = &id {
            let owner = self.owner_of(id).ok_or(PSP34Error::TokenNotExists)?;
            if approved && owner == operator {
                return Err(PSP34Error::SelfApprove);
            }

            if owner != caller && !self.allowance(owner, caller, None) {
                return Err(PSP34Error::NotApproved);
            }

            if !approved && self.allowance(owner, operator, None) {
                return Err(PSP34Error::Custom(String::from(
                    "Cannot revoke approval for a single token, when the operator has approval for all tokens."
                )));
            }
            caller = owner;
        }

        if approved {
            self.operator_approvals
                .insert((caller, operator, id.as_ref()), &());
        } else {
            self.operator_approvals
                .remove((caller, operator, id.as_ref()));
        }

        Ok(vec![PSP34Event::Approval {
            owner: caller,
            operator,
            id,
            approved,
        }])
    }

    pub fn transfer(
        &mut self,
        caller: AccountId,
        to: AccountId,
        id: Id,
        _data: Vec<u8>,
    ) -> Result<Vec<PSP34Event>, PSP34Error> {
        let owner = self.owner_of(&id).ok_or(PSP34Error::TokenNotExists)?;

        if owner == to {
            return Ok(vec![]);
        }

        if owner != caller && !self.allowance(owner, caller, Some(&id)) {
            return Err(PSP34Error::NotApproved);
        }

        self.balance.decrease_balance(&owner, &id, false);

        self.operator_approvals.remove((owner, caller, Some(&id)));
        self.token_owner.remove(&id);

        self.token_owner.insert(&id, &to);
        self.balance.increase_balance(&to, &id, false)?;

        Ok(vec![PSP34Event::Transfer {
            from: Some(caller),
            to: Some(to),
            id,
        }])
    }

    pub fn mint(&mut self, account: AccountId, id: Id) -> Result<Vec<PSP34Event>, PSP34Error> {
        if self.owner_of(&id).is_some() {
            return Err(PSP34Error::TokenExists);
        }
        self.balance.increase_balance(&account, &id, true)?;
        self.token_owner.insert(&id, &account);

        Ok(vec![PSP34Event::Transfer {
            from: None,
            to: Some(account),
            id,
        }])
    }

    pub fn burn(
        &mut self,
        caller: AccountId,
        account: AccountId,
        id: Id,
    ) -> Result<Vec<PSP34Event>, PSP34Error> {
        if self.owner_of(&id).is_none() {
            return Err(PSP34Error::TokenNotExists);
        }
        if account != caller && !self.allowance(caller, account, None) {
            return Err(PSP34Error::NotApproved);
        }
        self.balance.decrease_balance(&account, &id, true);
        self.token_owner.remove(&id);

        Ok(vec![PSP34Event::Transfer {
            from: Some(account),
            to: None,
            id,
        }])
    }

    pub fn get_last_token_id(&self) -> u64 {
        self.last_token_id
    }

    pub fn lock(&mut self, token_id: Id) -> Result<(), PSP34Error> {
        if let Some(locked_token_count) = self.locked_token_count.checked_add(1) {
            self.locked_token_count = locked_token_count;
            self.locked_tokens.insert(&token_id, &true);
            return Ok(());
        } else {
            return Err(PSP34Error::Custom(String::from("Cannot increase locked token count")));
        }
    }
    
    pub fn is_locked_nft(&self, token_id: Id) -> bool {
        if self.locked_tokens.get(&token_id).is_some() {
            return true;
        }
        return false;
    }
    
    pub fn get_locked_token_count(&self) -> u64 {
        self.locked_token_count
    }

    #[cfg(feature = "enumerable")]
    pub fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Result<Id, PSP34Error> {
        self.balance.owners_token_by_index(owner, index)
    }

    #[cfg(feature = "enumerable")]
    pub fn token_by_index(&self, index: u128) -> Result<Id, PSP34Error> {
        self.balance.token_by_index(index)
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::U128(0)
    }
}

impl From<Id> for u128 {
    fn from(id: Id) -> Self {
        match id {
            Id::U8(val) => val as u128,
            Id::U16(val) => val as u128,
            Id::U32(val) => val as u128,
            Id::U64(val) => val as u128,
            Id::U128(val) => val,
            Id::Bytes(val) => u128::from_be_bytes(val.as_slice().try_into().unwrap()),
        }
    }
}