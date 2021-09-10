use anchor_lang::prelude::*;

#[program]
pub mod rust_task {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {

}

pub struct Contract {
    pub users: Vec<User>,
    pub tokens: u32,
    pub size: u8,
    full: bool
}

impl Contract {
    pub fn update_shares(&mut self) {
        for mut user in &mut self.users {
            let all_tokens = self.tokens as f32;
            let user_tokens = user.tokens as f32;

            user.share = all_tokens / user_tokens;
        }
    }

    pub fn mint_tokens(&mut self) {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        let minted_tokens: u32 = rng.gen_range(self.tokens/4..self.tokens/4*5);

        self.tokens += minted_tokens;
    }

    pub fn release_tokens(&mut self) {
        for mut user in &mut self.users {
            let user_shares = user.share;
            let all_tokens = self.tokens as f32;

            user.tokens = (all_tokens * user_shares) as u32;
        }
    }

    pub fn is_full(&self) -> bool {
        self.full
    }
}
#[derive(Copy, Clone, PartialEq)]
pub struct User {
    pub tokens: u32,
    pub share: f32
}

impl User {
    pub fn deposit_tokens(&mut self, contract: &mut Contract, tokens_num: u32) {
        if !(contract.users.contains(self)) {
            self.join_contract(contract);
        }

        contract.tokens += tokens_num;
        self.tokens -= tokens_num;

        if contract.users.len() == contract.size as usize {
            contract.full = true;
            contract.mint_tokens();
        }

        contract.update_shares();

    }

    fn join_contract(&self, contract: &mut Contract) {
        contract.users.push(*self);
    }
}
