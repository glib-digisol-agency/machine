use {
    borsh::BorshDeserialize,
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program::invoke_signed,
        program_error::ProgramError,
        pubkey::Pubkey,
    },
};

use crate::processor::data_handlers::UserData;
use crate::processor::{consts::ADMIN, data_handlers::LotteryCampaign, utils::assert_with_message};

pub fn claim(program_id: &Pubkey, accounts: &[AccountInfo], campaign_index: u16) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    assert_with_message(
        accounts.winner.is_signer,
        ProgramError::InvalidAccountData,
        "Winner must be a signer.",
    )?;

    let (lottery_machine_pda, _) = Pubkey::find_program_address(&[ADMIN.as_ref()], program_id);
    let (campaign_pda, _bump) = Pubkey::find_program_address(
        &[lottery_machine_pda.as_ref(), &campaign_index.to_le_bytes()],
        program_id,
    );

    assert_with_message(
        &campaign_pda == accounts.campaign_pda.key,
        ProgramError::InvalidAccountData,
        "Wrong campaign PDA.",
    )?;

    let (user_account_pda, bump) = Pubkey::find_program_address(
        &[accounts.winner.key.as_ref(), campaign_pda.as_ref()],
        program_id,
    );
    assert_with_message(
        &user_account_pda == accounts.winner_account_pda.key,
        ProgramError::InvalidAccountData,
        "Campaign in progress.",
    )?;

    let lottery_campaign: LotteryCampaign =
        LotteryCampaign::try_from_slice(*(*accounts.campaign_pda.data).borrow())?;

    let user_data: UserData =
        UserData::try_from_slice(*(*accounts.winner_account_pda.data).borrow())?;

    assert_with_message(
        lottery_campaign.is_finished,
        ProgramError::InvalidAccountData,
        "Campaign in progress.",
    )?;
    assert_with_message(
        user_data.user_index == lottery_campaign.winner,
        ProgramError::InvalidAccountData,
        "Wrong winner PDA.",
    )?;

    let nft_account_pda = spl_associated_token_account::get_associated_token_address(
        accounts.campaign_pda.key,
        &lottery_campaign.nft,
    );
    assert_with_message(
        accounts.campaign_token_acc.key == &nft_account_pda,
        ProgramError::InvalidAccountData,
        "Wrong winner PDA.",
    )?;

    let transfer_ix = spl_token::instruction::transfer(
        accounts.token_program.key,
        accounts.campaign_token_acc.key,
        accounts.user_token_account.key,
        accounts.campaign_pda.key,
        &[],
        1,
    )?;
    let seed = &[
        lottery_machine_pda.as_ref(),
        &campaign_index.to_le_bytes(),
        &[bump],
    ];
    let sign = [&seed[..]];

    invoke_signed(
        &transfer_ix,
        &[
            accounts.campaign_token_acc.clone(),
            accounts.user_token_account.clone(),
            accounts.campaign_pda.clone(),
        ],
        &sign,
    )?;

    msg!("Winner got his reward.");

    Ok(())
}

struct Accounts<'a, 'b> {
    pub winner: &'a AccountInfo<'b>,
    pub winner_account_pda: &'a AccountInfo<'b>,
    pub campaign_pda: &'a AccountInfo<'b>,
    pub campaign_token_acc: &'a AccountInfo<'b>,
    pub user_token_account: &'a AccountInfo<'b>,
    pub token_program: &'a AccountInfo<'b>,
}

impl<'a, 'b> Accounts<'a, 'b> {
    fn new(accounts: &'a [AccountInfo<'b>]) -> Result<Self, ProgramError> {
        let account_iter = &mut accounts.iter();

        Ok(Self {
            winner: next_account_info(account_iter)?,
            winner_account_pda: next_account_info(account_iter)?,
            campaign_pda: next_account_info(account_iter)?,
            campaign_token_acc: next_account_info(account_iter)?,
            user_token_account: next_account_info(account_iter)?,
            token_program: next_account_info(account_iter)?,
        })
    }
}
