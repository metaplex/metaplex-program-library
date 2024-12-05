# Metaplex Program Library

Metaplex smart contracts and SDK.

[![Program Tests](https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/program.yml/badge.svg)](https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/program.yml)
[![Integration Tests](https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/integration.yml/badge.svg)](https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/integration.yml)
[![SDK Tests](https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/sdk.yml/badge.svg)](https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/sdk.yml)

## Metaplex Contracts

| Name                                       | Rust Crate                                                                | npm Package                                                            |
|:-------------------------------------------|:--------------------------------------------------------------------------|------------------------------------------------------------------------|
| [Auction House](./auction-house)           | [![Crate][mpl-auction-house-img-long]][mpl-auction-house-crate]           | [![NPM][mpl-auction-house-nimg-long]][mpl-auction-house-npm]           |
| [Auctioneer](./auctioneer)                 | [![Crate][mpl-auctioneer-img-long]][mpl-auctioneer-crate]                 | [![NPM][mpl-auctioneer-nimg-long]][mpl-auctioneer-npm]                 |
| [Bubblegum](./bubblegum)                   | [![Crate][mpl-bubblegum-img-long]][mpl-bubblegum-crate]                   | [![NPM][mpl-bubblegum-nimg-long]][mpl-bubblegum-npm]                   |
| [Candy Machine Core](./candy-machine-core) | [![Crate][mpl-candy-machine-core-img-long]][mpl-candy-machine-core-crate] | [![NPM][mpl-candy-machine-core-nimg-long]][mpl-candy-machine-core-npm] |
| [Candy Machine](./candy-machine)           | [![Crate][mpl-candy-machine-img-long]][mpl-candy-machine-crate]           | [![NPM][mpl-candy-machine-nimg-long]][mpl-candy-machine-npm]           |
| [Fixed Price Sale](./fixed-price-sale)     | [![Crate][mpl-fixed-price-sale-img-long]][mpl-fixed-price-sale-crate]     | [![NPM][mpl-fixed-price-sale-nimg-long]][mpl-fixed-price-sale-npm]     |
| [Gumdrop](./gumdrop)                       | [![Crate][mpl-gumdrop-img-long]][mpl-gumdrop-crate]                       | [![NPM][mpl-gumdrop-nimg-long]][mpl-gumdrop-npm]                       |
| [Hydra](./hydra)                           | [![Crate][mpl-hydra-img-long]][mpl-hydra-crate]                           | [![NPM][mpl-hydra-nimg-long]][mpl-hydra-npm]                           |
| [NFT Packs](./nft-packs)                   | [![Crate][mpl-nft-packs-img-long]][mpl-nft-packs-crate]                   | [![NPM][mpl-nft-packs-nimg-long]][mpl-nft-packs-npm]                   |
| [Token Entangler](./token-entangler)       | [![Crate][mpl-token-entangler-img-long]][mpl-token-entangler-crate]       | [![NPM][mpl-token-entangler-nimg-long]][mpl-token-entangler-npm]       |
| [Token Metadata](./token-metadata)         | [![Crate][mpl-token-metadata-img-long]][mpl-token-metadata-crate]         | [![NPM][mpl-token-metadata-nimg-long]][mpl-token-metadata-npm]         |

## Development

### Setting up Rust Tests

Run the `build.sh` script with the name of the program to build the shared object and put it in a directory
called `test-programs` in the root of the project.

E.g.:

```bash
./build.sh auction-house
```

Running with `all` builds all programs.

### Versioning and Publishing Packages

Smart contract SDK packages are versioned independently since a contract isn't necessarily coupled
to other contracts.

We use the following `(pre|post)(version|publish)` npm scripts to manage related checks, tagging,
committing and pushing the version bump.

- `preversion`: ensures that the package builds and its tests pass
- `postversion`: adds and commits the version bump and adds a tag indicating package name and new
  version, i.e. `@metaplex-foundation/mp-core@v0.0.1`
- `prepublishOnly`: ensures that the package builds and its tests pass again (just to be _really_ sure)
- `postpublish`: pushes the committed change and new tag to GitHub

In order to version and then publish a package just run the following commands from the folder of
the package you want to update:

- `npm version <patch|minor|major>`
- `npm publish`

As you note if version + publish succeeds the scripts end up pushing those updates to the master
branch. Therefore, please ensure to be on and up to date `master` branch before running them. Please
**don't ever publish from another branch** but only from the main one with only PR approved changes
merged.

### Rust Crates

