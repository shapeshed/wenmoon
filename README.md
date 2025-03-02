# Wen moon?

<!-- dprint-ignore-start -->
[![GitHub Actions CI Workflow Status][1]][2]
[![GitHub Actions Release Workflow Status][3]][4] 
[![Crates.io Version][5]][6]
<!-- dprint-ignore-end -->

Your crypto portfolio and terminal together at last :handshake:

![wen moon?](doc/wenmoon-opt.gif)

## Installation

### From cargo

```sh
cargo install wenmoon --locked
```

### Build from source

```
git clone https://github.com/shapeshed/wenmoon
cd wenmoon
cargo install --path .
```

## Configuration

The following are supported as data providers

- [CoinMarketCap][7]
- [CoinGecko][9]

### CoinMarketCap

Obtain a [CoinMarketCap API key][8]. Create a file at
`~/.config/wenmoon/config.toml`. Ticker values can be found by searching on
[CoinMarketCap][7].

`amount` and `entry_price` are optional fields if you want to compute the value
and P&L of your position.

```toml
[coinmarketcap]
api_key = "YOUR_API_KEY_HERE"

[[portfolio]]
ticker = "AAVE"
amount = "100.02"
entry_price = "100.02"

[[portfolio]]
ticker = "MATIC"
amount = "0.643"
entry_price = "1.22"

[[portfolio]]
ticker = "SOL"
amount = "98.45"
entry_price = "150"

[[portfolio]]
ticker = "OSMO"
amount = "265.84"
entry_price = "1.45"

[[portfolio]]
ticker = "KUJI"
amount = "1053.34"
entry_price = "3.95"
```

### CoinGecko

Obtain a [CoinGecko API key][10]. Create a file at
`~/.config/wenmoon/config.toml`. Ticker values can be found by searching for the
coin on [CoinGecko][9] and finding the API ID field under Info.

`amount` and `entry_price` are optional fields if you want to compute the value
and P&L of your position.

```toml
[coingecko]
api_key = "YOUR_API_KEY_HERE"

[[portfolio]]
ticker = "aave"
amount = "100.02"

[[portfolio]]
entry_price = "100.02"
ticker = "matic-network"
amount = "0.643"

[[portfolio]]
entry_price = "1.22"
ticker = "solana"
amount = "98.45"
entry_price = "150"

[[portfolio]]
ticker = "osmosis"
amount = "265.84"
entry_price = "1.45"

[[portfolio]]
ticker = "kujira"
amount = "1053.34"
entry_price = "3.95"
```

## Usage

Vanilla usage. Will read file from `~/.config/wenmoon/config.toml`

```sh
wenmoon
```

Custom config file. Pass the `-c` flag followed by the path to the file

```sh
wenmoon -c ./path/to/custom/config.toml
```

Multiple portfolios and watchlists using different files

```sh
wenmoon -c ~/.config/wenmoon/watchlist.toml
wenmoon -c ~/.config/wenmoon/shitcoins.toml
wenmoon -c ~/.config/wenmoon/vegas_fund.toml
```

Sorting the list on hourly, daily, weekly and monthly change

```sh
wenmoon -s h
wenmoon -s d
wenmoon -s w
wenmoon -s m
```

If your list is long and you just want to see a single price `wenmoon` plays
nice with UNIX tools such as `grep` or `ripgrep`.

```sh
wenmoon | grep BTC
wenmoon | rg BTC
```

[1]: https://img.shields.io/github/actions/workflow/status/shapeshed/wenmoon/ci.yml?style=for-the-badge&label=ci
[2]: https://github.com/shapeshed/wenmoon/actions/workflows/ci.yml
[3]: https://img.shields.io/github/actions/workflow/status/shapeshed/wenmoon/release.yml?style=for-the-badge&label=release
[4]: https://github.com/shapeshed/wenmoon/releases
[5]: https://img.shields.io/crates/v/wenmoon?style=for-the-badge
[6]: https://crates.io/crates/wenmoon
[7]: https://coinmarketcap.com/
[8]: https://coinmarketcap.com/api/
[9]: https://www.coingecko.com/
[10]: https://www.coingecko.com/en/api
