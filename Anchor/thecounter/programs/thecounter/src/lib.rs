use anchor_lang::prelude::*;

declare_id!("8vkLjf7savP6rzB9WpkvGKAAqkPXWbjezvNUEtwVHeLG");

#[program]
pub mod thecounter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data :u64) -> Result<()> {
        ctx.accounts.my_account.mybalance = data;
        msg!("Initialized from: {:?}", ctx.accounts.my_account.mybalance);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MyAccount{
    mybalance:u64
}
