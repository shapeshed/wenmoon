# Wen moon?

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/shapeshed/wenmoon/release.yml)
![Crates.io Version](https://img.shields.io/crates/v/wenmoon)

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

Obtain a [CoinMarketCap API key][1]. Create a file at
`~/.config/wenmoon/config.yml` with the following format. Ticker values can be
found by searching on [CoinMarketCap][2].

`amount` and `entry_price` are optional fields if you want to compute the value
and P&L of your position.

```yaml
api_key: [YOUR_API_KEY_HERE]
portfolio:
  - ticker: AAVE
    amount: 100.02
    entry_price: 100.02
  - ticker: MATIC
    amount: 0.643
    entry_price: 1.22
  - ticker: SOL
    amount: 98.45
    entry_price: 150
  - ticker: OSMO
    amount: 265.84
    entry_price: 1.45
  - ticker: KUJI
    amount: 1053.34
    entry_price: 3.95
```

<!-- dprint-ignore-start -->
> [!TIP] 
> If you would like to change the location of the config file pass the `-c` argument with the location of the config file.
<!-- dprint-ignore-end -->

## Usage

Vanilla usage. Will read file from `~/.config/wenmoon/config.yml`

```sh
wenmoon
```

Custom config file. Pass the `-c` flag followed by the path to the file

```sh
wenmoon -c ./path/to/config.yml
```

Multiple portfolios and watchlists using different files

```sh
wenmoon -c ~/.config/wenmoon/watchlist.yml
wenmoon -c ~/.config/wenmoon/shitcoins.yml
wenmoon -c ~/.config/wenmoon/vegas_fund.yml
```

If your list is long and you just want to see a single price `wenmoon` plays
nice with UNIX tools such as `grep` or `ripgrep`.

```sh
wenmoon | grep BTC
wenmoon | rg BTC
```

[1]: https://coinmarketcap.com/api/
[2]: https://coinmarketcap.com/
