import express from 'express';
import { Connection, Transaction, PublicKey } from '@solana/web3.js';
import cors from 'cors';


// Approximation of the error function
function erf(x: number): number {
    // Constants
    const a1 = 0.254829592;
    const a2 = -0.284496736;
    const a3 = 1.421413741;
    const a4 = -1.453152027;
    const a5 = 1.061405429;
    const p = 0.3275911;

    // Save the sign of x
    const sign = (x >= 0) ? 1 : -1;
    x = Math.abs(x);

    // A&S formula 7.1.26
    const t = 1.0 / (1.0 + p * x);
    const y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * Math.exp(-x * x);

    return sign * y;
}

// Standard Normal Cumulative Distribution Function
function normalCDF(x: number): number {
    return 0.5 * (1 + erf(x / Math.sqrt(2)));
}

// Black-Scholes Call Option Price
function blackScholesCall(spot: number, strike: number, tenor: number, volatility: number, interestRate: number): number {
    const d1 = (Math.log(spot / strike) + (interestRate + 0.5 * volatility * volatility) * tenor) / (volatility * Math.sqrt(tenor));
    const d2 = d1 - volatility * Math.sqrt(tenor);

    return spot * normalCDF(d1) - strike * Math.exp(-interestRate * tenor) * normalCDF(d2);
}

// Function to calculate premium
function calculatePremium(price: number, strikeRatio: number, volatility: number): number {
    const strike = price * strikeRatio;
    const tenor = 30 / 365; // 30 days in years
    const interest = 0.04;

    return blackScholesCall(price, strike, tenor, volatility, interest);
}

const app = express();
const port = 5001;

// Enable CORS for all routes
app.use(cors({
    origin: 'http://localhost:3000', // Replace with your React app's URL
    methods: ['GET', 'POST', 'OPTIONS'],
    allowedHeaders: ['Content-Type', 'Authorization'],
    credentials: true,
    optionsSuccessStatus: 204
}));

// Parse JSON bodies
app.use(express.json());

interface CoinGeckoResponse {
    solana: {
        usd: number;
    };
}

app.get('/price/action', async (req, res) => {
    try {
        // const { action } = req.query;
        // if (!action || typeof action !== 'string') {
        //     return res.status(400).json({ error: 'Missing or invalid action parameter' });
        // }

        // const decodedAction = decodeURIComponent(action);

        // Fetch Solana price from CoinGecko
        const response = await fetch('https://api.coingecko.com/api/v3/simple/price?ids=solana&vs_currencies=usd');
        const data = await response.json() as CoinGeckoResponse;

        // Check if the response has the expected structure
        if (!data.solana || typeof data.solana.usd !== 'number') {
            throw new Error('Unexpected response format from CoinGecko API');
        }

        const solanaPrice = data.solana.usd;

        const volatility = 0.75; // 75% annualized volatility

        // Calculate premiums
        const premium120 = calculatePremium(solanaPrice, 1.2, volatility);
        const premium200 = calculatePremium(solanaPrice, 2.0, volatility);

        const finalPremium = premium120 - premium200;

        res.json({
            solanaPrice,
            premium120,
            premium200,
            finalPremium
        });
    } catch (error) {
        res.status(500).json({ error: 'An error occurred while processing the request' });
    }
});

app.post('/transaction', async (req, res) => {
    try {
        const { signedTransaction, programId } = req.body;

        if (!signedTransaction || !programId) {
            return res.status(400).json({ error: 'Missing signedTransaction or programId' });
        }


        // Initialize a connection to the Solana network
        const connection = new Connection('https://api.mainnet-beta.solana.com');
        const transaction = Transaction.from(Buffer.from(signedTransaction, 'base64'));

        // This is where you would sign the transaction with your private key
        // transaction.sign(privateKey);

        const signature = await connection.sendRawTransaction(transaction.serialize());

        res.json({ signature });
    } catch (error) {
        res.status(500).json({ error: 'An error occurred while processing the transaction' });
    }
});


app.listen(port, () => {
    console.log(`Server is running on http://localhost:${port}`);
});