# worm-miner

## Burn

`worm-miner burn --network sepolia/anvil/mainnet --private-key 0x --amount 1.0 --reveal 0.5 --broadcaster-fee 0.1 --broadcaster 0x --sell-on-uniswap 0.1`

* `--network`: Optional (Default: `mainnet`)
* `--private-key`: Required (The private key of the account which performs the burn)
* `--amount`: Required (The amount we want to send to the burn-address)
* `--reveal`: Optional (Default: maximum, same as `--amount`) (You can partially reveal the burned amount as BETH and encrypt the rest in a note file)
* `--broadcaster-fee`: Optional (Default: 0)
* `--broadcaster`: Required (It can be a http endpoint `https://relayer.worm.cx/broadcast` (If we want someone else to broadcast for us) or a private key `0x...` (If we want to broadcast ourself with another private key))
* `--sell-on-uniswap`: Optional (Default: 0) Part of the reveal amount can be sold in exchange of ETH

The burn info (Burn-key, amount etc.) is stored in `burn.json` in case of failure.
The remaining coin (`--amount` - `--reveal`) will be saved as a note in a JSON file.

## Recover

In case the proving/minting fails along the way, you can recover:

`worm-miner recover burn.json`

## Spend

`worm-miner spend --note note.json --amount 0.1`

Creates a new note file for the remaining amount (E.g note2.json)

## Participate

`worm-miner participate [COMMON OPTS] --num-epochs 10 --amount-per-epoch 0.1`

* `--num-epochs`: Participate in N next epochs
* `--amount-per-epoch`: Put X BETH per epochs

Creates a participation file: `participate_10_0.1.json`

## Claim

`worm-miner participate [COMMON OPTS] participate_*.json

Claim all input participations.
