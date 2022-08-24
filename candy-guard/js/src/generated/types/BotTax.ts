/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
export type BotTax = {
  lamports: beet.bignum
  lastInstruction: boolean
}

/**
 * @category userTypes
 * @category generated
 */
export const botTaxBeet = new beet.BeetArgsStruct<BotTax>(
  [
    ['lamports', beet.u64],
    ['lastInstruction', beet.bool],
  ],
  'BotTax'
)
