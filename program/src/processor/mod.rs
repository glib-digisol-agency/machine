use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::pubkey::Pubkey;

use crate::error::ContractError;
use crate::instruction::ExampleInstruction::{
    self, BuyTicket, ClaimReward, Draw, InitializeAccount, InitializeCampaign, InitializeMachine,
    SetCampaignManager,
};
use crate::processor::buy_ticket::buy_ticket;
use crate::processor::claim_reward::claim;
use crate::processor::draw::draw;
use crate::processor::initialize_account::initialize_account;
use crate::processor::initialize_campaign::initialize_campaign;
use crate::processor::initialize_machine::initialize_machine;
use crate::processor::set_manager::set_manager;

pub mod buy_ticket;
pub mod claim_reward;
pub mod consts;
pub mod data_handlers;
pub mod draw;
mod initialize_account;
pub mod initialize_campaign;
mod initialize_machine;
mod set_manager;
pub mod utils;

/// Program state handler
pub struct Processor {}

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction: ExampleInstruction =
            match ExampleInstruction::try_from_slice(instruction_data) {
                Ok(insn) => insn,
                Err(err) => {
                    msg!("Failed to deserialize instruction: {}", err);
                    return Err(ContractError::InvalidInstructionData.into());
                }
            };

        match instruction {
            BuyTicket {
                amount,
                campaign_index,
            } => buy_ticket(program_id, accounts, amount, campaign_index)?,
            InitializeCampaign { price } => initialize_campaign(program_id, accounts, price)?,
            InitializeAccount => initialize_account(program_id, accounts)?,
            InitializeMachine => initialize_machine(program_id, accounts)?,
            Draw {
                campaign_index,
                user,
            } => draw(program_id, accounts, user.to_vec(), campaign_index)?,
            ClaimReward { campaign_index } => claim(program_id, accounts, campaign_index)?,
            SetCampaignManager => set_manager(program_id, accounts)?,
        };

        Ok(())
    }
}
