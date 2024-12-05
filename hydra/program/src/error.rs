use anchor_lang::prelude::*;
use std::result::Result as StdResult;
pub trait OrArithError<T> {
    fn or_arith_error(self) -> StdResult<T, error::Error>;
}

impl OrArithError<u64> for Option<u64> {
    fn or_arith_error(self) -> StdResult<u64, error::Error> {
        self.ok_or_else(|| HydraError::BadArtithmetic.into())
    }
}

impl OrArithError<u32> for Option<u32> {
    fn or_arith_error(self) -> StdResult<u32, error::Error> {
        self.ok_or_else(|| HydraError::BadArtithmetic.into())
    }
}

impl OrArithError<u128> for Option<u128> {
    fn or_arith_error(self) -> StdResult<u128, error::Error> {
        self.ok_or_else(|| HydraError::BadArtithmetic.into())
    }
}

#[error_code]
pub enum HydraError {
    #[msg("Encountered an arithmetic error")]
    BadArtithmetic,

    #[msg("Invalid authority")]
    InvalidAuthority,

    #[msg("Not Enough Available Shares")]
    InsufficientShares,

    #[msg("All available shares must be assigned to a member")]
    SharesArentAtMax,

    #[msg("A New mint account must be provided")]
    NewMintAccountRequired,

    #[msg("A Token type Fanout requires a Membership Mint")]
    MintAccountRequired,

    #[msg("Invalid Membership Model")]
    InvalidMembershipModel,

    #[msg("Invalid Membership Voucher")]
    InvalidMembershipVoucher,

    #[msg("Invalid Mint for the config")]
    MintDoesNotMatch,

    #[msg("Holding account does not match the config")]
    InvalidHoldingAccount,

    #[msg("A Mint holding account must be an ata for the mint owned by the config")]
    HoldingAccountMustBeAnATA,

    DerivedKeyInvalid,

    IncorrectOwner,

    #[msg("Wallet Does not Own Membership Token")]
    WalletDoesNotOwnMembershipToken,

    #[msg("The Metadata specified is not valid Token Metadata")]
    InvalidMetadata,

    NumericalOverflow,

    #[msg("Not enough new balance to distribute")]
    InsufficientBalanceToDistribute,

    InvalidFanoutForMint,

    #[msg(
        "This operation must be the instruction right after a distrobution on the same accounts."
    )]
    MustDistribute,

    InvalidStakeAta,

    CannotTransferToSelf,

    #[msg("Transfer is not supported on this membership model")]
    TransferNotSupported,

    #[msg("Remove is not supported on this membership model")]
    RemoveNotSupported,

    #[msg("Before you remove a wallet or NFT member please transfer the shares to another member")]
    RemoveSharesMustBeZero,

    #[msg("Sending Sol to a non-Native Account destination will render the sol unusable")]
    InvalidCloseAccountDestination,
}
