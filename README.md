# Marinade Staking Integration

This project demonstrates integration with the Marinade Staking protocol on Solana using Anchor and TypeScript. It includes a test that deposits SOL, receives mSOL, and then performs a unstake.

## Transaction Hashes

- Marinade Deposit Tx:  
  `UrQ9obSpdhw8GhmChUpxZZDhuFmviw2XxtBYAx3pB2G8bPCUYmzcrWHgn3zDKM3nummiiYmBNYUV6tTdfh2gZ4u`
- Marinade Unstake Tx:  
  `2bZP9H1TVVgfr3FfbySUuYFDgwkYWEf7kr5rUbJTPzwwP99kWewtJZaQHyHJKDGN9PdZtDuMe7Ht8zZNYTRUdkHA`

## Versions Used

- Solana CLI: `1.18.8`
- Anchor: `0.29.0`
- anchor-spl: `0.29.0`
- spl-token: `4.0.0`
- spl-associated-token-account: `2.0.0`
- Node.js: `22.19.0`

## Commands

### 1. Clean the build artifacts

```bash
anchor clean
```

### 2. Generate a new keypair

```bash
solana-keygen new --outfile ./target/deploy/marinadestaking-keypair.json --force
```

### 3. Sync program keys

```bash
anchor keys sync
```

### 4. Build the program

```bash
anchor build
```

### 5. Deploy the program

```bash
anchor deploy
```

> **After deploying, update the `programId` in `tests/marinadestaking.ts` with the new deployed Program ID.**

### 6. Run the tests

```bash
anchor test --skip-deploy
```

## Notes

- The test script is located at `tests/marinadestaking.ts`.
- The project is configured to use the Solana Devnet.
- The Anchor test command uses a custom script defined in `Anchor.toml`.
