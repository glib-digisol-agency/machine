use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::AccountInfo,
        entrypoint::ProgramResult,
        program::{invoke, invoke_signed},
        program_error::ProgramError,
        pubkey::Pubkey,
        rent::Rent,
        sysvar::Sysvar,
        {msg, system_instruction},
    },
};

use crate::processor::consts::ADMIN;

pub trait Data {
    fn data_size(&self) -> u64 {
        std::mem::size_of_val(self) as u64
    }
}

pub trait Initialize: Data + BorshSerialize {
    /// Execute create account instruction with data len of `Self`.
    fn create_account<'a, 'b>(
        &self,
        from: &'a AccountInfo<'b>,
        to: &'a AccountInfo<'b>,
        sys: &'a AccountInfo<'b>,
        owner: &Pubkey,
        sign: &[&[&[u8]]],
    ) -> ProgramResult {
        let data_len = (self.try_to_vec()?).len();

        let lamports_required = Rent::get()?.minimum_balance(data_len);

        invoke_signed(
            &system_instruction::create_account(
                from.key,
                to.key,
                lamports_required,
                data_len as u64,
                owner,
            ),
            &[from.clone(), to.clone(), sys.clone()],
            sign,
        )?;
        Ok(())
    }
    /// Send corresponding to data size of `Self`, lamports to account.
    fn pay_rent<'a, 'b>(
        &self,
        from: &'a AccountInfo<'b>,
        to: &'a AccountInfo<'b>,
        rent: &'a AccountInfo<'b>,
    ) -> ProgramResult {
        let size = self.data_size();
        let required_lamports = Rent::from_account_info(rent)?
            .minimum_balance(size as usize)
            .max(1)
            .saturating_sub(to.lamports());

        invoke(
            &system_instruction::transfer(from.key, to.key, required_lamports),
            &[from.clone(), to.clone()],
        )?;

        Ok(())
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct LotteryMachine {
    pub campaign_index: u16,
    pub admin: Pubkey,
}

impl Default for LotteryMachine {
    fn default() -> Self {
        Self {
            campaign_index: 0,
            admin: ADMIN,
        }
    }
}

impl Data for LotteryMachine {}

impl Initialize for LotteryMachine {}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct LotteryCampaign {
    pub campaign_index: u16,
    pub tickets: u16,
    pub bought_tickets: u16,
    pub user_index: u16,
    pub price: u32,
    pub wallet: Pubkey,
    pub nft: Pubkey,
    pub is_finished: bool,
    pub winner: u16,
}

impl LotteryCampaign {
    pub fn new(price: u32, nft: Pubkey, wallet: Pubkey, campaign_index: u16) -> Self {
        Self {
            tickets: 1000,
            bought_tickets: 0,
            user_index: 0,
            price,
            nft,
            wallet,
            campaign_index,
            is_finished: false,
            winner: Default::default(),
        }
    }
}

impl LotteryCampaign {
    pub fn is_enough_tickets(&self, amount: u16) -> ProgramResult {
        if self.tickets == self.bought_tickets + amount {
            msg!("Not enough tickets.");
            return Err(ProgramError::Custom(0));
        }
        Ok(())
    }
}

impl Data for LotteryCampaign {}

impl Initialize for LotteryCampaign {}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct TicketData {
    pub owner: Pubkey,
    pub tickets: u16,
    pub campaign_index: u16,
}

impl TicketData {
    pub fn new(owner: Pubkey, tickets: u16, campaign_index: u16) -> Self {
        Self {
            owner,
            tickets,
            campaign_index,
        }
    }
}

impl Data for TicketData {}

impl Initialize for TicketData {}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct UserData {
    pub user_index: u16,
}

impl UserData {
    pub fn new(user_index: u16) -> Self {
        Self { user_index }
    }
}

impl Data for UserData {}

impl Initialize for UserData {}
