use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Token, Transfer, SetAuthority};
use anchor_spl::token::spl_token::instruction::AuthorityType;

declare_id!("7FBiDyFR88R4xKFn3sXexJx9ebqBXH9jJYDrawdWiFUA");

#[program]
pub mod anchor_escrow {
    use super::*;

    /// Initialize escrow: create EscrowAccount and set vault authority to PDA
    pub fn initialize(
        ctx: Context<Initialize>,
        expected_amount: u64, // amount of payment token expected from taker
    ) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow_account;
        escrow.initializer_key = *ctx.accounts.initializer.key;
        escrow.initializer_token_account = ctx.accounts.initializer_token_account.key();
        escrow.vault_token_account = ctx.accounts.vault_token_account.key();
        escrow.payment_mint = ctx.accounts.payment_mint.key();
        escrow.expected_amount = expected_amount;
        escrow.bump = *ctx.bumps.get("escrow_account").unwrap();

        // change owner of vault_token_account to the vault_authority (PDA)
        let cpi_accounts = SetAuthority {
            account_or_mint: ctx.accounts.vault_token_account.to_account_info(),
            current_authority: ctx.accounts.initializer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        // Set authority to Program PDA (vault_authority)
        let vault_authority = ctx.accounts.vault_authority.key();
        let signer_seeds: &[&[u8]] = &[];
        token::set_authority(
            CpiContext::new(cpi_program, cpi_accounts),
            AuthorityType::AccountOwner,
            Some(vault_authority),
        )?;

        Ok(())
    }

    /// Exchange: taker sends expected payment tokens to initializer and receives vault tokens
    pub fn exchange(ctx: Context<Exchange>, _bump: u8) -> Result<()> {
        let escrow = &ctx.accounts.escrow_account;

        // Transfer payment from taker_payment_ata -> initializer_payment_ata
        let cpi_accounts_payment = Transfer {
            from: ctx.accounts.taker_payment_ata.to_account_info(),
            to: ctx.accounts.initializer_payment_ata.to_account_info(),
            authority: ctx.accounts.taker.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let amount = escrow.expected_amount;
        token::transfer(
            CpiContext::new(cpi_program.clone(), cpi_accounts_payment),
            amount,
        )?;

        // Transfer vault tokens from vault_token_account (owned by PDA) to taker_receive_ata
        // PDA must sign for it
        let seeds = &[
            b"escrow",
            escrow.initializer_key.as_ref(),
            &[escrow.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts_vault_to_taker = Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.taker_receive_ata.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        };

        token::transfer(
            CpiContext::new_with_signer(cpi_program.clone(), cpi_accounts_vault_to_taker, signer),
            ctx.accounts.vault_token_amount,
        )?;

        // Close the vault token account to initializer (optional)
        // We skip closing to keep balances visible; you can add a close instruction if needed.

        Ok(())
    }

    /// Cancel escrow and refund initializer: transfer vault tokens back to initializer
    pub fn cancel(ctx: Context<Cancel>, _bump: u8) -> Result<()> {
        let escrow = &ctx.accounts.escrow_account;
        let seeds = &[
            b"escrow",
            escrow.initializer_key.as_ref(),
            &[escrow.bump],
        ];
        let signer = &[&seeds[..]];

        // Transfer vault tokens back to initializer_token_account
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.initializer_token_account.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        };
        token::transfer(
            CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts, signer),
            ctx.accounts.vault_token_amount,
        )?;

        Ok(())
    }
}

#[account]
pub struct EscrowAccount {
    pub initializer_key: Pubkey,
    pub initializer_token_account: Pubkey, // token account that provided the tokens
    pub vault_token_account: Pubkey,       // PDA owned token account that holds tokens
    pub payment_mint: Pubkey,              // mint of token expected from taker
    pub expected_amount: u64,
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(expected_amount: u64)]
pub struct Initialize<'info> {
    #[account(init, payer = initializer, space = 8 + 32*4 + 8 + 1, seeds = [b"escrow", initializer.key().as_ref()], bump)]
    pub escrow_account: Account<'info, EscrowAccount>,

    /// CHECK: PDA that will be set as the authority of vault_token_account
    #[account(seeds = [b"escrow", initializer.key().as_ref()], bump)]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub initializer: Signer<'info>,

    /// The token account the initializer currently owns and will deposit tokens from
    #[account(mut)]
    pub initializer_token_account: Account<'info, TokenAccount>,

    /// The vault token account which will be set to PDA authority (must be a token account for the mint)
    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    /// The mint that is the payment token you expect from taker
    pub payment_mint: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Exchange<'info> {
    #[account(mut, has_one = initializer_key)]
    pub escrow_account: Account<'info, EscrowAccount>,

    /// CHECK: PDA authority for vault token account
    #[account(seeds = [b"escrow", escrow_account.initializer_key.as_ref()], bump = escrow_account.bump)]
    pub vault_authority: UncheckedAccount<'info>,

    // Taker who pays expected_amount
    #[account(mut)]
    pub taker: Signer<'info>,

    // Taker payment token account (must have at least expected_amount)
    #[account(mut)]
    pub taker_payment_ata: Account<'info, TokenAccount>,

    // Where the payment goes (initializer payment ATA)
    #[account(mut)]
    pub initializer_payment_ata: Account<'info, TokenAccount>,

    // Vault token account (owned by PDA)
    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    // Where taker receives the vault tokens
    #[account(mut)]
    pub taker_receive_ata: Account<'info, TokenAccount>,

    // The amount of tokens in the vault to transfer to taker
    // We pass this as an account to easily supply amount (u64)
    // Instead we'll read amount passed by client via ctx.accounts.vault_token_amount
    #[account(mut)]
    pub token_program: Program<'info, Token>,

    // helper to pass amount (not an account in Anchor). Instead pass as instruction data in client.
}

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(mut, has_one = initializer_key)]
    pub escrow_account: Account<'info, EscrowAccount>,

    /// CHECK: PDA authority
    #[account(seeds = [b"escrow", escrow_account.initializer_key.as_ref()], bump = escrow_account.bump)]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub initializer_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

