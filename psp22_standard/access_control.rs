use crate::PSP22Error;
use ink::{
    primitives::AccountId,
    storage::Mapping,
};

pub type RoleType = u32;
pub const DEFAULT_ADMIN_ROLE: RoleType = 0;

#[ink::storage_item]
#[derive(Debug, Default)]
pub struct AccessControlData {
    admin_roles: Mapping<RoleType, RoleType>,
    members: Mapping<(RoleType, Option<AccountId>), ()>,
}

impl AccessControlData {
    pub fn new() -> AccessControlData {
        let data = AccessControlData {
            admin_roles: Default::default(),
            members: Default::default()
        };
        data
    }

    pub fn has_role(&self, role: RoleType, address: Option<AccountId>) -> bool {
        self.members.contains(&(role, address))
    }

    pub fn get_role_admin(&self, role: RoleType) -> RoleType {
        self.admin_roles.get(role).unwrap_or(DEFAULT_ADMIN_ROLE)
    }

    pub fn only_role(&self, role: RoleType, account: Option<AccountId>) -> bool {
        if !self.has_role(role, account) {
            return false;
        }
        return true;
    }

    pub fn grant_role(&mut self, role: RoleType, account: Option<AccountId>) -> Result<(), PSP22Error> {
        if self.has_role(role, account) {
            return Err(PSP22Error::RoleRedundant)
        }
        self.members.insert((role, account), &());
        Ok(())
    }

    pub fn revoke_role(&mut self, role: RoleType, account: Option<AccountId>) -> Result<(), PSP22Error> {
        if !self.has_role(role, account) {
            return Err(PSP22Error::MissingRole)
        }
        self.members.remove(&(role, account));
        Ok(())
    }
}
