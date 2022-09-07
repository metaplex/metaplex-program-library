/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'
import {
  CandyMachineData,
  candyMachineDataBeet,
} from '../types/CandyMachineData'

/**
 * @category Instructions
 * @category Initialize
 * @category generated
 */
export type InitializeInstructionArgs = {
  data: CandyMachineData
}
/**
 * @category Instructions
 * @category Initialize
 * @category generated
 */
export const initializeStruct = new beet.FixableBeetArgsStruct<
  InitializeInstructionArgs & {
    instructionDiscriminator: number[] /* size: 8 */
  }
>(
  [
    ['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['data', candyMachineDataBeet],
  ],
  'InitializeInstructionArgs'
)
/**
 * Accounts required by the _initialize_ instruction
 *
 * @property [_writable_] candyMachine
 * @property [] wallet
 * @property [] authority
 * @property [] updateAuthority
 * @property [**signer**] payer
 * @category Instructions
 * @category Initialize
 * @category generated
 */
export type InitializeInstructionAccounts = {
  candyMachine: web3.PublicKey
  wallet: web3.PublicKey
  authority: web3.PublicKey
  updateAuthority: web3.PublicKey
  payer: web3.PublicKey
  systemProgram?: web3.PublicKey
  rent?: web3.PublicKey
}

export const initializeInstructionDiscriminator = [
  175, 175, 109, 31, 13, 152, 155, 237,
]

/**
 * Creates a _Initialize_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category Initialize
 * @category generated
 */
export function createInitializeInstruction(
  accounts: InitializeInstructionAccounts,
  args: InitializeInstructionArgs,
  programId = new web3.PublicKey('cndy3CZK71ZHMp9ddpq5NVvQDx33o6cCYDf4JBAWCk7')
) {
  const [data] = initializeStruct.serialize({
    instructionDiscriminator: initializeInstructionDiscriminator,
    ...args,
  })
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.candyMachine,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.wallet,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.authority,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.updateAuthority,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.payer,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: accounts.systemProgram ?? web3.SystemProgram.programId,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.rent ?? web3.SYSVAR_RENT_PUBKEY,
      isWritable: false,
      isSigner: false,
    },
  ]

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  })
  return ix
}
