use anchor_lang::prelude::*;

declare_id!("88hTydB9esnUMmX7gVUb5PrdLgnJ8YvEiqCr9vZ1cXrH");

#[program]
pub mod liftskit_membership_v3 {
    use super::*;
    
    pub fn create(ctx: Context<Create>, member_address: Pubkey) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let address = base_account.to_account_info().key;
        base_account.members.push(*address);
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>, member_address: Pubkey) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let address = base_account.to_account_info().key;
        base_account.members.push(*address);
        Ok(())
    }
}

// Transaction instructions
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 64 + 64)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Transaction instructions
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

// An account that goes inside a transaction instruction
#[account]
pub struct BaseAccount {
    pub members: Vec<Pubkey>,
}