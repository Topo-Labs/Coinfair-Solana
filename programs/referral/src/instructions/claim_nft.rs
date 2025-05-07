use crate::error::ReferralError;
use crate::states::ReferralConfig;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct ClaimReferralNFT<'info> {
    /// 下级用户，执行领取操作
    #[account(mut)]
    pub user: Signer<'info>,

    /// 全局配置，包含官方NFT mint地址、手续费等信息
    #[account(
        seeds = [b"config"],
        bump = config.bump,
    )]
    pub config: Account<'info, ReferralConfig>,

    /// 官方NFT Mint账户
    #[account(
        mut,
        address = config.nft_mint, // 确保官方NFT的mint地址
    )]
    pub official_mint: Account<'info, Mint>,

    /// 用户的ATA账户，接收NFT
    #[account(
        mut,
        associated_token::mint = official_mint,
        associated_token::authority = user,
    )]
    pub user_ata: Account<'info, TokenAccount>,

    /// 用户支付手续费的账户
    #[account(
        mut,
        constraint = user_token_account.amount >= config.claim_fee, // 用户账户余额必须足够支付手续费
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    /// 支付手续费的目标账户，协议方钱包
    #[account(
        mut,
        address = config.protocol_wallet, // 协议接收钱包
    )]
    pub protocol_receive_wallet: Account<'info, TokenAccount>,

    /// CPI相关的Token Program
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn claim_nft(ctx: Context<ClaimReferralNFT>) -> Result<()> {
    let user = &ctx.accounts.user;
    let config = &ctx.accounts.config;
    let protocol_receive_wallet = &ctx.accounts.protocol_receive_wallet;

    // 1. 确保用户未领取过
    let user_ata = &ctx.accounts.user_ata;
    if user_ata.amount > 0 {
        return Err(ReferralError::AlreadyClaimed.into());
    }

    // 2. 扣除手续费
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.user_token_account.to_account_info(),
                to: protocol_receive_wallet.to_account_info(),
                authority: user.to_account_info(),
            },
        ),
        config.claim_fee, // 扣除配置数量的 SOL（lamports）
    )?;

    // 3. 将NFT mint给用户
    token::mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.official_mint.to_account_info(),
                to: ctx.accounts.user_ata.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        1, // 每次领取1个NFT
    )?;

    Ok(())
}
