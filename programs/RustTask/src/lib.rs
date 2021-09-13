use anchor_lang::prelude::*;
use anchor_lang::solana_program::{declare_id, system_program};
use rand::Rng;

declare_id!("5wjcBrJYAE1wRcD5kFKjXQP3jZNEEFXWUB2hf7GxH5kC");

#[program]
pub mod rust_task {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>, authority: Pubkey) -> ProgramResult {
        let mut rng = rand::thread_rng();

        let user_account = &mut ctx.accounts.user_account;
        user_account.authority = authority;
        user_account.tokens = rng.gen_range(10..100);
        user_account.share = 0.0;

        Ok(())
    }

    pub fn initialize_contract(ctx: Context<InitializeContract>) -> ProgramResult {
        let contract = &mut ctx.accounts.contract;
        contract.users_accounts = Vec::new();
        contract.tokens = 0;
        contract.size = 5;
        contract.full = false;

        Ok(())
    }

    pub fn deposit_tokens(ctx: Context<DepositTokens>) -> Result<()> {
        let mut rng = rand::thread_rng();

        let contract = &mut ctx.accounts.contract;
        let user_account = &mut ctx.accounts.user_account;
        let tokens_num = rng.gen_range(5..user_account.tokens);

        if !contract.full {
            user_account.deposit_tokens(contract, tokens_num);
        } else {
            return Err(ErrorCode::UserNumberReached.into());
        }

        Ok(())
    }

    pub fn release_tokens(ctx: Context<ReleaseTokens>) -> ProgramResult {
        let contract = &mut ctx.accounts.contract;
        contract.release_tokens();

        Ok(())
    }


}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub user_account: ProgramAccount<'info, UserAccount>,
    #[account(signer)]
    pub user: AccountInfo<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>
}

#[derive(Accounts)]
pub struct InitializeContract<'info> {
    #[account(init, payer = system, space = 8 + 40)]
    pub contract: ProgramAccount<'info, Contract>,
    pub system: AccountInfo<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>
}

#[derive(Accounts)]
pub struct DepositTokens<'info> {
    #[account(mut)]
    pub contract: ProgramAccount<'info, Contract>,
    #[account(mut, has_one = authority)]
    pub user_account: ProgramAccount<'info, UserAccount>,
    #[account(signer)]
    pub authority: AccountInfo<'info>
}

#[derive(Accounts)]
pub struct ReleaseTokens<'info> {
    #[account(mut)]
    pub contract: ProgramAccount<'info, Contract>
}

#[account]
pub struct Contract {
    pub users_accounts: Vec<UserAccount>,
    pub tokens: u32,
    pub size: u32,
    full: bool
}

impl Contract {
    pub fn update_shares(&mut self) {
        for mut user_account in &mut self.users_accounts {
            let all_tokens = self.tokens as f32;
            let user_tokens = user_account.tokens as f32;

            user_account.share = all_tokens / user_tokens;
        }
    }

    pub fn mint_tokens(&mut self) {
        let mut rng = rand::thread_rng();
        let minted_tokens: u32 = rng.gen_range(self.tokens/4..self.tokens/4*5);

        self.tokens += minted_tokens;
    }

    pub fn release_tokens(&mut self) {
        for mut user_account in &mut self.users_accounts {
            let user_shares = user_account.share;
            let all_tokens = self.tokens as f32;

            user_account.tokens = (all_tokens * user_shares) as u32;
        }
    }

    pub fn is_full(&self) -> bool {
        self.full
    }
}

#[account]
#[derive(Copy, PartialEq)]
pub struct UserAccount {
    pub authority: Pubkey,
    pub tokens: u32,
    pub share: f32
}

impl UserAccount {
    pub fn deposit_tokens(&mut self, contract: &mut Contract, tokens_num: u32) {
        if !(contract.users_accounts.contains(self)) {
            self.join_contract(contract);
        }

        contract.tokens += tokens_num;
        self.tokens -= tokens_num;

        contract.update_shares();

        if contract.users_accounts.len() == contract.size as usize {
            contract.full = true;
            contract.mint_tokens();
        }

    }

    fn join_contract(&self, contract: &mut Contract) {
        contract.users_accounts.push(*self);
    }
}

#[error]
pub enum ErrorCode {
    #[msg("Contract reached maximum number of users")] UserNumberReached
}
