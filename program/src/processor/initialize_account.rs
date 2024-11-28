use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        pubkey::Pubkey,
    },
};

use crate::processor::consts::MAX_USERS;
use crate::processor::{
    data_handlers::{Initialize, LotteryCampaign, UserData},
    utils::assert_with_message,
};

/// Initialize new user account, to handle user index.
pub fn initialize_account(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    // Derive PDA from ticket buyer address and campaign PDA.
    let (user_data_pda, bump) = Pubkey::find_program_address(
        &[
            accounts.ticket_buyer.key.as_ref(),
            accounts.campaign_pda.key.as_ref(),
        ],
        program_id,
    );

    assert_with_message(
        accounts.ticket_buyer.is_signer,
        ProgramError::InvalidAccountData,
        "Payer should be a signer.",
    )?;

    assert_with_message(
        accounts.ticket_buyer_pda.key == &user_data_pda,
        ProgramError::InvalidAccountData,
        "Wrong ticket buyer pda.",
    )?;

    assert_with_message(
        accounts.ticket_buyer_pda.data_is_empty(),
        ProgramError::InvalidAccountData,
        "Has already had account.",
    )?;

    // Campaign data, for user index.
    let mut campaign_data: LotteryCampaign =
        LotteryCampaign::try_from_slice(*(*accounts.campaign_pda.data).borrow())?;

    assert_with_message(
        campaign_data.user_index != MAX_USERS,
        ProgramError::AccountDataTooSmall,
        "User limit expired.",
    )?;

    let users_count = campaign_data.user_index;
    let data = UserData::new(users_count + 1);

    let seed = &[
        accounts.ticket_buyer.key.as_ref(),
        accounts.campaign_pda.key.as_ref(),
        &[bump],
    ];

    let sign = [&seed[..]];

    data.create_account(
        accounts.ticket_buyer,
        accounts.ticket_buyer_pda,
        accounts.system_account,
        program_id,
        &sign,
    )?;

    data.pay_rent(
        accounts.ticket_buyer,
        accounts.ticket_buyer_pda,
        accounts.rent_account,
    )?;

    campaign_data.user_index += 1;
    campaign_data.serialize(&mut &mut *(accounts.campaign_pda).try_borrow_mut_data()?)?;

    data.serialize(&mut &mut *(accounts.ticket_buyer_pda).try_borrow_mut_data()?)?;

    msg!("Account saved");

    Ok(())
}

struct Accounts<'a, 'b> {
    pub ticket_buyer: &'a AccountInfo<'b>,
    pub ticket_buyer_pda: &'a AccountInfo<'b>,
    pub campaign_pda: &'a AccountInfo<'b>,
    pub system_account: &'a AccountInfo<'b>,
    pub rent_account: &'a AccountInfo<'b>,
}

impl<'a, 'b> Accounts<'a, 'b> {
    fn new(accounts: &'a [AccountInfo<'b>]) -> Result<Self, ProgramError> {
        let account_iter = &mut accounts.iter();

        Ok(Self {
            ticket_buyer: next_account_info(account_iter)?,
            ticket_buyer_pda: next_account_info(account_iter)?,
            campaign_pda: next_account_info(account_iter)?,
            system_account: next_account_info(account_iter)?,
            rent_account: next_account_info(account_iter)?,
        })
    }
}
