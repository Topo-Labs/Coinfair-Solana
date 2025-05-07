pub mod constants;
pub mod error;
pub mod instructions;
pub mod states;
pub mod utils;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("NFTqT8aVmB4WjzFkyKX9h5sEhhbTowNpuwbKNi2CZXs");

#[program]
pub mod referral {
    use super::*;

    pub fn mint_referral_nft(ctx: Context<MintReferralNFT>, amount: u64) -> Result<()> {
        mint_nft::mint_nft(ctx, amount)
    }

    pub fn claim_referral_nft(ctx: Context<ClaimReferralNFT>) -> Result<()> {
        claim_nft::claim_nft(ctx)
    }

    pub fn set_upper(ctx: Context<SetUpper>) -> Result<()> {
        set_upper::set_upper(ctx)
    }

    pub fn init_config(
        ctx: Context<InitReferralConfig>,
        admin: Pubkey,
        official_nft_mint: Pubkey,
        protocol_receive_wallet: Pubkey,
        claim_fee_lamports: u64,
    ) -> Result<()> {
        init_config::init_config(
            ctx,
            admin,
            official_nft_mint,
            protocol_receive_wallet,
            claim_fee_lamports,
        )
    }
}
