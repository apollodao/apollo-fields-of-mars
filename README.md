# Martian Field

Martian Field is a leveraged yield farming strategy utilizing contract-to-contract (C2C) lending from [Mars Protocol](https://twitter.com/mars_protocol).

## Overview

A common type of yield farms in the Terra ecosystem works as follows. The user provides a _primary asset_ (e.g. ANC, Anchor Protocol's governance token) and equal value of UST to an AMM pool (e.g. Terraswap), then deposit the AMM's liquidity token into a staking contract. Over time, staking reward is accrued in the form of the primary asset (ANC in this case) and withdrawable by the user.

To reinvest the farming gains, the user needs to

1. claim staking reward
2. sell half of the reward to UST
3. provide the UST and the other half of the reward to the AMM
4. deposit the liquidity token to the staking contract

**Martian Field** is an autocompounder that 1) automates this process, and 2) allow user to take up to 2x leverage utilizing C2C lending from Mars protocol.

Martian Field also tracks each user's loan-to-value ratio (LTV). If a user's LTV exceeds a preset threshold, typically as a result of the primary asset's price falling or debt builds up too quickly, the position is subject to liquidation.

## Development

### Dependencies

- Rust v1.44.1+
- `wasm32-unknown-unknown` target
- Docker
- [LocalTerra](https://github.com/terra-project/LocalTerra)
- Node.js v16

### Envrionment Setup

1. Install `rustup` via https://rustup.rs/

2. Add `wasm32-unknown-unknown` target

```sh
rustup default stable
rustup target add wasm32-unknown-unknown
```

3. Install [Docker](https://www.docker.com/)

4. Clone the [LocalTerra](https://github.com/terra-project/LocalTerra#usage) repository, edit `config/genesis.json` as follows. Set the stability fee ("tax") to zero by:

```diff
"app_state": {
  "treasury": {
    "params": {
      "tax_policy": {
-       "rate_min": "0.000500000000000000",
-       "rate_max": "0.010000000000000000",
+       "rate_min": "0.000000000000000000",
+       "rate_max": "0.000000000000000000",
      },
-     "change_rate_max": "0.000250000000000000"
+     "change_rate_max": "0.000000000000000000"
    }
  }
}
```

5. Optionally, [speed up LocalTerra's blocktime](https://github.com/terra-project/LocalTerra#pro-tip-speed-up-block-time) by changing `config/config.toml` as follows:

```diff
##### consensus configuration options #####
[consensus]

wal_file = "data/cs.wal/wal"
- timeout_propose = "3s"
- timeout_propose_delta = "500ms"
- timeout_prevote = "1s"
- timeout_prevote_delta = "500ms"
- timeout_precommit_delta = "500ms"
- timeout_commit = "5s"
+ timeout_propose = "200ms"
+ timeout_propose_delta = "200ms"
+ timeout_prevote = "200ms"
+ timeout_prevote_delta = "200ms"
+ timeout_precommit_delta = "200ms"
+ timeout_commit = "200ms"
```

6. Install Node, preferrably using [nvm](https://github.com/nvm-sh/nvm#installing-and-updating), as well as libraries required for testing:

```bash
nvm install 16
nvm alias default 16
cd fields-of-mars/scripts
npm install
```

### Compile

Make sure the current working directory is set to the root directory of this repository, then

```bash
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.11.5
```

### Test

Start LocalTerra:

```bash
cd /path/to/LocalTerra
git checkout main
git pull
docker compose up
```

Run test scripts: inside `scripts` folder,

```bash
ts-node tests/1_mock_astro_generator.test.ts
ts-node tests/2_mock_oracle.test.ts
ts-node tests/3_mock_red_bank.test.ts
ts-node tests/4_martian_field.test.ts
```

### Deploy

Provide seed phrase of the deployer account in `scripts/.env`; create an `instantiate_msg.json` storing the contract's instantiate message; then

```bash
ts-node 1_deploy.ts --network mainnet|testnet --msg /path/to/instantiate_msg.json [--code-id codeId]
```

### Notes

- LocalTerra [only works on X86 processors](https://github.com/terra-project/LocalTerra#requirements). There is currently no way to run the tests on Macs with the M1 processor.

- Our development setup includes the VSCode text editor, [rust-analyzer](https://github.com/rust-analyzer/rust-analyzer) for Rust, and ESLint + Prettier for TypeScript.

## Deployment

### Mainnet

| Contract               | Mainnet                                                                                                                                              | Testnet                                                                                                                                              |
| ---------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| LUNA-UST Pair          | [`terra1m6ywlgn6wrjuagcmmezzz2a029gtldhey5k552`](https://finder.extraterrestrial.money/mainnet/address/terra1m6ywlgn6wrjuagcmmezzz2a029gtldhey5k552) | [`terra12eq2zmdmycvx9n6skwpu9kqxts0787rekjnlwm`](https://finder.extraterrestrial.money/testnet/address/terra12eq2zmdmycvx9n6skwpu9kqxts0787rekjnlwm) |
| LUNA-UST LP            | [`terra1m24f7k4g66gnh9f7uncp32p722v0kyt3q4l3u5`](https://finder.extraterrestrial.money/mainnet/address/terra1m24f7k4g66gnh9f7uncp32p722v0kyt3q4l3u5) | [`terra1sjpns87xfa48hwy6pwqdchxzsrsmmewsxjwvcj`](https://finder.extraterrestrial.money/testnet/address/terra1sjpns87xfa48hwy6pwqdchxzsrsmmewsxjwvcj) |
| ANC Token              | [`terra14z56l0fp2lsf86zy3hty2z47ezkhnthtr9yq76`](https://finder.extraterrestrial.money/mainnet/address/terra14z56l0fp2lsf86zy3hty2z47ezkhnthtr9yq76) | [`terra1747mad58h0w4y589y3sk84r5efqdev9q4r02pc`](https://finder.extraterrestrial.money/testnet/address/terra1747mad58h0w4y589y3sk84r5efqdev9q4r02pc) |
| ANC-UST Pair           | [`terra1qr2k6yjjd5p2kaewqvg93ag74k6gyjr7re37fs`](https://finder.extraterrestrial.money/mainnet/address/terra1qr2k6yjjd5p2kaewqvg93ag74k6gyjr7re37fs) | [`terra12eq2zmdmycvx9n6skwpu9kqxts0787rekjnlwm`](https://finder.extraterrestrial.money/testnet/address/terra12eq2zmdmycvx9n6skwpu9kqxts0787rekjnlwm) |
| ANC-UST LP             | [`terra1wmaty65yt7mjw6fjfymkd9zsm6atsq82d9arcd`](https://finder.extraterrestrial.money/mainnet/address/terra1wmaty65yt7mjw6fjfymkd9zsm6atsq82d9arcd) | [`terra1sjpns87xfa48hwy6pwqdchxzsrsmmewsxjwvcj`](https://finder.extraterrestrial.money/testnet/address/terra1sjpns87xfa48hwy6pwqdchxzsrsmmewsxjwvcj) |
| MIR Token              | [`terra15gwkyepfc6xgca5t5zefzwy42uts8l2m4g40k6`](https://finder.extraterrestrial.money/mainnet/address/terra15gwkyepfc6xgca5t5zefzwy42uts8l2m4g40k6) | [`terra10llyp6v3j3her8u3ce66ragytu45kcmd9asj3u`](https://finder.extraterrestrial.money/testnet/address/terra10llyp6v3j3her8u3ce66ragytu45kcmd9asj3u) |
| MIR-UST Pair           | [`terra143xxfw5xf62d5m32k3t4eu9s82ccw80lcprzl9`](https://finder.extraterrestrial.money/mainnet/address/terra143xxfw5xf62d5m32k3t4eu9s82ccw80lcprzl9) | [`terra1cz6qp8lfwht83fh9xm9n94kj04qc35ulga5dl0`](https://finder.extraterrestrial.money/testnet/address/terra1cz6qp8lfwht83fh9xm9n94kj04qc35ulga5dl0) |
| MIR-UST LP             | [`terra17trxzqjetl0q6xxep0s2w743dhw2cay0x47puc`](https://finder.extraterrestrial.money/mainnet/address/terra17trxzqjetl0q6xxep0s2w743dhw2cay0x47puc) | [`terra1zrryfhlrpg49quz37u90ck6f396l4xdjs5s08j`](https://finder.extraterrestrial.money/testnet/address/terra1zrryfhlrpg49quz37u90ck6f396l4xdjs5s08j) |
| ASTRO Token            | [`terra1xj49zyqrwpv5k928jwfpfy2ha668nwdgkwlrg3`](https://finder.extraterrestrial.money/mainnet/address/terra1xj49zyqrwpv5k928jwfpfy2ha668nwdgkwlrg3) | [`terra1cc2up8erdqn2l7nz37qjgvnqy56sr38aj9vqry`](https://finder.extraterrestrial.money/testnet/address/terra1cc2up8erdqn2l7nz37qjgvnqy56sr38aj9vqry) |
| ASTRO-UST Pair         | [`terra1l7xu2rl3c7qmtx3r5sd2tz25glf6jh8ul7aag7`](https://finder.extraterrestrial.money/mainnet/address/terra1l7xu2rl3c7qmtx3r5sd2tz25glf6jh8ul7aag7) | [`terra1dk57pl4v4ut9kwsmtrv9k4kkn9fxrh290zvg2w`](https://finder.extraterrestrial.money/testnet/address/terra1dk57pl4v4ut9kwsmtrv9k4kkn9fxrh290zvg2w) |
| ASTRO-UST LP Token     | [`terra17n5sunn88hpy965mzvt3079fqx3rttnplg779g`](https://finder.extraterrestrial.money/mainnet/address/terra17n5sunn88hpy965mzvt3079fqx3rttnplg779g) | [`terra1uahqpnm4p3ag8ma40xhtft96uvuxy6vn9p6x9v`](https://finder.extraterrestrial.money/testnet/address/terra1uahqpnm4p3ag8ma40xhtft96uvuxy6vn9p6x9v) |
| Astro Generator        | [`terra1zgrx9jjqrfye8swykfgmd6hpde60j0nszzupp9`](https://finder.extraterrestrial.money/mainnet/address/terra1zgrx9jjqrfye8swykfgmd6hpde60j0nszzupp9) | [`terra1cmqhxgna6uasnycgdcx974uq8u56rp2ta3r356`](https://finder.extraterrestrial.money/testnet/address/terra1cmqhxgna6uasnycgdcx974uq8u56rp2ta3r356) |
| Mars Oracle            | TBD                                                                                                                                                  | [`terra1uxs9f90kr2lgt3tpkpyk5dllqrwra5tgwv0pc5`](https://finder.extraterrestrial.money/testnet/address/terra1uxs9f90kr2lgt3tpkpyk5dllqrwra5tgwv0pc5) |
| Mars Red Bank          | TBD                                                                                                                                                  | [`terra19fy8q4vx6uzv4rmhvvp329fgr5343qrunntq60`](https://finder.extraterrestrial.money/testnet/address/terra19fy8q4vx6uzv4rmhvvp329fgr5343qrunntq60) |
| Mars Treasury          | TBD                                                                                                                                                  | [`terra1u4sk8992wz4c9p5c8ckffj4h8vh97hfeyw9x5n`](https://finder.extraterrestrial.money/testnet/address/terra1u4sk8992wz4c9p5c8ckffj4h8vh97hfeyw9x5n) |
| Mars Governance        | TBD                                                                                                                                                  | [`terra1w0acggjar67f7l4phnvqzeg0na0k5fcn9lv5zz`](https://finder.extraterrestrial.money/testnet/address/terra1w0acggjar67f7l4phnvqzeg0na0k5fcn9lv5zz) |
| Martian Field LUNA-UST | TBD                                                                                                                                                  | [`terra1jx2qgq0vcvywy87aelw9l3n5l2sy62nzkuhl00`](https://finder.extraterrestrial.money/testnet/address/terra1jx2qgq0vcvywy87aelw9l3n5l2sy62nzkuhl00) |
| Martian Field ANC-UST  | TBD                                                                                                                                                  | [``](https://finder.extraterrestrial.money/testnet/address/)                                                                                         |
| Martian Field MIR-UST  | TBD                                                                                                                                                  | [``](https://finder.extraterrestrial.money/testnet/address/)                                                                                         |

## License

Contents of this repository are open source under [GNU General Public License v3](https://www.gnu.org/licenses/gpl-3.0.en.html) or later.
