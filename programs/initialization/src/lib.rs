use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_pack::Pack;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod initialization {
    use super::*;

    pub fn secure_initialization(ctx: Context<InitializeUser>) -> Result<()> {
        let user = &mut ctx.accounts.user;
        require!(
            user.authority == Pubkey::default(),
            CustomError::AlreadyInitialized
        );
        user.authority = *ctx.accounts.authority.key;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + 32,
        seeds = [b"user", authority.key().as_ref()],
        bump
    )]
    pub user: Account<'info, UserSecure>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserSecure {
    pub authority: Pubkey,
}

#[error_code]
pub enum CustomError {
    #[msg("Account already initialized.")]
    AlreadyInitialized,
}