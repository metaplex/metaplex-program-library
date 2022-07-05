/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet';
import * as web3 from '@solana/web3.js';

/**
 * @category Instructions
 * @category RemoveCollection
 * @category generated
 */
export const removeCollectionStruct = new beet.BeetArgsStruct<{
  instructionDiscriminator: number[] /* size: 8 */;
}>(
  [['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)]],
  'RemoveCollectionInstructionArgs',
);
/**
 * Accounts required by the _removeCollection_ instruction
 *
 * @property [_writable_] candyMachine
 * @property [**signer**] authority
 * @property [_writable_] collectionPda
 * @property [] metadata
 * @property [] mint
 * @property [_writable_] collectionAuthorityRecord
 * @property [] tokenMetadataProgram
 * @category Instructions
 * @category RemoveCollection
 * @category generated
 */
export type RemoveCollectionInstructionAccounts = {
  candyMachine: web3.PublicKey;
  authority: web3.PublicKey;
  collectionPda: web3.PublicKey;
  metadata: web3.PublicKey;
  mint: web3.PublicKey;
  collectionAuthorityRecord: web3.PublicKey;
  tokenMetadataProgram: web3.PublicKey;
};

export const removeCollectionInstructionDiscriminator = [223, 52, 106, 217, 61, 220, 36, 160];

/**
 * Creates a _RemoveCollection_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @category Instructions
 * @category RemoveCollection
 * @category generated
 */
export function createRemoveCollectionInstruction(
  accounts: RemoveCollectionInstructionAccounts,
  programId = new web3.PublicKey('cndy3Z4yapfJBmL3ShUp5exZKqR3z33thTzeNMm2gRZ'),
) {
  const [data] = removeCollectionStruct.serialize({
    instructionDiscriminator: removeCollectionInstructionDiscriminator,
  });
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.candyMachine,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.authority,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: accounts.collectionPda,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.metadata,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.mint,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.collectionAuthorityRecord,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.tokenMetadataProgram,
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
