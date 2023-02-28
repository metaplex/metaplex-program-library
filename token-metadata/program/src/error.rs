//! Error types

use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

/// Errors that may be returned by the Metadata program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum MetadataError {
    /// 0 Failed to unpack instruction data
    #[error("")]
    InstructionUnpackError,

    /// Failed to pack instruction data
    #[error("")]
    InstructionPackError,

    /// Lamport balance below rent-exempt threshold.
    #[error("")]
    NotRentExempt,

    /// Already initialized
    #[error("")]
    AlreadyInitialized,

    /// Uninitialized
    #[error("")]
    Uninitialized,

    ///  Metadata's key must match seed of ['metadata', program id, mint] provided
    #[error("")]
    InvalidMetadataKey,

    ///  Edition's key must match seed of ['metadata', program id, name, 'edition'] provided
    #[error("")]
    InvalidEditionKey,

    /// Update Authority given does not match
    #[error("")]
    UpdateAuthorityIncorrect,

    /// Update Authority needs to be signer to update metadata
    #[error("")]
    UpdateAuthorityIsNotSigner,

    /// You must be the mint authority and signer on this transaction
    #[error("")]
    NotMintAuthority,

    /// 10 - Mint authority provided does not match the authority on the mint
    #[error("")]
    InvalidMintAuthority,

    /// Name too long
    #[error("")]
    NameTooLong,

    /// Symbol too long
    #[error("")]
    SymbolTooLong,

    /// URI too long
    #[error("")]
    UriTooLong,

    /// Update authority must be equivalent to the metadata's authority and also signer of this transaction
    #[error("")]
    UpdateAuthorityMustBeEqualToMetadataAuthorityAndSigner,

    /// Mint given does not match mint on Metadata
    #[error("")]
    MintMismatch,

    /// Editions must have exactly one token
    #[error("")]
    EditionsMustHaveExactlyOneToken,

    /// Maximum editions printed already
    #[error("")]
    MaxEditionsMintedAlready,

    /// Token mint to failed
    #[error("")]
    TokenMintToFailed,

    /// The master edition record passed must match the master record on the edition given
    #[error("")]
    MasterRecordMismatch,

    /// 20 - The destination account does not have the right mint
    #[error("")]
    DestinationMintMismatch,

    /// An edition can only mint one of its kind!
    #[error("")]
    EditionAlreadyMinted,

    /// Printing mint decimals should be zero
    #[error("")]
    PrintingMintDecimalsShouldBeZero,

    /// OneTimePrintingAuthorizationMint mint decimals should be zero
    #[error("")]
    OneTimePrintingAuthorizationMintDecimalsShouldBeZero,

    /// Edition mint decimals should be zero
    #[error("")]
    EditionMintDecimalsShouldBeZero,

    /// Token burn failed
    #[error("")]
    TokenBurnFailed,

    /// The One Time authorization mint does not match that on the token account!
    #[error("")]
    TokenAccountOneTimeAuthMintMismatch,

    /// Derived key invalid
    #[error("")]
    DerivedKeyInvalid,

    /// The Printing mint does not match that on the master edition!
    #[error("")]
    PrintingMintMismatch,

    /// The  One Time Printing Auth mint does not match that on the master edition!
    #[error("")]
    OneTimePrintingAuthMintMismatch,

    /// 30 - The mint of the token account does not match the Printing mint!
    #[error("")]
    TokenAccountMintMismatch,

    /// The mint of the token account does not match the master metadata mint!
    #[error("")]
    TokenAccountMintMismatchV2,

    /// Not enough tokens to mint a limited edition
    #[error("")]
    NotEnoughTokens,

    /// The mint on your authorization token holding account does not match your Printing mint!
    #[error("")]
    PrintingMintAuthorizationAccountMismatch,

    /// The authorization token account has a different owner than the update authority for the master edition!
    #[error("")]
    AuthorizationTokenAccountOwnerMismatch,

    /// This feature is currently disabled.
    #[error("")]
    Disabled,

    /// Creators list too long
    #[error("")]
    CreatorsTooLong,

    /// Creators must be at least one if set
    #[error("")]
    CreatorsMustBeAtleastOne,

    /// If using a creators array, you must be one of the creators listed
    #[error("")]
    MustBeOneOfCreators,

    /// This metadata does not have creators
    #[error("")]
    NoCreatorsPresentOnMetadata,

    /// 40 - This creator address was not found
    #[error("")]
    CreatorNotFound,

    /// Basis points cannot be more than 10000
    #[error("")]
    InvalidBasisPoints,

    /// Primary sale can only be flipped to true and is immutable
    #[error("")]
    PrimarySaleCanOnlyBeFlippedToTrue,

    /// Owner does not match that on the account given
    #[error("")]
    OwnerMismatch,

    /// This account has no tokens to be used for authorization
    #[error("")]
    NoBalanceInAccountForAuthorization,

    /// Share total must equal 100 for creator array
    #[error("")]
    ShareTotalMustBe100,

    /// This reservation list already exists!
    #[error("")]
    ReservationExists,

    /// This reservation list does not exist!
    #[error("")]
    ReservationDoesNotExist,

    /// This reservation list exists but was never set with reservations
    #[error("")]
    ReservationNotSet,

    /// This reservation list has already been set!
    #[error("")]
    ReservationAlreadyMade,

    /// 50 - Provided more addresses than max allowed in single reservation
    #[error("")]
    BeyondMaxAddressSize,

    /// NumericalOverflowError
    #[error("")]
    NumericalOverflowError,

    /// This reservation would go beyond the maximum supply of the master edition!
    #[error("")]
    ReservationBreachesMaximumSupply,

    /// Address not in reservation!
    #[error("")]
    AddressNotInReservation,

    /// You cannot unilaterally verify another creator, they must sign
    #[error("")]
    CannotVerifyAnotherCreator,

    /// You cannot unilaterally unverify another creator
    #[error("")]
    CannotUnverifyAnotherCreator,

    /// In initial reservation setting, spots remaining should equal total spots
    #[error("")]
    SpotMismatch,

    /// Incorrect account owner
    #[error("")]
    IncorrectOwner,

    /// printing these tokens would breach the maximum supply limit of the master edition
    #[error("")]
    PrintingWouldBreachMaximumSupply,

    /// Data is immutable
    #[error("")]
    DataIsImmutable,

    /// 60 - No duplicate creator addresses
    #[error("")]
    DuplicateCreatorAddress,

    /// Reservation spots remaining should match total spots when first being created
    #[error("")]
    ReservationSpotsRemainingShouldMatchTotalSpotsAtStart,

    /// Invalid token program
    #[error("")]
    InvalidTokenProgram,

    /// Data type mismatch
    #[error("")]
    DataTypeMismatch,

    /// Beyond alotted address size in reservation!
    #[error("")]
    BeyondAlottedAddressSize,

    /// The reservation has only been partially alotted
    #[error("")]
    ReservationNotComplete,

    /// You cannot splice over an existing reservation!
    #[error("")]
    TriedToReplaceAnExistingReservation,

    /// Invalid operation
    #[error("")]
    InvalidOperation,

    /// Invalid owner
    #[error("")]
    InvalidOwner,

    /// Printing mint supply must be zero for conversion
    #[error("")]
    PrintingMintSupplyMustBeZeroForConversion,

    /// 70 - One Time Auth mint supply must be zero for conversion
    #[error("")]
    OneTimeAuthMintSupplyMustBeZeroForConversion,

    /// You tried to insert one edition too many into an edition mark pda
    #[error("")]
    InvalidEditionIndex,

    // In the legacy system the reservation needs to be of size one for cpu limit reasons
    #[error("")]
    ReservationArrayShouldBeSizeOne,

    /// Is Mutable can only be flipped to false
    #[error("")]
    IsMutableCanOnlyBeFlippedToFalse,

    #[error("")]
    CollectionCannotBeVerifiedInThisInstruction,

    #[error("")]
    Removed, //For the curious we cannot get rid of an instruction in the enum or move them or it will break our api, this is a friendly way to get rid of them

    #[error("")]
    MustBeBurned,

    #[error("")]
    InvalidUseMethod,

    #[error("")]
    CannotChangeUseMethodAfterFirstUse,

    #[error("")]
    CannotChangeUsesAfterFirstUse,

    // 80
    #[error("")]
    CollectionNotFound,

    #[error("")]
    InvalidCollectionUpdateAuthority,

    #[error("")]
    CollectionMustBeAUniqueMasterEdition,

    #[error("")]
    UseAuthorityRecordAlreadyExists,

    #[error("")]
    UseAuthorityRecordAlreadyRevoked,

    #[error("")]
    Unusable,

    #[error("")]
    NotEnoughUses,

    #[error("")]
    CollectionAuthorityRecordAlreadyExists,

    #[error("")]
    CollectionAuthorityDoesNotExist,

    #[error("")]
    InvalidUseAuthorityRecord,

    // 90
    #[error("")]
    InvalidCollectionAuthorityRecord,

    #[error("")]
    InvalidFreezeAuthority,

    #[error("")]
    InvalidDelegate,

    #[error("")]
    CannotAdjustVerifiedCreator,

    #[error("")]
    CannotRemoveVerifiedCreator,

    #[error("")]
    CannotWipeVerifiedCreators,

    #[error("")]
    NotAllowedToChangeSellerFeeBasisPoints,

    /// Edition override cannot be zero
    #[error("")]
    EditionOverrideCannotBeZero,

    #[error("")]
    InvalidUser,

    /// Revoke Collection Authority signer is incorrect
    #[error("")]
    RevokeCollectionAuthoritySignerIncorrect,

    // 100
    #[error("")]
    TokenCloseFailed,

    /// 101 - Calling v1.3 function on unsized collection
    #[error("")]
    UnsizedCollection,

    /// 102 - Calling v1.2 function on a sized collection
    #[error("")]
    SizedCollection,

    /// 103 - Can't burn a verified member of a collection w/o providing collection metadata account
    #[error("")]
    MissingCollectionMetadata,

    /// 104 - This NFT is not a member of the specified collection.
    #[error("")]
    NotAMemberOfCollection,

    /// 105 - This NFT is not a verified member of the specified collection.
    #[error("")]
    NotVerifiedMemberOfCollection,

    /// 106 - This NFT is not a collection parent NFT.
    #[error("")]
    NotACollectionParent,

    /// 107 - Could not determine a TokenStandard type.
    #[error("")]
    CouldNotDetermineTokenStandard,

    /// 108 - Missing edition account for a non-fungible token type.
    #[error("")]
    MissingEditionAccount,

    /// 109 - Not a Master Edition
    #[error("")]
    NotAMasterEdition,

    /// 110 - Master Edition has prints.
    #[error("")]
    MasterEditionHasPrints,

    /// 111 - Borsh Deserialization Error
    #[error("")]
    BorshDeserializationError,

    /// 112 - Cannot update a verified colleciton in this command
    #[error("")]
    CannotUpdateVerifiedCollection,

    /// 113 - Edition Account Doesnt Match Collection
    #[error("")]
    CollectionMasterEditionAccountInvalid,

    /// 114 - Item is already verified.
    #[error("")]
    AlreadyVerified,

    /// 115 - Item is already unverified.
    #[error("")]
    AlreadyUnverified,

    /// 116 - Not a Print Edition
    #[error("")]
    NotAPrintEdition,

    /// 117 - Invalid Edition Marker
    #[error("")]
    InvalidMasterEdition,

    /// 118 - Invalid Edition Marker
    #[error("")]
    InvalidPrintEdition,

    /// 119 - Invalid Edition Marker
    #[error("")]
    InvalidEditionMarker,

    /// 120 - Reservation List is Deprecated
    #[error("")]
    ReservationListDeprecated,

    /// 121 - Print Edition doesn't match Master Edition
    #[error("")]
    PrintEditionDoesNotMatchMasterEdition,

    /// 122 - Edition Number greater than max supply
    #[error("")]
    EditionNumberGreaterThanMaxSupply,

    /// 123 - Must unverify before migrating collections.
    #[error("")]
    MustUnverify,

    /// 124 - Invalid Escrow Account Bump Seed
    #[error("")]
    InvalidEscrowBumpSeed,

    /// 125 - Must be Escrow Authority
    #[error("")]
    MustBeEscrowAuthority,

    /// 126 - Invalid System Program
    #[error("")]
    InvalidSystemProgram,

    /// 127 - Must be a Non Fungible Token
    #[error("")]
    MustBeNonFungible,

    /// 128 - Insufficient tokens for transfer
    #[error("")]
    InsufficientTokens,

    /// 129 - Borsh Serialization Error
    #[error("")]
    BorshSerializationError,

    /// 130 - Cannot create NFT with no Freeze Authority.
    #[error("")]
    NoFreezeAuthoritySet,

    /// 131
    #[error("")]
    InvalidCollectionSizeChange,

    /// 132
    #[error("")]
    InvalidBubblegumSigner,
    /// 133
    #[error("")]
    EscrowParentHasDelegate,

    /// 134
    #[error("")]
    MintIsNotSigner,

    /// 135
    #[error("")]
    InvalidTokenStandard,

    /// 136
    #[error("")]
    InvalidMintForTokenStandard,

    /// 137
    #[error("")]
    InvalidAuthorizationRules,

    /// 138
    #[error("")]
    MissingAuthorizationRules,

    /// 139
    #[error("")]
    MissingProgrammableConfig,

    /// 140
    #[error("")]
    InvalidProgrammableConfig,

    /// 141
    #[error("")]
    DelegateAlreadyExists,

    /// 142
    #[error("")]
    DelegateNotFound,

    /// 143
    #[error("")]
    MissingAccountInBuilder,

    /// 144
    #[error("")]
    MissingArgumentInBuilder,

    /// 145
    #[error("")]
    FeatureNotSupported,

    /// 146
    #[error("")]
    InvalidSystemWallet,

    /// 147
    #[error("")]
    OnlySaleDelegateCanTransfer,

    /// 148
    #[error("")]
    MissingTokenAccount,

    /// 149
    #[error("")]
    MissingSplTokenProgram,

    /// 150
    #[error("")]
    MissingAuthorizationRulesProgram,

    /// 151
    #[error("")]
    InvalidDelegateRoleForTransfer,

    /// 152
    #[error("")]
    InvalidTransferAuthority,

    /// 153
    #[error("")]
    InstructionNotSupported,

    /// 154
    #[error("")]
    KeyMismatch,

    /// 155
    #[error("")]
    LockedToken,

    /// 156
    #[error("")]
    UnlockedToken,

    /// 157
    #[error("")]
    MissingDelegateRole,

    /// 158
    #[error("")]
    InvalidAuthorityType,

    /// 159
    #[error("")]
    MissingTokenRecord,

    /// 160
    #[error("")]
    MintSupplyMustBeZero,

    /// 161
    #[error("")]
    DataIsEmptyOrZeroed,

    /// 162
    #[error("")]
    MissingTokenOwnerAccount,

    /// 163
    #[error("")]
    InvalidMasterEditionAccountLength,

    /// 164
    #[error("")]
    IncorrectTokenState,

    /// 165
    #[error("")]
    InvalidDelegateRole,

    /// 166
    #[error("")]
    MissingPrintSupply,

    /// 167
    #[error("")]
    MissingMasterEditionAccount,

    /// 168
    #[error("")]
    AmountMustBeGreaterThanZero,

    /// 169
    #[error("")]
    InvalidDelegateArgs,

    /// 170
    #[error("")]
    MissingLockedTransferAddress,

    /// 171
    #[error("")]
    InvalidLockedTransferAddress,

    /// 172
    #[error("")]
    DataIncrementLimitExceeded,

    /// 173
    #[error("")]
    CannotUpdateAssetWithDelegate,

    /// 174
    #[error("")]
    InvalidAssociatedTokenAccountProgram,

    /// 175
    #[error("")]
    InvalidInstructionsSysvar,

    /// 175
    #[error("Authority cannot apply all update args")]
    InvalidUpdateArgs,
}

impl PrintProgramError for MetadataError {
    fn print<E>(&self) {
        msg!("{:?}", &self);
    }
}

impl From<MetadataError> for ProgramError {
    fn from(e: MetadataError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for MetadataError {
    fn type_of() -> &'static str {
        "Metadata Error"
    }
}
