use crate::states::ReferralConfig;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitReferralConfig<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        seeds = [b"config"],
        bump,
        space = 8 + 32 * 3 + 8 + 1, // 8+32+32+32+8+1
    )]
    pub config: Account<'info, ReferralConfig>,

    pub system_program: Program<'info, System>,
}

pub fn init_config(
    ctx: Context<InitReferralConfig>,
    admin: Pubkey,
    nft_mint: Pubkey,
    protocol_wallet: Pubkey,
    claim_fee: u64,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.admin = admin;
    config.nft_mint = nft_mint;
    config.protocol_wallet = protocol_wallet;
    config.claim_fee = claim_fee;
    config.bump = ctx.bumps.config;
    Ok(())
}
