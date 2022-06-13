/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet';
/**
 * @category enums
 * @category generated
 */
export enum AuthorityScope {
  Deposit,
  Buy,
  PublicBuy,
  ExecuteSale,
  Sell,
  Cancel,
  Withdraw,
}

/**
 * @category userTypes
 * @category generated
 */
export const authorityScopeBeet = beet.fixedScalarEnum(AuthorityScope) as beet.FixedSizeBeet<
  AuthorityScope,
  AuthorityScope
>;
