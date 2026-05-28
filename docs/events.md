# SO4 Event Catalog

Every event emitted by every SO4 contract is listed here.
Frontend and indexer developers should be able to reconstruct complete protocol activity from these events alone.

---

## Notation

- **Topic** — the `Symbol` that appears as the first element of the event's topic tuple.  
  For `publish_event` events the topic is the string inside `#[contractevent(topics = ["..."])]`.  
  For `publish` events it is the argument to `symbol_short!(...)`.
- **Data fields** — the data tuple emitted alongside the topic, in order.
- All `BytesN<32>` values are sha-256 key hashes unless noted otherwise.
- `FLOAT_PRECISION = 10^30`; `TOKEN_PRECISION = 10^7`.

---

## role_store

### `init`
Emitted once, when the contract is initialised.

| Field | Type | Description |
|---|---|---|
| `admin` | `Address` | Account granted the initial admin role |
| `admin_role` | `BytesN<32>` | sha256 of "ROLE_ADMIN" |

**Indexer behaviour:** record the initial admin and the admin-role hash. All subsequent `grant` / `revoke` events should be filtered against known role hashes.

---

### `grant`
Emitted each time a role is granted to an account.

| Field | Type | Description |
|---|---|---|
| `account` | `Address` | Grantee |
| `role` | `BytesN<32>` | Role hash (see `libs/keys/src/lib.rs → roles::*`) |

---

### `revoke`
Emitted each time a role is revoked from an account.

| Field | Type | Description |
|---|---|---|
| `account` | `Address` | Account that lost the role |
| `role` | `BytesN<32>` | Role hash |

---

## data_store

### `init`
Emitted once, when the contract is initialised.

| Field | Type | Description |
|---|---|---|
| `role_store` | `Address` | Address of the paired role_store contract |

---

## oracle

### `prices`
Emitted each time a keeper successfully submits a price batch (both the verified `set_prices` and the simple `set_prices_simple` paths).

| Field | Type | Description |
|---|---|---|
| `caller` | `Address` | Keeper that submitted the prices |
| `count` | `u32` | Number of `TokenPrice` entries in the batch |

**Indexer behaviour:** use this event to detect price feed activity. The actual price values are stored in the oracle's temporary storage and expire per ledger; they are not in the event.

---

## market_factory

### `wasm_set`
Emitted when the market-token wasm hash is registered or updated.

| Field | Type | Description |
|---|---|---|
| `wasm_hash` | `BytesN<32>` | New wasm hash for market_token deployments |

---

### `mkt_new`
Emitted when a new market is created.

| Field | Type | Description |
|---|---|---|
| `market_token` | `Address` | Newly deployed LP token contract |
| `index_token` | `Address` | Synthetic price reference token |
| `long_token` | `Address` | Long-side collateral token |
| `short_token` | `Address` | Short-side collateral / stablecoin token |

**Indexer behaviour:** build the market registry from these events. Every subsequent event that references a `market` address refers to the `market_token` address emitted here.

---

## market_token

One market_token contract is deployed per market. All events below may originate from any of those contracts; index the emitting contract address to identify the market.

### `approve`
Emitted when an allowance is set.

| Field | Type | Description |
|---|---|---|
| `from` | `Address` | Token owner |
| `spender` | `Address` | Approved spender |
| `amount` | `i128` | Approved amount (TOKEN_PRECISION) |
| `expiration_ledger` | `u32` | Ledger number at which the approval expires |

---

### `transfer`
Emitted on direct transfer.

| Field | Type | Description |
|---|---|---|
| `from` | `Address` | Sender |
| `to` | `Address` | Recipient |
| `amount` | `i128` | Amount transferred (TOKEN_PRECISION) |

---

### `xfer_from`
Emitted on approved-spend transfer.

| Field | Type | Description |
|---|---|---|
| `spender` | `Address` | Account that consumed the allowance |
| `from` | `Address` | Token owner |
| `to` | `Address` | Recipient |
| `amount` | `i128` | Amount transferred (TOKEN_PRECISION) |

---

### `burn`
Emitted when LP tokens are burned directly by the holder.

| Field | Type | Description |
|---|---|---|
| `from` | `Address` | Holder whose tokens are burned |
| `amount` | `i128` | Amount burned (TOKEN_PRECISION) |

---

### `burn_from`
Emitted when LP tokens are burned via approved allowance.

