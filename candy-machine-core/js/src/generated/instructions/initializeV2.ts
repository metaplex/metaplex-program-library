/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet';
import * as web3 from '@solana/web3.js';
import { CandyMachineData, candyMachineDataBeet } from '../types/CandyMachineData';

/**
 * @category Instructions
 * @category InitializeV2
 * @category generated
 */
export type InitializeV2InstructionArgs = {
  data: CandyMachineData;
  tokenStandard: number;
};
/**
 * @category Instructions
 * @category InitializeV2
 * @category generated
 */
export const initializeV2Struct = new beet.FixableBeetArgsStruct<
  InitializeV2InstructionArgs & {
    instructionDiscriminator: number[] /* size: 8 */;
  }
>(
  [
    ['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['data', candyMachineDataBeet],
    ['tokenStandard', beet.u8],
  ],
  'InitializeV2InstructionArgs',
);
/**
 * Accounts required by the _initializeV2_ instruction
 *
 * @property [_writable_] candyMachine
 * @property [_writable_] authorityPda
 * @property [] authority
 * @property [**signer**] payer
 * @property [] ruleSet (optional)
 * @property [_writable_] collectionMetadata
 * @property [] collectionMint
 * @property [] collectionMasterEdition
 * @property [_writable_, **signer**] collectionUpdateAuthority
 * @property [_writable_] collectionDelegateRecord
 * @property [] tokenMetadataProgram
 * @property [] sysvarInstructions
 * @property [] authorizationRulesProgram (optional)
 * @property [] authorizationRules (optional)
 * @category Instructions
 * @category InitializeV2
 * @category generated
 */
export type InitializeV2InstructionAccounts = {
  candyMachine: web3.PublicKey;
  authorityPda: web3.PublicKey;
  authority: web3.PublicKey;
  payer: web3.PublicKey;
  ruleSet?: web3.PublicKey;
  collectionMetadata: web3.PublicKey;
  collectionMint: web3.PublicKey;
  collectionMasterEdition: web3.PublicKey;
  collectionUpdateAuthority: web3.PublicKey;
  collectionDelegateRecord: web3.PublicKey;
  tokenMetadataProgram: web3.PublicKey;
  systemProgram?: web3.PublicKey;
  sysvarInstructions: web3.PublicKey;
  authorizationRulesProgram?: web3.PublicKey;
  authorizationRules?: web3.PublicKey;
  anchorRemainingAccounts?: web3.AccountMeta[];
};

export const initializeV2InstructionDiscriminator = [67, 153, 175, 39, 218, 16, 38, 32];

/**
 * Creates a _InitializeV2_ instruction.
 *
 * Optional accounts that are not provided default to the program ID since
 * this was indicated in the IDL from which this instruction was generated.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category InitializeV2
 * @category generated
 */
export function createInitializeV2Instruction(
  accounts: InitializeV2InstructionAccounts,
  args: InitializeV2InstructionArgs,
  programId = new web3.PublicKey('CndyV3LdqHUfDLmE5naZjVN8rBZz4tqhdefbAnjHG3JR'),
) {
  const [data] = initializeV2Struct.serialize({
    instructionDiscriminator: initializeV2InstructionDiscriminator,
    ...args,
  });
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.candyMachine,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.authorityPda,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.authority,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.payer,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: accounts.ruleSet ?? programId,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.collectionMetadata,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.collectionMint,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.collectionMasterEdition,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.collectionUpdateAuthority,
      isWritable: true,
      isSigner: true,
    },
    {
      pubkey: accounts.collectionDelegateRecord,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.tokenMetadataProgram,
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
    {
      pubkey: accounts.authorizationRulesProgram ?? programId,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.authorizationRules ?? programId,
      isWritable: false,
      isSigner: false,
    },
  ];

  if (accounts.anchorRemainingAccounts != null) {
    for (const acc of accounts.anchorRemainingAccounts) {
      keys.push(acc);
    }
  }

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  });
  return ix;
}