| Package            | Link                                                   | Version                                                              |
|:-------------------|:-------------------------------------------------------|:---------------------------------------------------------------------|
| Auction House      | [mpl-auction-house][mpl-auction-house-crate]           | [![Crate][mpl-auction-house-img]][mpl-auction-house-crate]           |
| Auctioneer         | [mpl-auctioneer][mpl-auctioneer-crate]                 | [![Crate][mpl-auctioneer-img]][mpl-auctioneer-crate]                 |
| Bubblegum          | [mpl-bubblegum][mpl-bubblegum-crate]                   | [![Crate][mpl-bubblegum-img]][mpl-bubblegum-crate]                   |
| Candy Machine Core | [mpl-candy-machine-core][mpl-candy-machine-core-crate] | [![Crate][mpl-candy-machine-core-img]][mpl-candy-machine-core-crate] |
| Testing Utils      | [mpl-testing-utils][mpl-testing-utils-crate]           | [![Crate][mpl-testing-utils-img]][mpl-testing-utils-crate]           |
| Utils              | [mpl-utils][mpl-utils-crate]                           | [![Crate][mpl-utils-img]][mpl-utils-crate]                           |
| Fixed Price Sale   | [mpl-fixed-price-sale][mpl-fixed-price-sale-crate]     | [![Crate][mpl-fixed-price-sale-img]][mpl-fixed-price-sale-crate]     |
| Gumdrop            | [mpl-gumdrop][mpl-gumdrop-crate]                       | [![Crate][mpl-gumdrop-img]][mpl-gumdrop-crate]                       |
| Hydra              | [mpl-hydra][mpl-hydra-crate]                           | [![Crate][mpl-hydra-img]][mpl-hydra-crate]                           |
| NFT Packs          | [mpl-nft-packs][mpl-nft-packs-crate]                   | [![Crate][mpl-nft-packs-img]][mpl-nft-packs-crate]                   |
| Token Entangler    | [mpl-token-entangler][mpl-token-entangler-crate]       | [![Crate][mpl-token-entangler-img]][mpl-token-entangler-crate]       |
| Token Metadata     | [mpl-token-metadata][mpl-token-metadata-crate]         | [![Crate][mpl-token-metadata-img]][mpl-token-metadata-crate]         |

### npm Packages

| Package            | Link                                                 | Version                                                           |
|:-------------------|:-----------------------------------------------------|:------------------------------------------------------------------|
| Auction House      | [mpl-auction-house][mpl-auction-house-npm]           | [![NPM][mpl-auction-house-nimg]][mpl-auction-house-npm]           |
| Auctioneer         | [mpl-auctioneer][mpl-auctioneer-npm]                 | [![NPM][mpl-auctioneer-nimg]][mpl-auctioneer-npm]                 |
| Bubblegum          | [mpl-bubblegum][mpl-bubblegum-npm]                   | [![NPM][mpl-bubblegum-nimg]][mpl-bubblegum-npm]                   |
| Candy Machine Core | [mpl-candy-machine-core][mpl-candy-machine-core-npm] | [![NPM][mpl-candy-machine-core-nimg]][mpl-candy-machine-core-npm] |
| Candy Machine      | [mpl-candy-machine][mpl-candy-machine-npm]           | [![NPM][mpl-candy-machine-nimg]][mpl-candy-machine-npm]           |
| Fixed Price Sale   | [mpl-fixed-price-sale][mpl-fixed-price-sale-npm]     | [![NPM][mpl-fixed-price-sale-nimg]][mpl-fixed-price-sale-npm]     |
| Gumdrop            | [mpl-gumdrop][mpl-gumdrop-npm]                       | [![NPM][mpl-gumdrop-nimg]][mpl-gumdrop-npm]                       |
| Hydra              | [mpl-hydra][mpl-hydra-npm]                           | [![NPM][mpl-hydra-nimg]][mpl-hydra-npm]                           |
| NFT Packs          | [mpl-nft-packs][mpl-nft-packs-npm]                   | [![NPM][mpl-nft-packs-nimg]][mpl-nft-packs-npm]                   |
| Token Entangler    | [mpl-token-entangler][mpl-token-entangler-npm]       | [![NPM][mpl-token-entangler-nimg]][mpl-token-entangler-npm]       |
| Token Metadata     | [mpl-token-metadata][mpl-token-metadata-npm]         | [![NPM][mpl-token-metadata-nimg]][mpl-token-metadata-npm]         |

## Reporting security issues

To report a security issue, please follow the guidance on the [SECURITY](.github/SECURITY.md) page.

## License

The Rust/Cargo programs are licensed under the
“Apache-style” [Metaplex(TM) NFT Open Source License][metaplex-nft-license] and the JS/TS client libraries are licensed
under either the [MIT][mit-license] or the [Apache][apache-license] licenses.


<!-- ===================================== -->
<!-- Links for badges and such shown above -->
<!-- Please add any links you add to the   -->
<!-- readme here instead of inlining them  -->
<!-- ===================================== -->

