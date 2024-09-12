//Solana Betting Program Structure

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("Your_Program_ID_Here");

#[program]
pub mod solana_betting {
    use super::*;

    pub fn place_bet(
        ctx: Context<PlaceBet>,
        notional_amount: u64,
        premium_percentage: u64,
        strike_price_percentage: u64,
        strike_cap_percentage: u64,
        token: Pubkey,
    ) -> Result<()> {
        let bet = &mut ctx.accounts.bet;
        let user = &ctx.accounts.user;
        let clock = Clock::get()?;

        // Validate input
        // locked in at 10% max premium
        require!(premium_percentage <= 1000, BettingError::PremiumTooHigh);
        // strike cap must be higher than strike price
        require!(strike_cap_percentage > strike_price, BettingError::InvalidStrikeCap);

        // Calculate premium amount
        // premium = notional_amount * premium_percentage / 10000
        let premium_amount = notional_amount
            .checked_mul(premium_percentage)
            .ok_or(BettingError::CalculationError)?
            .checked_div(10000)
            .ok_or(BettingError::CalculationError)?;

        // Calculate maximum payout
        // max_payout = notional_amount * (strike_cap_percentage - strike_percentage) / 10000
        // e.g. a value of 18000 strike_cap_percentage means 180% cap
        let max_payout = notional_amount
            .checked_mul(strike_cap_percentage - strike_price_percentage)
            .ok_or(BettingError::CalculationError)?
            .checked_div(10000)
            .ok_or(BettingError::CalculationError)?;

        // Check if the contract has enough USDC to back the bet
        let contract_usdc_balance = ctx.accounts.contract_usdc_account.amount;
        let total_locked_usdc = ctx.accounts.contract_state.total_locked_usdc;
        let available_usdc = contract_usdc_balance
            .checked_sub(total_locked_usdc)
            .ok_or(BettingError::InsufficientContractBalance)?;

        require!(available_usdc >= max_payout, BettingError::InsufficientContractBalance);

        // Transfer premium from user to contract
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_usdc_account.to_account_info(),
            to: ctx.accounts.contract_usdc_account.to_account_info(),
            authority: user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, premium_amount)?;

        // Update contract state
        ctx.accounts.contract_state.total_locked_usdc = ctx
            .accounts
            .contract_state
            .total_locked_usdc
            .checked_add(max_payout)
            .ok_or(BettingError::CalculationError)?;

        let start_price = get_current_price(&ctx.accounts.pyth_price_account)?;
        let strike_price = start_price
            .checked_mul(strike_price_percentage)
            .ok_or(BettingError::CalculationError)?
            .checked_div(10000)
            .ok_or(BettingError::CalculationError)?;

        // Initialize bet struct
        bet.user = *user.key;
        bet.has_claimed = false;
        bet.expiry = clock.unix_timestamp + 86400; // 1 day from now
        bet.notional_amount = notional_amount;
        bet.premium_amount = premium_amount;
        bet.start_price = start_price;
        bet.strike_price = strike_price;
        bet.strike_cap_percentage = strike_cap_percentage;
        bet.token = token;

        // Add bet ID to user's bet list
        let user_bets = &mut ctx.accounts.user_bets;
        user_bets.bet_ids.push(*ctx.accounts.bet.to_account_info().key);

