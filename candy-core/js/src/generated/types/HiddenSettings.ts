/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
export type HiddenSettings = {
  name: string
  uri: string
  hash: number[] /* size: 32 */
}

/**
 * @category userTypes
 * @category generated
 */
export const hiddenSettingsBeet =
  new beet.FixableBeetArgsStruct<HiddenSettings>(
    [
      ['name', beet.utf8String],
      ['uri', beet.utf8String],
      ['hash', beet.uniformFixedSizeArray(beet.u8, 32)],
    ],
    'HiddenSettings'
  )
