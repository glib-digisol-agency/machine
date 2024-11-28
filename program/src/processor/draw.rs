use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        clock::Clock,
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        pubkey::Pubkey,
        sysvar::Sysvar,
    },
};

use crate::processor::{
    consts::ADMIN, data_handlers::LotteryCampaign, data_handlers::LotteryMachine,
    utils::assert_with_message,
};

pub fn draw(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    index_tickets: Vec<(u16, u8)>,
    campaign_index: u16,
) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    assert_with_message(
        !index_tickets.is_empty(),
        ProgramError::InvalidAccountData,
        "Empty data.",
    )?;

    let (lottery_machine_pda, _) = Pubkey::find_program_address(&[ADMIN.as_ref()], program_id);

    let (campaign_pda, _) = Pubkey::find_program_address(
        &[lottery_machine_pda.as_ref(), &campaign_index.to_le_bytes()],
        program_id,
    );

    assert_with_message(
        accounts.lottery_machine_pda.key == &lottery_machine_pda,
        ProgramError::InvalidAccountData,
        "Wrong lottery machine PDA.",
    )?;

    assert_with_message(
        accounts.campaign_pda.key == &campaign_pda,
        ProgramError::InvalidAccountData,
        "Wrong campaign PDA.",
    )?;

    let lottery_machine: LotteryMachine =
        LotteryMachine::try_from_slice(*(*accounts.lottery_machine_pda.data).borrow())?;

    assert_with_message(
        accounts.payer.key == &ADMIN || accounts.payer.key == &lottery_machine.admin,
        ProgramError::InvalidAccountData,
        "Only admin can call this instruction.",
    )?;

    assert_with_message(
        accounts.payer.is_signer,
        ProgramError::InvalidAccountData,
        "Payer must be a signer.",
    )?;

    let mut campaign: LotteryCampaign =
        LotteryCampaign::try_from_slice(*(*accounts.campaign_pda.data).borrow())?;

    assert_with_message(
        !campaign.is_finished,
        ProgramError::InvalidAccountData,
        "Campaign finished!",
    )?;

    let mut users_indexes = all_ticket_owners(&index_tickets,campaign.user_index)?;
    let tickets_sold = users_indexes.len();

    assert_with_message(
        tickets_sold == campaign.bought_tickets as usize
            && index_tickets.len() == campaign.user_index as usize,
        ProgramError::InvalidAccountData,
        "Inappropriate number of members.",
    )?;

    let random_number = get_random(&accounts, tickets_sold as u16)?;

    let winner: u16 = users_indexes.remove(random_number as usize);

    campaign.is_finished = true;
    campaign.winner = winner;

    campaign.serialize(&mut *(*accounts.campaign_pda).try_borrow_mut_data()?)?;

    msg!("Campaign winner index is {}!", winner);

    Ok(())
}

struct Accounts<'a, 'b> {
    pub payer: &'a AccountInfo<'b>,
    pub lottery_machine_pda: &'a AccountInfo<'b>,
    pub campaign_pda: &'a AccountInfo<'b>,
    pub chain_link: &'a AccountInfo<'b>,
    pub feed1: &'a AccountInfo<'b>,
    pub feed2: &'a AccountInfo<'b>,
    pub feed3: &'a AccountInfo<'b>,
}

impl<'a, 'b> Accounts<'a, 'b> {
    fn new(accounts: &'a [AccountInfo<'b>]) -> Result<Self, ProgramError> {
        let account_iter = &mut accounts.iter();

        Ok(Self {
            payer: next_account_info(account_iter)?,
            lottery_machine_pda: next_account_info(account_iter)?,
            campaign_pda: next_account_info(account_iter)?,
            chain_link: next_account_info(account_iter)?,
            feed1: next_account_info(account_iter)?,
            feed2: next_account_info(account_iter)?,
            feed3: next_account_info(account_iter)?,
        })
    }
}

fn all_ticket_owners(members: &Vec<(u16, u8)>, max_index: u16) -> Result<Vec<u16>,ProgramError> {
    let mut all_tickets: Vec<u16> = vec![];


    for (index, tickets) in members {
        assert_with_message(
            max_index >= *index,
            ProgramError::InvalidInstructionData,
            "Wrong user index."
        )?;
        assert_with_message(
            !all_tickets.contains(index),
            ProgramError::InvalidInstructionData,
            "Wrong user index."
        )?;
        for _ in 0..*tickets {
            all_tickets.push(*index);
        }
    }

   Ok( all_tickets )
}

fn get_random(accounts: &Accounts, max_value: u16) -> Result<u16, ProgramError> {
    let clock = Clock::get()?;

    let feed1 =
        chainlink_solana::latest_round_data(accounts.chain_link.clone(), accounts.feed1.clone())?
            .answer;

    let feed2 =
        chainlink_solana::latest_round_data(accounts.chain_link.clone(), accounts.feed2.clone())?
            .answer;

    let feed3 =
        chainlink_solana::latest_round_data(accounts.chain_link.clone(), accounts.feed3.clone())?
            .answer;

    let final_val = if clock.unix_timestamp % 2 == 0 {
        (feed1 * feed2 * feed3) as u128
    } else {
        (feed3 * feed2) as u128
    };

    let result = final_val % max_value as u128;

    Ok(result as u16)
}