        Ok(())
    }

    pub fn claim_bet(ctx: Context<ClaimBet>) -> Result<()> {
        let bet = &mut ctx.accounts.bet;
        let clock = Clock::get()?;

        require!(!bet.has_claimed, BettingError::AlreadyClaimed);
        require!(clock.unix_timestamp > bet.expiry, BettingError::BetNotExpired);
        require!(clock.unix_timestamp <= bet.expiry + 86400, BettingError::ClaimPeriodExpired);

        let current_price = get_current_price(&ctx.accounts.pyth_price_account)?;

        // did the option expire in the money
        let is_winning_bet = current_price >= bet.strike_price;
        let mut excess_to_unlock = if !is_winning_bet {
            // if losing bet you can unlock everything
            notional_amount
            .checked_mul(strike_cap_percentage - strike_price_percentage)
            .ok_or(BettingError::CalculationError)?
            .checked_div(10000)
            .ok_or(BettingError::CalculationError)?;
        };

        if is_winning_bet {
            let cap_price = bet
                            .start_price
                            .checked_mul(bet.strike_cap_percentage)
                            .ok_or(BettingError::CalculationError)?
                            .checked_div(10000)
                            .ok_or(BettingError::CalculationError)?;
            // payout capped at cap price
            let payout_price = current_price > cap_price ? cap_price : current_price;
            // get max payout, will later be used for excess unlocking
            let max_payout = bet
                            .notional_amount
                            .checked_mul(cap_price - bet.strike_price)
                            .ok_or(BettingError::CalculationError)?
                            .checked_div(bet.start_price)
                            .ok_or(BettingError::CalculationError)?;
            let payout = (payout_price - bet.strike_price)
                        .checked_div(bet.start_price)
                        .ok_or(BettingError::CalculationError)?
                        .checked_mul(bet.notional_amount)
                        .ok_or(BettingError::CalculationError)?; 

            // Transfer payout to user
            let cpi_accounts = Transfer {
                from: ctx.accounts.contract_usdc_account.to_account_info(),
                to: ctx.accounts.user_usdc_account.to_account_info(),
                authority: ctx.accounts.contract_state.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::transfer(cpi_ctx, payout)?;

            // unlock the excess if any from locked max payout and actual payout
            excess_to_unlock = max_payout - payout;
        }

        // Update contract state
        ctx.accounts.contract_state.total_locked_usdc = ctx
        .accounts
        .contract_state
        .total_locked_usdc
        .checked_sub(excess_to_unlock)
        .ok_or(BettingError::CalculationError)?;

        bet.has_claimed = true;

        Ok(())
    }

    pub fn unlock_expired_bet(ctx: Context<ClaimBet>, bet_id: Pubkey) -> Result<()> {
        let bet = &mut ctx.accounts.bet;
        let clock = Clock::get()?;

        // Check that the bet hasn't been claimed
        require!(!bet.has_claimed, BettingError::AlreadyClaimed);

        // Check that expiry + 86400 has passed
        require!(
            clock.unix_timestamp > bet.expiry + 86400,
            BettingError::ClaimPeriodNotExpired
        );

        // Calculate max payout
        let cap_price = bet
            .start_price
            .checked_mul(bet.strike_cap_percentage)
            .ok_or(BettingError::CalculationError)?
            .checked_div(10000)
            .ok_or(BettingError::CalculationError)?;

        let max_payout = bet
            .notional_amount
            .checked_mul(cap_price.checked_sub(bet.strike_price).ok_or(BettingError::CalculationError)?)
            .ok_or(BettingError::CalculationError)?
            .checked_div(bet.start_price)
            .ok_or(BettingError::CalculationError)?;

        // Update contract state
        ctx.accounts.contract_state.total_locked_usdc = ctx
            .accounts
            .contract_state
            .total_locked_usdc
            .checked_sub(max_payout)
            .ok_or(BettingError::CalculationError)?;

        // Mark bet as claimed
        // this prevents the owner unlocking the same bet multiple times
        bet.has_claimed = true;

        msg!("Unlocked expired bet with ID: {:?}", bet_id);
        msg!("Unlocked amount: {}", max_payout);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(init, payer = payer, space = 8 + Bet::LEN)]
    pub bet: Account<'info, Bet>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub user_bets: Account<'info, UserBets>,
    #[account(mut)]
    pub user_usdc_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub contract_usdc_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub contract_state: Account<'info, ContractState>,
    pub pyth_price_account: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Bet {
    pub user: Pubkey,
    pub has_claimed: bool,
    pub expiry: i64,
    pub notional_amount: u64,
    pub premium_amount: u64,
    pub start_price: u64,
    pub strike_price: u64,
    pub strike_cap_percentage: u64,
    pub token: Pubkey,
}

#[account]
pub struct ContractState {
    pub total_locked_usdc: u64,
}

#[account]
pub struct UserBets {
    pub user: Pubkey,
    pub bet_ids: Vec<Pubkey>,
}

#[error_code]
pub enum BettingError {
    #[msg("Bet has already been claimed")]
    AlreadyClaimed,
    #[msg("Bet has not expired yet")]
    BetNotExpired,
    #[msg("Claim period has expired")]
    ClaimPeriodExpired,
    #[msg("Premium percentage is too high")]
    PremiumTooHigh,
    #[msg("Invalid strike cap")]
    InvalidStrikeCap,
    #[msg("Calculation error")]
    CalculationError,
    #[msg("Insufficient contract balance")]
    InsufficientContractBalance,
    #[msg("Pyth error")]
    PythError,
    #[msg("Claim period has not expired yet")]
    ClaimPeriodNotExpired
}

impl Bet {
    pub const LEN: usize = 32 + 1 + 8 + 8 + 8 + 8 + 8 + 8 + 32;
}

fn get_current_price(pyth_price_account: &AccountInfo) -> Result<u64> {
    let price_feed = load_price_feed_from_account_info(pyth_price_account)
        .map_err(|_| error!(BettingError::PythError))?;
    let current_price = price_feed
        .get_price_unchecked()
        .price;
    Ok(current_price as u64)
}