use crate::states::ReferralAccount;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct GetReferrer<'info> {
    /// CHECK: 用户地址，客户端传过来的
    pub user: UncheckedAccount<'info>,

    // 读取用户对应的 ReferralAccount
    #[account(
        seeds = [b"referral", user.key().as_ref()],
        bump,
    )]
    pub referral_account: Account<'info, ReferralAccount>,
}

pub fn get_upper(ctx: Context<GetReferrer>) -> Result<(Option<Pubkey>, Option<Pubkey>)> {
    let referral = &ctx.accounts.referral_account;
    Ok((referral.upper, referral.upper_upper))
}
