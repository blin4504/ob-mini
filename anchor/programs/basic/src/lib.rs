#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, MintTo, Transfer};
mod contexts;
mod errors;
mod state;
use contexts::*;
use errors::*;

declare_id!("34MTvzxZGZNwW2vfcPaMYAt5GvdBbn4A8zssr6gRsAU6");

#[program]
pub mod basic {

    use anchor_lang::solana_program::{program::invoke, system_instruction};
    use anchor_spl::token::mint_to;

    use super::*;

    pub fn create_market(
        _ctx: Context<CreateMarket>,
        market_name: String,
        deadline: i64,
    ) -> Result<()> {
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
            amount,
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                _ctx.accounts.signer.to_account_info(),
                _ctx.accounts.vault.to_account_info(),
            ],
        )?;

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

    pub fn buy(
        _ctx: Context<Buy>,
        token_type: bool,
        token_amount: u64,
        max_price: u64,
    ) -> Result<()> {
        let order = &mut _ctx.accounts.order;
        order.market = _ctx.accounts.market.key();
        order.buyer = _ctx.accounts.signer.key();
        order.token = token_type;
        order.token_amount = token_amount;
        order.max_price = max_price;
        order.executed = false;

        let total_cost = token_amount * max_price;
        let ix = system_instruction::transfer(
            &_ctx.accounts.signer.key(),
            &_ctx.accounts.escrow.key(),
            total_cost,
        );
        invoke(
            &ix,
            &[
                _ctx.accounts.signer.to_account_info(),
                _ctx.accounts.escrow.to_account_info(),
            ],
        )?;
        msg!("BUY ORDER");
        Ok(())
    }

    pub fn sell(
        _ctx: Context<Sell>,
        token_type: bool,
        token_amount: u64,
        min_price: u64,
    ) -> Result<()> {
        let order = &mut _ctx.accounts.order;
        order.market = _ctx.accounts.market.key();
        order.seller = _ctx.accounts.signer.key();
        order.token = token_type;
        order.token_amount = token_amount;
        order.min_price = min_price;
        order.executed = false;

        if token_type == true {
            if _ctx.accounts.token_mint.key() != _ctx.accounts.market.yes_mint {
                return Err(MyError::WrongMint.into());
            }
        } else {
            if _ctx.accounts.token_mint.key() != _ctx.accounts.market.no_mint {
                return Err(MyError::WrongMint.into());
            }
        }

        let cpi = CpiContext::new(
            _ctx.accounts.user_token_account.to_account_info(),
            Transfer {
                from: _ctx.accounts.user_token_account.to_account_info(),
                to: _ctx.accounts.escrow_token_account.to_account_info(),
                authority: _ctx.accounts.signer.to_account_info(),
            },
        );
        transfer(cpi, token_amount)?;

        let token_escrow = &mut _ctx.accounts.token_escrow;
        token_escrow.order = _ctx.accounts.order.key();
        token_escrow.token_account = _ctx.accounts.escrow_token_account.key();

        msg!("SELL ORDER");
        Ok(())
    }
}
