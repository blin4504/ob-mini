#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount, MintTo}};

declare_id!("34MTvzxZGZNwW2vfcPaMYAt5GvdBbn4A8zssr6gRsAU6");

#[program]
pub mod basic {

    use anchor_spl::token::mint_to;

    use super::*;

    pub fn create_market(_ctx: Context<CreateMarket>, market_name: String, deadline: i64) -> Result<()> {
        let market = &mut _ctx.accounts.market;
        market.market_name = market_name;
        market.deadline = deadline;
        market.yes_mint = _ctx.accounts.yes_mint.key();
        market.no_mint = _ctx.accounts.no_mint.key();
        market.authority = _ctx.accounts.signer.key();
        market.resolved = false;
        msg!("CREATED MARKET");
        Ok(())
    }

    pub fn mint(_ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        let token_amount: u64 = amount / 10000000;
        let seeds = &["market".as_bytes(), &[_ctx.bumps.market]];
        let signer = [&seeds[..]];

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &_ctx.accounts.signer.key(), 
            &_ctx.accounts.vault.key(),
            amount);
        
        anchor_lang::solana_program::program::invoke(&ix, &[
            _ctx.accounts.signer.to_account_info(),
            _ctx.accounts.vault.to_account_info(),
        ],)?;
        
        mint_to(
            CpiContext::new_with_signer(
                _ctx.accounts.token_program.to_account_info(),
                MintTo {
                    authority: _ctx.accounts.market.to_account_info(),
                    to: _ctx.accounts.user_yes_token.to_account_info(),
                    mint: _ctx.accounts.yes_mint.to_account_info(),
                },
                &signer,
            ),
            token_amount,
        )?;

        mint_to(
            CpiContext::new_with_signer(
                _ctx.accounts.token_program.to_account_info(),
                MintTo {
                    authority: _ctx.accounts.market.to_account_info(),
                    to: _ctx.accounts.user_no_token.to_account_info(),
                    mint: _ctx.accounts.no_mint.to_account_info(),
                },
                &signer,
            ),
            token_amount,
        )?;
        msg!("MINTED TOKENS");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateMarket<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + Market::INIT_SPACE,
        seeds = [b"market", signer.key().as_ref()],
        bump
    )]
    pub market: Account<'info, Market>,

    #[account(
        init,
        payer = signer,
        mint::decimals = 6,
        mint::authority = market,
        seeds = [b"yes_mint"],
        bump,
    )]
    pub yes_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = signer,
        mint::decimals = 6,
        mint::authority = market,
        seeds = [b"no_mint"],
        bump,
    )]
    pub no_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    // need a token address

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"market", signer.key().as_ref()],
        bump
    )]
    pub market: Account<'info, Market>,

    #[account(mut)]
    pub yes_mint: Account<'info, Mint>,

    #[account(mut)]
    pub no_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = yes_mint,
        associated_token::authority = signer,
    )]
    pub user_yes_token: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = no_mint,
        associated_token::authority = signer,
    )]
    pub user_no_token: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + Vault::INIT_SPACE,
        seeds = [b"vault", signer.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Market {
    #[max_len(100)]
    market_name: String,
    deadline: i64,
    yes_mint: Pubkey,
    no_mint: Pubkey,
    authority: Pubkey,
    resolved: bool
}

#[account]
#[derive(InitSpace)]
pub struct Vault {}
