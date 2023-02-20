/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet';
import * as web3 from '@solana/web3.js';
import { VerifyArgs, verifyArgsBeet } from '../types/VerifyArgs';

/**
 * @category Instructions
 * @category Verify
 * @category generated
 */
export type VerifyInstructionArgs = {
  verifyArgs: VerifyArgs;
};
/**
 * @category Instructions
 * @category Verify
 * @category generated
 */
export const VerifyStruct = new beet.BeetArgsStruct<
  VerifyInstructionArgs & {
    instructionDiscriminator: number;
  }
>(
  [
    ['instructionDiscriminator', beet.u8],
    ['verifyArgs', verifyArgsBeet],
  ],
  'VerifyInstructionArgs',
);
/**
 * Accounts required by the _Verify_ instruction
 *
 * @property [**signer**] authority Creator to verify, collection owner or delegate
 * @property [] delegateRecord (optional) Delegate record PDA
 * @property [_writable_] metadata Metadata account
 * @property [] collectionMint (optional) Mint of the Collection
 * @property [_writable_] collectionMetadata (optional) Metadata Account of the Collection
 * @property [] collectionMasterEdition (optional) Master Edition Account of the Collection Token
 * @property [] sysvarInstructions Instructions sysvar account
 * @category Instructions
 * @category Verify
 * @category generated
 */
export type VerifyInstructionAccounts = {
  authority: web3.PublicKey;
  delegateRecord?: web3.PublicKey;
  metadata: web3.PublicKey;
  collectionMint?: web3.PublicKey;
  collectionMetadata?: web3.PublicKey;
  collectionMasterEdition?: web3.PublicKey;
  systemProgram?: web3.PublicKey;
  sysvarInstructions: web3.PublicKey;
};

export const verifyInstructionDiscriminator = 52;

/**
 * Creates a _Verify_ instruction.
 *
 * Optional accounts that are not provided default to the program ID since
 * this was indicated in the IDL from which this instruction was generated.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category Verify
 * @category generated
 */
export function createVerifyInstruction(
  accounts: VerifyInstructionAccounts,
  args: VerifyInstructionArgs,
  programId = new web3.PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s'),
) {
  const [data] = VerifyStruct.serialize({
    instructionDiscriminator: verifyInstructionDiscriminator,
    ...args,
  });
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.authority,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: accounts.delegateRecord ?? programId,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.metadata,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.collectionMint ?? programId,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.collectionMetadata ?? programId,
      isWritable: accounts.collectionMetadata != null,
      isSigner: false,
    },
    {
      pubkey: accounts.collectionMasterEdition ?? programId,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.systemProgram ?? web3.SystemProgram.programId,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.sysvarInstructions,
      isWritable: false,
      isSigner: false,
    },
  ];

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  });
  return ix;
}