| Field | Type | Description |
|---|---|---|
| `spender` | `Address` | Account that consumed the allowance |
| `from` | `Address` | Token owner |
| `amount` | `i128` | Amount burned (TOKEN_PRECISION) |

---

### `mint`
Emitted when the deposit_handler mints new LP tokens to a depositor.

| Field | Type | Description |
|---|---|---|
| `caller` | `Address` | deposit_handler contract address |
| `to` | `Address` | LP token recipient (depositor's receiver) |
| `amount` | `i128` | Amount minted (TOKEN_PRECISION) |

---

### `pool_out`
Emitted when the withdrawal_handler transfers underlying pool tokens out of the market contract.

| Field | Type | Description |
|---|---|---|
| `pool_token` | `Address` | Long or short token being returned |
| `receiver` | `Address` | Withdrawal receiver |
| `amount` | `i128` | Amount transferred (TOKEN_PRECISION) |

---

## deposit_handler

### `dep_crt`
Emitted when a two-step deposit is created and collateral is locked in the vault.

| Field | Type | Description |
|---|---|---|
| `key` | `BytesN<32>` | Unique deposit key (sha256 of "DEPOSIT" ‖ nonce) |
| `caller` | `Address` | User who created the deposit |
| `market` | `Address` | Target market_token address |

**Indexer behaviour:** track this key to correlate subsequent `dep_exe` or `dep_can` events.

---

### `dep_exe`
Emitted when a keeper successfully executes a pending deposit.

| Field | Type | Description |
|---|---|---|
| `key` | `BytesN<32>` | Deposit key (matches `dep_crt`) |
| `receiver` | `Address` | Account that received the LP tokens |
| `mint_amount` | `i128` | LP tokens minted (TOKEN_PRECISION) |

---

### `dep_can`
Emitted when a deposit is cancelled and collateral is refunded.

| Field | Type | Description |
|---|---|---|
| `key` | `BytesN<32>` | Deposit key (matches `dep_crt`) |
| `account` | `Address` | Depositor who was refunded |

---

## withdrawal_handler

### `wth_crt`
Emitted when a two-step withdrawal is created and LP tokens are locked in the vault.

| Field | Type | Description |
|---|---|---|
| `key` | `BytesN<32>` | Unique withdrawal key (sha256 of "WITHDRAWAL" ‖ nonce) |
| `caller` | `Address` | User who created the withdrawal |
| `market` | `Address` | Market_token address |

---

### `wth_exe`
Emitted when a keeper successfully executes a pending withdrawal.

| Field | Type | Description |
|---|---|---|
| `key` | `BytesN<32>` | Withdrawal key (matches `wth_crt`) |
| `receiver` | `Address` | Account that received the pool tokens |
| `long_out` | `i128` | Long token amount returned (TOKEN_PRECISION) |
| `short_out` | `i128` | Short token amount returned (TOKEN_PRECISION) |

---

### `wth_can`
Emitted when a withdrawal is cancelled and LP tokens are refunded.

| Field | Type | Description |
|---|---|---|
| `key` | `BytesN<32>` | Withdrawal key (matches `wth_crt`) |
| `account` | `Address` | Account that was refunded |

---

## order_handler

### `ord_crt`
Emitted when a new order is created.

| Field | Type | Description |
|---|---|---|
| `key` | `BytesN<32>` | Unique order key (sha256 of "ORDER" ‖ nonce) |
| `caller` | `Address` | Order creator |
| `market` | `Address` | Market_token address |

---

### `ord_exe`
Emitted when a keeper successfully executes a pending order.

| Field | Type | Description |
|---|---|---|
| `key` | `BytesN<32>` | Order key (matches `ord_crt`) |
| `account` | `Address` | Order owner |

---

### `ord_can`
Emitted when an order is cancelled.

| Field | Type | Description |
|---|---|---|
| `key` | `BytesN<32>` | Order key |
| `account` | `Address` | Order owner |

---

### `ord_upd`
Emitted when an order's trigger price, acceptable price, size, or min output is updated.

| Field | Type | Description |
|---|---|---|
| `key` | `BytesN<32>` | Order key |
| `caller` | `Address` | Account that performed the update (must equal order owner) |

---

### `ord_frz`
Emitted when a keeper marks an order as frozen (circuit breaker).

| Field | Type | Description |
|---|---|---|
| `key` | `BytesN<32>` | Order key |

---

### `liq_exe`
Emitted when a position is liquidated via `order_handler::liquidate_position`.

| Field | Type | Description |
|---|---|---|
| `account` | `Address` | Position owner |
| `market` | `Address` | Market_token address |
| `pnl_usd` | `i128` | Realised PnL (FLOAT_PRECISION, may be negative) |
| `execution_price` | `i128` | Execution price at which the liquidation closed (FLOAT_PRECISION) |

---

### `adl_exe`
Emitted when a position is partially closed by the ADL keeper via `order_handler::execute_adl`.

| Field | Type | Description |
|---|---|---|
| `account` | `Address` | Position owner |
| `market` | `Address` | Market_token address |
| `size_delta_usd` | `i128` | USD size that was reduced (FLOAT_PRECISION) |
| `pnl_usd` | `i128` | Realised PnL from the partial close (FLOAT_PRECISION) |

---

## liquidation_handler

### `liq_req`
Emitted when a liquidation keeper triggers a liquidation check that results in calling `order_handler::liquidate_position`.

| Field | Type | Description |
|---|---|---|
| `account` | `Address` | Position owner being liquidated |
| `market` | `Address` | Market_token address |
| `is_long` | `bool` | Whether the position is long |

**Indexer behaviour:** pair `liq_req` with the subsequent `liq_exe` from `order_handler` to get the full liquidation picture (trigger + outcome).

---

## adl_handler

### `adl_req`
Emitted when an ADL keeper triggers auto-deleveraging on a position.

| Field | Type | Description |
|---|---|---|
| `account` | `Address` | Position owner being deleveraged |
| `market` | `Address` | Market_token address |
| `is_long` | `bool` | Whether the position is long |
| `size_delta_usd` | `i128` | USD size to be reduced (FLOAT_PRECISION) |
| `pnl_usd` | `i128` | Position PnL at the time of ADL check (FLOAT_PRECISION) |

**Indexer behaviour:** pair `adl_req` with the subsequent `adl_exe` from `order_handler` to get the full ADL picture.

---

## fee_handler

### `fee_clm` (`FeeClaimed`)
Emitted when accrued protocol fees are claimed for a market.

| Field | Type | Description |
|---|---|---|
| `market` | `Address` | Market_token address |
| `token` | `Address` | Fee token (long or short token) |
| `amount` | `u128` | Amount claimed (TOKEN_PRECISION) |
| `receiver` | `Address` | Fee recipient |

---

### `fnd_clm` (`FundingFeeClaimed`)
Emitted when a trader claims their accumulated funding fee credit.

| Field | Type | Description |
|---|---|---|
| `account` | `Address` | Trader that claimed |
| `market` | `Address` | Market_token address |
| `token` | `Address` | Funding fee token |
| `amount` | `u128` | Amount claimed (TOKEN_PRECISION) |

---

## referral_storage

### `ref_reg` (`CodeRegistered`)
Emitted when a new referral code is registered.

| Field | Type | Description |
|---|---|---|
| `caller` | `Address` | Referrer who registered the code |
| `code` | `BytesN<32>` | sha256 of the referral code string |

---

### `ref_set` (`TraderCodeSet`)
Emitted when a trader links their account to a referral code.

| Field | Type | Description |
|---|---|---|
| `trader` | `Address` | Trader who set the code |
| `code` | `BytesN<32>` | Referral code sha256 hash |

---

## Indexer reconstruction guide

To reconstruct complete protocol activity from events alone:

1. **Market registry** — build from `mkt_new` events.
2. **Role registry** — build from `init` (role_store), `grant`, `revoke`.
3. **Pending deposits** — open on `dep_crt`, close on `dep_exe` or `dep_can`.
4. **Pending withdrawals** — open on `wth_crt`, close on `wth_exe` or `wth_can`.
5. **Open orders** — open on `ord_crt`, close on `ord_exe` or `ord_can`. `ord_upd` modifies fields; `ord_frz` marks as frozen.
6. **Position changes** — infer from `ord_exe` (increase/decrease), `liq_exe` (full close), and `adl_exe` (partial close). `liq_req` and `adl_req` are the keeper-side triggers.
7. **Fee activity** — `fee_clm` for protocol fees, `fnd_clm` for trader funding credits.
8. **LP token movements** — `mint`, `burn`, `burn_from`, `transfer`, `xfer_from` on each market_token contract. `pool_out` tracks pool-to-user transfers on withdrawal execution.
