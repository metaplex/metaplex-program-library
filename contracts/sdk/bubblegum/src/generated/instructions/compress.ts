/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as splToken from '@solana/spl-token'
import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'

/**
 * @category Instructions
 * @category Compress
 * @category generated
 */
export const compressStruct = new beet.BeetArgsStruct<{
  instructionDiscriminator: number[] /* size: 8 */
}>(
  [['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)]],
  'CompressInstructionArgs'
)
/**
 * Accounts required by the _compress_ instruction
 *
 * @property [] authority
 * @property [] merkleSlab
 * @property [**signer**] owner
 * @property [] delegate
 * @property [_writable_] tokenAccount
 * @property [_writable_] mint
 * @property [_writable_] metadata
 * @property [_writable_] masterEdition
 * @property [_writable_, **signer**] payer
 * @property [] tokenMetadataProgram
 * @property [] candyWrapper
 * @property [] gummyrollProgram
 * @category Instructions
 * @category Compress
 * @category generated
 */
export type CompressInstructionAccounts = {
  authority: web3.PublicKey
  merkleSlab: web3.PublicKey
  owner: web3.PublicKey
  delegate: web3.PublicKey
  tokenAccount: web3.PublicKey
  mint: web3.PublicKey
  metadata: web3.PublicKey
  masterEdition: web3.PublicKey
  payer: web3.PublicKey
  tokenMetadataProgram: web3.PublicKey
  candyWrapper: web3.PublicKey
  gummyrollProgram: web3.PublicKey
}

export const compressInstructionDiscriminator = [
  82, 193, 176, 117, 176, 21, 115, 253,
]

/**
 * Creates a _Compress_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @category Instructions
 * @category Compress
 * @category generated
 */
export function createCompressInstruction(
  accounts: CompressInstructionAccounts
) {
  const {
    authority,
    merkleSlab,
    owner,
    delegate,
    tokenAccount,
    mint,
    metadata,
    masterEdition,
    payer,
    tokenMetadataProgram,
    candyWrapper,
    gummyrollProgram,
  } = accounts

  const [data] = compressStruct.serialize({
    instructionDiscriminator: compressInstructionDiscriminator,
  })
  const keys: web3.AccountMeta[] = [
    {
      pubkey: authority,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: merkleSlab,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: owner,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: delegate,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: tokenAccount,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: mint,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: metadata,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: masterEdition,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: payer,
      isWritable: true,
      isSigner: true,
    },
    {
      pubkey: web3.SystemProgram.programId,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: tokenMetadataProgram,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: splToken.TOKEN_PROGRAM_ID,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: candyWrapper,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: gummyrollProgram,
      isWritable: false,
      isSigner: false,
    },
  ]

  const ix = new web3.TransactionInstruction({
    programId: new web3.PublicKey(
      'BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY'
    ),
    keys,
    data,
  })
  return ix
}
