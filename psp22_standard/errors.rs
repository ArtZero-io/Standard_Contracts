use ink::prelude::string::String;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP22Error {
    Custom(String),
    InsufficientBalance,
    InsufficientAllowance,
    ZeroRecipientAddress,
    ZeroSenderAddress,
    SafeTransferCheckFailed(String),
    CapExceeded,
    NewOwnerIsNotSet,
    CallerIsNotOwner,
    RoleRedundant,
    MissingRole,
    InvalidCaller
}