<!-- Workflow Status Badges -->

[integration-tests-yml]:https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/integration.yml
[integration-tests-svg]:https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/integration.yml/badge.svg
[program-tests-yml]:https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/program.yml
[program-tests-svg]:https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/program.yml/badge.svg
[sdk-tests-yml]:https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/sdk.yml
[sdk-tests-svg]:https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/sdk.yml/badge.svg

<!-- Crates -->

[mpl-auction-house-crate]:https://crates.io/crates/mpl-auction-house
[mpl-auctioneer-crate]:https://crates.io/crates/mpl-auctioneer
[mpl-bubblegum-crate]:https://crates.io/crates/mpl-bubblegum
[mpl-candy-machine-core-crate]:https://crates.io/crates/mpl-candy-machine-core
[mpl-candy-machine-crate]:https://crates.io/crates/mpl-candy-machine
[mpl-fixed-price-sale-crate]:https://crates.io/crates/mpl-fixed-price-sale
[mpl-utils-crate]:https://crates.io/crates/mpl-utils
[mpl-testing-utils-crate]:https://crates.io/crates/mpl-testing-utils
[mpl-gumdrop-crate]:https://crates.io/crates/mpl-gumdrop
[mpl-hydra-crate]:https://crates.io/crates/mpl-hydra
[mpl-nft-packs-crate]:https://crates.io/crates/mpl-nft-packs
[mpl-token-entangler-crate]:https://crates.io/crates/mpl-token-entangler
[mpl-token-metadata-crate]:https://crates.io/crates/mpl-token-metadata

[mpl-auction-house-img-long]:https://img.shields.io/crates/v/mpl-auction-house?label=crates.io%20%7C%20mpl-auction-house&logo=rust
[mpl-auction-house-img]:https://img.shields.io/crates/v/mpl-auction-house?logo=rust

[mpl-auctioneer-img-long]:https://img.shields.io/crates/v/mpl-auctioneer?label=crates.io%20%7C%20mpl-auctioneer&logo=rust
[mpl-auctioneer-img]:https://img.shields.io/crates/v/mpl-auctioneer?logo=rust

[mpl-bubblegum-img-long]:https://img.shields.io/crates/v/mpl-bubblegum?label=crates.io%20%7C%20mpl-bubblegum&logo=rust
[mpl-bubblegum-img]:https://img.shields.io/crates/v/mpl-bubblegum?logo=rust

[mpl-candy-machine-core-img-long]:https://img.shields.io/crates/v/mpl-candy-machine-core?label=crates.io%20%7C%20mpl-candy-machine-core&logo=rust
[mpl-candy-machine-core-img]:https://img.shields.io/crates/v/mpl-candy-machine-core?logo=rust

[mpl-candy-machine-img-long]:https://img.shields.io/crates/v/mpl-candy-machine?label=crates.io%20%7C%20mpl-candy-machine&logo=rust
[mpl-candy-machine-img]:https://img.shields.io/crates/v/mpl-candy-machine?logo=rust

[mpl-fixed-price-sale-img-long]:https://img.shields.io/crates/v/mpl-fixed-price-sale?label=crates.io%20%7C%20mpl-fixed-price-sale&logo=rust
[mpl-fixed-price-sale-img]:https://img.shields.io/crates/v/mpl-fixed-price-sale?logo=rust

[mpl-utils-img-long]:https://img.shields.io/crates/v/mpl-utils?label=crates.io%20%7C%20mpl-utils&logo=rust
[mpl-utils-img]:https://img.shields.io/crates/v/mpl-utils?logo=rust

[mpl-testing-utils-img-long]:https://img.shields.io/crates/v/mpl-testing-utils?label=crates.io%20%7C%20mpl-testing-utils&logo=rust
[mpl-testing-utils-img]:https://img.shields.io/crates/v/mpl-testing-utils?logo=rust

[mpl-gumdrop-img-long]:https://img.shields.io/crates/v/mpl-gumdrop?label=crates.io%20%7C%20mpl-gumdrop&logo=rust
[mpl-gumdrop-img]:https://img.shields.io/crates/v/mpl-gumdrop?logo=rust

[mpl-hydra-img-long]:https://img.shields.io/crates/v/mpl-hydra?label=crates.io%20%7C%20mpl-hydra&logo=rust
[mpl-hydra-img]:https://img.shields.io/crates/v/mpl-hydra?logo=rust

[mpl-nft-packs-img-long]:https://img.shields.io/crates/v/mpl-nft-packs?label=crates.io%20%7C%20mpl-nft-packs&logo=rust
[mpl-nft-packs-img]:https://img.shields.io/crates/v/mpl-nft-packs?logo=rust

