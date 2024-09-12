# BlinkBet

## Project Overview

This project implements a decentralized betting system on the Solana blockchain, allowing users to place bets on asset prices using USDC as collateral. The system leverages blockchain links (blinks) from Dialect, enabling users to interact with the betting protocol directly through URL interfaces, streamlining the user experience and reducing friction.

## Key Features

- Decentralized betting on asset prices (SPL tokens and Solana)
- Integration with Dialect's blinks for seamless user interactions
- USDC collateral held in escrow contracts
- Dynamic price thresholds and betting periods
- Automated price fetching using Pyth oracle
- User-friendly claiming process for winnings

## System Architecture

### Smart Contracts (Solana Programs)

1. **Betting Program**
   - Manage bet placements
   - Handle escrow of USDC collateral
   - Implement claiming logic for winnings

2. **Oracle Integration**
   - Interface with Pyth for real-time price data
   - Ensure reliable and tamper-resistant price feeds

### Off-chain Components

1. **Blink URL Generator**
   - Create encoded URLs for bet actions
   - Include relevant bet parameters (asset, amount, threshold)

2. **Frontend Application**
   - Decode blink URLs and display bet information
   - Provide intuitive UI for placing bets and claiming winnings
   - Integrate with Solana wallet adapters for transaction signing

3. **Backend Service**
   - Process signed transactions from the frontend
   - Interact with Solana programs to execute bets and claims
   - Manage user accounts and bet history

4. **Serverless Functions**
   - Handle blink URL requests
   - Perform necessary computations and data fetching

## Betting Process Flow

1. User accesses the betting interface via a blink URL
2. Frontend decodes the URL and displays bet parameters
3. User reviews and confirms the bet
4. Frontend facilitates transaction signing
5. Signed transactions are sent to the backend for processing
6. Solana program escrows the USDC and records the bet
7. After the betting period, users can claim winnings if eligible

## Technical Stack

- Solana blockchain and Solana Program Library (SPL)
- Rust for Solana program development
- Anchor framework for smart contract development
- TypeScript/JavaScript for frontend and backend development
- NextJS for the user interface
- Dialect SDK for blinks integration

## Development Roadmap

1. **Phase 1: Core Infrastructure**
   - Develop and test Solana betting program
   - Implement basic blinks functionality
   - Create minimal viable frontend and backend

2. **Phase 2: Enhanced Functionality**
   - Integrate Pyth oracle for price feeds
   - Implement dynamic betting parameters
   - Develop comprehensive frontend UI

3. **Phase 3: Security and Optimization**
   - Conduct security audits
   - Optimize gas usage and transaction flow
   - Implement advanced error handling and recovery mechanisms

4. **Phase 4: User Experience and Scaling**
   - Refine blinks integration for seamless UX
   - Implement analytics and monitoring
   - Prepare for high-volume betting scenarios

## Getting Started

### Prerequisites

- Rust and Cargo (latest stable version)
- Solana CLI tools (v1.16 or later)
- Node.js (v14 or later) and npm
- Anchor CLI (v0.28.0 or later)

### Installation and Local Development

1. Clone the repository:
   ```
   git clone https://github.com/sheek801/EncodeSolanaDevBootcampFinalProject
   cd EncodeSolanaDevBootcampFinalProject
   ```

2. Install dependencies:
   ```
   npm install
   ```

3. Build the Solana program:
   ```
   anchor build
   ```

4. Start a local Solana validator:
   ```
   solana-test-validator
   ```

5. Deploy the program locally:
   ```
   anchor deploy
   ```

6. Run tests:
   ```
   anchor test
   ```

## Contributing

We welcome contributions to BlinkBet! Please read our contributing guidelines before submitting pull requests.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Disclaimer

This project is for educational and experimental purposes only. Always consult with legal and financial experts before implementing or using blockchain-based betting systems.
