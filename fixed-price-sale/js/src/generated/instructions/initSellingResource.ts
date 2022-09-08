/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as splToken from '@solana/spl-token';
import * as beet from '@metaplex-foundation/beet';
import * as web3 from '@solana/web3.js';

/**
 * @category Instructions
 * @category InitSellingResource
 * @category generated
 */
export type InitSellingResourceInstructionArgs = {
  masterEditionBump: number;
  vaultOwnerBump: number;
  maxSupply: beet.COption<beet.bignum>;
};
/**
 * @category Instructions
 * @category InitSellingResource
 * @category generated
 */
export const initSellingResourceStruct = new beet.FixableBeetArgsStruct<
  InitSellingResourceInstructionArgs & {
    instructionDiscriminator: number[] /* size: 8 */;
  }
>(
  [
    ['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['masterEditionBump', beet.u8],
    ['vaultOwnerBump', beet.u8],
    ['maxSupply', beet.coption(beet.u64)],
  ],
  'InitSellingResourceInstructionArgs',
);
/**
 * Accounts required by the _initSellingResource_ instruction
 *
 * @property [] store
 * @property [_writable_, **signer**] admin
 * @property [_writable_, **signer**] sellingResource
 * @property [] sellingResourceOwner
 * @property [] resourceMint
 * @property [] masterEdition
 * @property [] metadata
 * @property [_writable_] vault
 * @property [] owner
 * @property [_writable_] resourceToken
 * @category Instructions
 * @category InitSellingResource
 * @category generated
 */
export type InitSellingResourceInstructionAccounts = {
  store: web3.PublicKey;
  admin: web3.PublicKey;
  sellingResource: web3.PublicKey;
  sellingResourceOwner: web3.PublicKey;
  resourceMint: web3.PublicKey;
  masterEdition: web3.PublicKey;
  metadata: web3.PublicKey;
  vault: web3.PublicKey;
  owner: web3.PublicKey;
  resourceToken: web3.PublicKey;
  rent?: web3.PublicKey;
  tokenProgram?: web3.PublicKey;
  systemProgram?: web3.PublicKey;
  anchorRemainingAccounts?: web3.AccountMeta[];
};

export const initSellingResourceInstructionDiscriminator = [56, 15, 222, 211, 147, 205, 4, 145];

/**
 * Creates a _InitSellingResource_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category InitSellingResource
 * @category generated
 */
export function createInitSellingResourceInstruction(
  accounts: InitSellingResourceInstructionAccounts,
  args: InitSellingResourceInstructionArgs,
  programId = new web3.PublicKey('SaLeTjyUa5wXHnGuewUSyJ5JWZaHwz3TxqUntCE9czo'),
) {
  const [data] = initSellingResourceStruct.serialize({
    instructionDiscriminator: initSellingResourceInstructionDiscriminator,
    ...args,
  });
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.store,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.admin,
      isWritable: true,
      isSigner: true,
    },
    {
      pubkey: accounts.sellingResource,
      isWritable: true,
      isSigner: true,
    },
    {
      pubkey: accounts.sellingResourceOwner,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.resourceMint,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.masterEdition,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.metadata,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.vault,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.owner,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.resourceToken,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.rent ?? web3.SYSVAR_RENT_PUBKEY,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.tokenProgram ?? splToken.TOKEN_PROGRAM_ID,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.systemProgram ?? web3.SystemProgram.programId,
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