[mpl-token-entangler-img-long]:https://img.shields.io/crates/v/mpl-token-entangler?label=crates.io%20%7C%20mpl-token-entangler&logo=rust
[mpl-token-entangler-img]:https://img.shields.io/crates/v/mpl-token-entangler?logo=rust

[mpl-token-metadata-img-long]:https://img.shields.io/crates/v/mpl-token-metadata?label=crates.io%20%7C%20mpl-token-metadata&logo=rust
[mpl-token-metadata-img]:https://img.shields.io/crates/v/mpl-token-metadata?logo=rust

<!-- NPM Packages -->

[mpl-auction-house-npm]:https://www.npmjs.com/package/@metaplex-foundation/mpl-auction-house
[mpl-auctioneer-npm]:https://www.npmjs.com/package/@metaplex-foundation/mpl-auctioneer
[mpl-bubblegum-npm]:https://www.npmjs.com/package/@metaplex-foundation/mpl-bubblegum
[mpl-candy-machine-core-npm]:https://www.npmjs.com/package/@metaplex-foundation/mpl-candy-machine-core
[mpl-candy-machine-npm]:https://www.npmjs.com/package/@metaplex-foundation/mpl-candy-machine
[mpl-fixed-price-sale-npm]:https://www.npmjs.com/package/@metaplex-foundation/mpl-fixed-price-sale
[mpl-core-npm]:https://www.npmjs.com/package/@metaplex-foundation/mpl-core
[mpl-gumdrop-npm]:https://www.npmjs.com/package/@metaplex-foundation/mpl-gumdrop
[mpl-hydra-npm]:https://www.npmjs.com/package/@metaplex-foundation/mpl-hydra
[mpl-nft-packs-npm]:https://www.npmjs.com/package/@metaplex-foundation/mpl-nft-packs
[mpl-token-entangler-npm]:https://www.npmjs.com/package/@metaplex-foundation/mpl-token-entangler
[mpl-token-metadata-npm]:https://www.npmjs.com/package/@metaplex-foundation/mpl-token-metadata

[mpl-auction-house-nimg-long]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-auction-house?label=npm%20%7C%20mpl-auction-house&logo=typescript
[mpl-auction-house-nimg]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-auction-house?logo=typescript

[mpl-auctioneer-nimg-long]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-auctioneer?label=npm%20%7C%20mpl-auctioneer&logo=typescript
[mpl-auctioneer-nimg]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-auctioneer?logo=typescript

[mpl-bubblegum-nimg-long]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-bubblegum?label=npm%20%7C%20mpl-bubblegum&logo=typescript
[mpl-bubblegum-nimg]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-bubblegum?logo=typescript

[mpl-candy-machine-core-nimg-long]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-candy-machine-core?label=npm%20%7C%20mpl-candy-machine-core&logo=typescript
[mpl-candy-machine-core-nimg]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-candy-machine-core?logo=typescript

[mpl-candy-machine-nimg-long]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-candy-machine?label=npm%20%7C%20mpl-candy-machine&logo=typescript
[mpl-candy-machine-nimg]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-candy-machine?logo=typescript

[mpl-fixed-price-sale-nimg-long]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-fixed-price-sale?label=npm%20%7C%20mpl-fixed-price-sale&logo=typescript
[mpl-fixed-price-sale-nimg]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-fixed-price-sale?logo=typescript

[mpl-gumdrop-nimg-long]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-gumdrop?label=npm%20%7C%20mpl-gumdrop&logo=typescript
[mpl-gumdrop-nimg]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-gumdrop?logo=typescript

[mpl-hydra-nimg-long]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-hydra?label=npm%20%7C%20mpl-hydra&logo=typescript
[mpl-hydra-nimg]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-hydra?logo=typescript

[mpl-nft-packs-nimg-long]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-nft-packs?label=npm%20%7C%20mpl-nft-packs&logo=typescript
[mpl-nft-packs-nimg]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-nft-packs?logo=typescript

[mpl-token-entangler-nimg-long]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-token-entangler?label=npm%20%7C%20mpl-token-entangler&logo=typescript
[mpl-token-entangler-nimg]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-token-entangler?logo=typescript

[mpl-token-metadata-nimg-long]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-token-metadata?label=npm%20%7C%20mpl-token-metadata&logo=typescript
[mpl-token-metadata-nimg]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-token-metadata?logo=typescript

<!-- Licenses -->

[metaplex-nft-license]:  https://github.com/metaplex-foundation/metaplex-program-library/blob/master/LICENSE

[apache-license]: https://www.apache.org/licenses/LICENSE-2.0.txt

[mit-license]: https://www.mit.edu/~amini/LICENSE.md
 
