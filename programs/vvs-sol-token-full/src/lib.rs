use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, MintTo, SetAuthority, Transfer};

use instructions::*;

pub mod instructions;

declare_id!("9Wxd8XZAomK6FAURaNdK5UsUXQ4CnpLU9YrB4ibFY3Fr");

#[program]
pub mod vvs_sol_token_full {

    use super::*;

    pub fn transfer(ctx: Context<ProxyTransfer>, amount: u64) -> Result<()> {
        token::transfer(ctx.accounts.into(), amount)
    }

    pub fn fee_transfer(ctx: Context<ProxyFeeTransfer>, amount: u64) -> Result<()> {
        token::transfer(ctx.accounts.into(), amount)
    }

    pub fn mint_to(ctx: Context<ProxyMintTo>, amount: u64) -> Result<()> {
        token::mint_to(ctx.accounts.into(), amount)
    }

    pub fn burn(ctx: Context<ProxyBurn>, amount: u64) -> Result<()> {
        token::burn(ctx.accounts.into(), amount)
    }

    pub fn set_authority(
        ctx: Context<ProxySetAuthority>,
        authority_type: AuthorityType,
        new_authority: Option<Pubkey>,
    ) -> Result<()> {
        token::set_authority(ctx.accounts.into(), authority_type.into(), new_authority)
    }

    // Amm instructions

    /// Pre initiazlize a swap pool
    pub fn proxy_pre_initialize(ctx: Context<ProxyPreInitialize>, nonce: u8) -> Result<()> {
        instructions::pre_initialize(ctx, nonce)
    }

    /// Initiazlize a swap pool
    pub fn proxy_initialize(
        ctx: Context<ProxyInitialize>,
        nonce: u8,
        open_time: u64,
    ) -> Result<()> {
        instructions::initialize(ctx, nonce, open_time)
    }

    /// deposit instruction
    pub fn proxy_deposit(
        ctx: Context<ProxyDeposit>,
        max_coin_amount: u64,
        max_pc_amount: u64,
        base_side: u64,
    ) -> Result<()> {
        instructions::deposit(ctx, max_coin_amount, max_pc_amount, base_side)
    }

    /// withdraw instruction
    pub fn proxy_withdraw(ctx: Context<ProxyWithdraw>, amount: u64) -> Result<()> {
        instructions::withdraw(ctx, amount)
    }

    /// swap_base_in instruction
    pub fn proxy_swap_base_in(
        ctx: Context<ProxySwapBaseIn>,
        amount_in: u64,
        minimum_amount_out: u64,
    ) -> Result<()> {
        instructions::swap_base_in(ctx, amount_in, minimum_amount_out)
    }

    /// swap_base_out instruction
    pub fn proxy_swap_base_out(
        ctx: Context<ProxySwapBaseOut>,
        max_amount_in: u64,
        amount_out: u64,
    ) -> Result<()> {
        instructions::swap_base_out(ctx, max_amount_in, amount_out)
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum AuthorityType {
    /// Authority to mint new tokens
    MintTokens,
    /// Authority to freeze any account associated with the Mint
    FreezeAccount,
    /// Owner of a given token account
    AccountOwner,
    /// Authority to close a token account
    CloseAccount,
}

#[derive(Accounts)]
pub struct ProxyTransfer<'info> {
    #[account(signer)]
    /// CHECK: for safety
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: for sender
    pub from: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: for receiver
    pub to: AccountInfo<'info>,
    /// CHECK: for token program
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ProxyFeeTransfer<'info> {
    #[account(signer)]
    /// CHECK: for safety
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: for sender
    pub from: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: for receiver
    pub to: AccountInfo<'info>,
    /// CHECK: for token program
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ProxyMintTo<'info> {
    #[account(signer)]
    /// CHECK: for authority
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: for mint
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: for to
    pub to: AccountInfo<'info>,
    /// CHECK: for token_program
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ProxyBurn<'info> {
    #[account(signer)]
    /// CHECK: for authority
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: for mint
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: for from
    pub from: AccountInfo<'info>,
    /// CHECK: for token_program
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ProxySetAuthority<'info> {
    #[account(signer)]
    /// CHECK: for current_authority
    pub current_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: for account_or_mint
    pub account_or_mint: AccountInfo<'info>,
    /// CHECK: for token_program
    pub token_program: AccountInfo<'info>,
}

impl<'a, 'b, 'c, 'info> From<&mut ProxyTransfer<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Transfer<'info>>
{
    fn from(accounts: &mut ProxyTransfer<'info>) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: accounts.from.clone(),
            to: accounts.to.clone(),
            authority: accounts.authority.clone(),
        };
        let cpi_program = accounts.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'a, 'b, 'c, 'info> From<&mut ProxyFeeTransfer<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Transfer<'info>>
{
    fn from(
        accounts: &mut ProxyFeeTransfer<'info>,
    ) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: accounts.from.clone(),
            to: accounts.to.clone(),
            authority: accounts.authority.clone(),
        };
        let cpi_program = accounts.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'a, 'b, 'c, 'info> From<&mut ProxyMintTo<'info>>
    for CpiContext<'a, 'b, 'c, 'info, MintTo<'info>>
{
    fn from(accounts: &mut ProxyMintTo<'info>) -> CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: accounts.mint.clone(),
            to: accounts.to.clone(),
            authority: accounts.authority.clone(),
        };
        let cpi_program = accounts.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'a, 'b, 'c, 'info> From<&mut ProxyBurn<'info>> for CpiContext<'a, 'b, 'c, 'info, Burn<'info>> {
    fn from(accounts: &mut ProxyBurn<'info>) -> CpiContext<'a, 'b, 'c, 'info, Burn<'info>> {
        let cpi_accounts = Burn {
            mint: accounts.mint.clone(),
            from: accounts.from.clone(),
            authority: accounts.authority.clone(),
        };
        let cpi_program = accounts.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'a, 'b, 'c, 'info> From<&mut ProxySetAuthority<'info>>
    for CpiContext<'a, 'b, 'c, 'info, SetAuthority<'info>>
{
    fn from(
        accounts: &mut ProxySetAuthority<'info>,
    ) -> CpiContext<'a, 'b, 'c, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: accounts.account_or_mint.clone(),
            current_authority: accounts.current_authority.clone(),
        }; // TODO: Support multisig signers
        let cpi_program = accounts.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl From<AuthorityType> for spl_token::instruction::AuthorityType {
    fn from(authority_ty: AuthorityType) -> spl_token::instruction::AuthorityType {
        match authority_ty {
            AuthorityType::MintTokens => spl_token::instruction::AuthorityType::MintTokens,
            AuthorityType::FreezeAccount => spl_token::instruction::AuthorityType::FreezeAccount,
            AuthorityType::AccountOwner => spl_token::instruction::AuthorityType::AccountOwner,
            AuthorityType::CloseAccount => spl_token::instruction::AuthorityType::CloseAccount,
        }
    }
}
