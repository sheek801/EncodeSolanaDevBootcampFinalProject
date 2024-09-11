# EncodeSolanaDevBootcampFinalProject
Final project for solana development bootcamp

# Solana Betting Program

This project implements a betting system on the Solana blockchain, allowing users to place bets on asset prices using USDC as collateral.

## Features

- Place bets on asset prices
- Use USDC as collateral
- Automatic price fetching using Pyth oracle
- Claim winnings after bet expiry

## Prerequisites

- Rust and Cargo (latest stable version)
- Solana CLI tools (v1.16 or later)
- Node.js (v14 or later) and npm
- Anchor CLI (v0.28.0 or later)

## Installation and Local Development

1. Clone the repository:
   ```
   git clone https://github.com/your-username/solana-betting-program.git
   cd solana-betting-program
   ```

2. Install dependencies:
   ```
   npm install
   ```

3. Build the program:
   ```
   anchor build
   ```

4. Start a local Solana validator:
   ```
   solana-test-validator
   ```

5. Deploy the program to the local validator:
   ```
   anchor deploy
   ```

6. Run tests:
   ```
   anchor test
   ```

## Deploying to Devnet

1. Configure your Solana CLI to use devnet:
   ```
   solana config set --url https://api.devnet.solana.com
   ```

2. Create a `.env` file in the project root with your private key:
   ```
   DEPLOYER_PRIVATE_KEY=your_private_key_here
   ```

3. Create a script `deploy.js` in the project root:

   ```javascript
   const anchor = require("@project-serum/anchor");
   const { Keypair } = require("@solana/web3.js");
   require('dotenv').config();

   async function main() {
     const connection = new anchor.web3.Connection("https://api.devnet.solana.com");
     const wallet = new anchor.Wallet(Keypair.fromSecretKey(
       Buffer.from(JSON.parse(process.env.DEPLOYER_PRIVATE_KEY))
     ));
     
     const provider = new anchor.AnchorProvider(connection, wallet, {
       preflightCommitment: "confirmed",
     });
     
     const idl = JSON.parse(
       require("fs").readFileSync("./target/idl/solana_betting_program.json", "utf8")
     );
     
     const programId = new anchor.web3.PublicKey("your_program_id_here");
     
     const program = new anchor.Program(idl, programId, provider);
     
     console.log("Program ID:", programId.toString());
   }

   main().then(
     () => process.exit(),
     err => {
       console.error(err);
       process.exit(-1);
     },
   );
   ```

4. Install dotenv:
   ```
   npm install dotenv
   ```

5. Run the deployment script:
   ```
   node deploy.js
   ```

6. The script will output your Program ID. Save this ID for future use.

## Finding Your Program ID

After deploying your program, you can find your Program ID in several ways:

1. It's printed in the console output after running `anchor deploy` or the `deploy.js` script.

2. You can find it in the `target/deploy/solana_betting_program-keypair.json` file. To view it, run:
   ```
   solana address -k target/deploy/solana_betting_program-keypair.json
   ```

3. If you've lost the keypair file, you can find your recently deployed programs by running:
   ```
   solana address -k ~/.config/solana/id.json
   solana program show --programs
   ```
   Look for your program in the list of recently deployed programs.

Remember to update the `programId` in your client code and Anchor.toml file with the new Program ID whenever you redeploy the program.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
