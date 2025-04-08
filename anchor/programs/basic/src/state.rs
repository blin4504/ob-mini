use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Market {
    #[max_len(100)]
    pub market_name: String,
    pub deadline: i64,
    pub yes_mint: Pubkey,
    pub no_mint: Pubkey,
    pub authority: Pubkey,
    pub resolved: bool,
}

#[account]
#[derive(InitSpace)]
pub struct BuyOrder {
    pub market: Pubkey,
    pub buyer: Pubkey,
    pub token: bool,
    pub token_amount: u64,
    pub max_price: u64,
    pub executed: bool,
}

#[account]
#[derive(InitSpace)]
pub struct SellOrder {
    pub market: Pubkey,
    pub seller: Pubkey,
    pub token: bool,
    pub token_amount: u64,
    pub min_price: u64,
    pub executed: bool,
}

#[account]
#[derive(InitSpace)]
pub struct Vault {}

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub order: Pubkey,
    pub amount: u64,
}

#[account]
#[derive(InitSpace)]
pub struct TokenEscrow {
    pub order: Pubkey,
    pub token_account: Pubkey,
}