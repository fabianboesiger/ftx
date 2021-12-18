# FTX API

Unofficial Rust API bindings for the FTX exchange.

[crates.io](https://crates.io/crates/ftx) |
[docs.rs](https://docs.rs/ftx/latest/ftx/index.html) |
[FTX API Documentation](https://docs.ftx.com/#overview)

## Progress
Work in progress, contributions are welcome.

### REST
- [x] Authentication
- [x] Subaccounts
	- [x] Get all subaccounts
	- [x] Create subaccount
	- [x] Change subaccount name
	- [x] Delete subaccount
	- [x] Get subaccount balances
	- [x] Transfer between subaccounts
- [x] Markets
	- [x] Get markets
	- [x] Get single market
	- [x] Get orderbook
	- [x] Get trades
	- [x] Get historical prices
- [ ] Futures
	- [x] List all futures
	- [x] Get future
	- [x] Get future stats
	- [x] Get funding rates
	- [ ] Get index weights
	- [x] Get expired futures
	- [ ] Get historical index
- [ ] Account
	- [x] Get account information
	- [x] Get positions
	- [x] Change account leverage
- [ ] Wallet
	- [x] Get coins
	- [x] Get balances
	- [ ] Get balances of all accounts
	- [x] Get deposit address
	- [x] Get deposit history
	- [ ] Get withdrawal history
	- [ ] Request withdrawal
	- [ ] Get airdrops
	- [ ] Get withdrawal fees
	- [ ] Get saved addresses
	- [ ] Create saved addresses
	- [ ] Delete saved addresses
- [ ] Orders
	- [x] Get open orders
	- [x] Get order history
	- [ ] Get open trigger orders
	- [ ] Get trigger order triggers
	- [ ] Get trigger order history
	- [x] Place order
	- [ ] Place trigger order
	- [x] Modify order
	- [x] Modify order by client ID
	- [ ] Modify trigger order
	- [x] Get order status
	- [x] Get order status by client ID
	- [x] Cancel order
	- [x] Cancel order by client ID
	- [ ] Cancel open trigger order
	- [x] Cancel all orders
- [ ] Fills
- [ ] Funding Payments
- [ ] Leveraged Tokens
- [ ] Options
- [ ] Staking
- [ ] Convert
- [ ] Spot Margin
	- [ ] Get lending history
	- [ ] Get borrow rates
	- [x] Get lending rates
	- [ ] Get daily borrowed amounts
	- [ ] Get market info
	- [ ] Get my borrow history
	- [x] Get my lending history
	- [ ] Get lending offers
	- [x] Get lending info
	- [x] Submit lending offer
- [ ] NFTs
- [ ] FTXPay

### Websockets
- [ ] Public Channels
	- [ ] Ticker
	- [ ] Markets
	- [x] Trades
	- [x] Orderbooks
		- [x] Verify checksum
	- [ ] Grouped Orderbooks
- [x] Private Channels
	- [x] Fills
	- [x] Orders

## Installation

The minimal supported Rust version is 1.54.

Add the following line to your Cargo.toml file:
```
ftx = "0.3"
```
Or for the latest github version:
```
ftx = { git = "https://github.com/fabianboesiger/ftx", branch = "main" }
```

## Usage

### Rate Limiting
Using the FTX API requires rate-limiting requests to no more than 30 requests per second in order to avoid HTTP 429 errors. You will need to rate-limit your own requests in your usage of this library.

See the [FTX API Documentation](https://docs.ftx.com/#rate-limits)

### Pagination
If needed, you will need to paginate your own requests in your usage of this library.
See the [FTX API Documentation](https://docs.ftx.com/#pagination) and [sample Python code](https://github.com/ftexchange/ftx/blob/master/rest/client.py#L163)

### REST Usage Examples

- [Query the price](https://docs.rs/ftx/latest/ftx/rest/struct.Rest.html#method.get_market) of BTC/USD: `examples/btc_price.rs`
- [Get account info](https://docs.rs/ftx/latest/ftx/rest/struct.Rest.html#method.get_account): `examples/get_accounts.rs`
- [Get markets](https://docs.rs/ftx/latest/ftx/rest/struct.Rest.html#method.get_markets): `rest::tests::get_markets`
- [Get futures](https://docs.rs/ftx/latest/ftx/rest/struct.Rest.html#method.get_futures): `rest::tests::get_futures`
- [Get orderbook](https://docs.rs/ftx/latest/ftx/rest/struct.Rest.html#method.get_orderbook): `rest::tests::get_orderbook`
- [Get trades](https://docs.rs/ftx/latest/ftx/rest/struct.Rest.html#method.get_trades): `rest::tests::get_trades`
- [Get historical prices](https://docs.rs/ftx/latest/ftx/rest/struct.Rest.html#method.get_historical_prices): `rest::tests::get_historical_prices`
- [Placing](https://docs.rs/ftx/latest/ftx/rest/struct.Rest.html#method.place_order), [modifying](https://docs.rs/ftx/latest/ftx/rest/struct.Rest.html#method.modify_order), and [cancelling](https://docs.rs/ftx/latest/ftx/rest/struct.Rest.html#method.cancel_order) orders: `rest::tests::place_modify_cancel_orders`

### Websockets Usage Examples

- Listen to latest [Trade](https://docs.rs/ftx/latest/ftx/ws/struct.Trade.html)s and [Orderbook](https://docs.rs/ftx/latest/ftx/ws/struct.Orderbook.html) updates: `examples/watch_market.rs`
- [Subscribe](https://docs.rs/ftx/0.3.1/ftx/ws/struct.Ws.html#method.subscribe) and [unsubscribe](https://docs.rs/ftx/0.3.1/ftx/ws/struct.Ws.html#method.unsubscribe_all) from [Channel](https://docs.rs/ftx/latest/ftx/ws/enum.Channel.html)s: `ws::tests::subscribe_unsubscribe`
- [Update](https://docs.rs/ftx/0.3.1/ftx/ws/struct.Orderbook.html#method.update) an [Orderbook](https://docs.rs/ftx/latest/ftx/ws/struct.Orderbook.html): `ws::tests::order_book_update`
- Verify `OrderBook` [checksums](https://docs.rs/ftx/latest/ftx/ws/struct.Orderbook.html#method.verify_checksum): `ws::tests::order_book_checksum`
- Use the [Orderbook](https://docs.rs/ftx/latest/ftx/ws/struct.Orderbook.html) convenience [methods](https://docs.rs/ftx/latest/ftx/ws/struct.Orderbook.html#implementations): `ws::tests::order_book_helpers`
	- `bid_price`, `ask_price`, `mid_price`
	- `best_bid`, `best_ask`, `best_bid_and_ask`
	- `quote`
- Listen for [Fill](https://docs.rs/ftx/latest/ftx/ws/struct.Fill.html)s: `ws::tests::fills`
- Listen for [Order](https://docs.rs/ftx/latest/ftx/rest/struct.OrderInfo.html) updates: `ws::tests::orders`
