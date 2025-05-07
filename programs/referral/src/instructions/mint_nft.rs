use crate::error::ReferralError;
use crate::states::ReferralConfig;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};

#[derive(Accounts)]
pub struct MintReferralNFT<'info> {
    /// 铸造人
    #[account(mut)]
    pub authority: Signer<'info>,

    /// 读取全局配置
    #[account(
        seeds = [b"config"],
        bump = config.bump,
    )]
    pub config: Account<'info, ReferralConfig>,

    /// 官方NFT Mint
    #[account(
        mut,
        address = config.nft_mint, // 确保用的是正确的mint
    )]
    pub official_mint: Account<'info, Mint>,

    /// 用户ATA账户（如果不存在就自动创建）
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = official_mint,
        associated_token::authority = authority,
    )]
    pub user_ata: Account<'info, TokenAccount>,

    /// 基础依赖
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn mint_nft(ctx: Context<MintReferralNFT>, amount: u64) -> Result<()> {
    require!(amount > 0, ReferralError::InvalidMintAmount); // 防止乱mint 0个

    token::mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.official_mint.to_account_info(),
                to: ctx.accounts.user_ata.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        amount,
    )?;

    Ok(())
}
