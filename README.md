# worm-miner

## Burn

`worm-miner burn --network sepolia/anvil/mainnet --private-key 0x --amount 1.0 --reveal 0.5 --broadcaster-fee 0.1 --sell-on-uniswap 0.1 --receiver-address 0x.. --prover-fee 0.01`

* `--network`: Optional (Default: `mainnet`)
* `--private-key`: Required (The private key of the account which performs the burn)
* `--amount`: Required (The amount we want to send to the burn-address)
* `--reveal`: Optional (Default: maximum, same as `--amount`) (You can partially reveal the burned amount as BETH and encrypt the rest in a note file)
* `--broadcaster-fee`: Optional (Default: 0)
* `--sell-on-uniswap`: Optional (Default: 0) Part of the reveal amount can be sold in exchange of ETH
* `--receiver_address`: Required (Address) user will get BETH on this address
* `--prover-fee`: Default is `0` in case you want to prove it yourself

The burn info (Burn-key, amount etc.) is stored in `burn.json` in case of failure.
The remaining coin (`--amount` - `--reveal`) will be saved as a note in a JSON file.

## Mint

Minting BETH

`worm-miner mint --broadcaster 0x1234... burn.json`

* `--broadcaster`: Required (It can be a http endpoint `https://relayer.worm.cx/relay` (If we want someone else to broadcast for us, see the Relay section) or a private key `0x...` (If we want to broadcast ourself with another private key))


## Spend

`worm-miner spend --note note.json --amount 0.1`

Creates a new note file for the remaining amount (E.g note2.json)

## Participate

`worm-miner [COMMON OPTS] participate  --num-epochs 10 --amount-per-epoch 0.1`

* `--num-epochs`: Participate in N next epochs
* `--amount-per-epoch`: Put X BETH per epochs

Creates a participation file: `participate_10_0.1.json`

## Claim

`worm-miner [COMMON OPTS] claim participate_*.json`

Claim all input participations.

## Relay

Spins up a HTTP server, generates proofs and broadcasts them on behalf of others.

* GET `/proof` returns minimum proving fee of the relayer.
* POST `/proof` gets inputs of the proof-of-burn zk circuit and starts proving.
* GET `/proof/{burn-addres}` gets cached proof for the given burn-address.
* GET `/relay` returns minimum broadcasting fee of the relayer
* POST `/relay` gets inputs of a `mintCoin()` transaction and submits on behalf of you.
