use solana_program::program::invoke;
use solana_program::system_instruction;

use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        pubkey::Pubkey,
    },
    std::borrow::Borrow,
};

use crate::processor::consts::ADMIN;
use crate::processor::{
    data_handlers::{Initialize, LotteryCampaign, TicketData, UserData},
    utils::assert_with_message,
};

pub fn buy_ticket(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u16,
    campaign_index: u16,
) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    let (lottery_machine_pda, _) = Pubkey::find_program_address(&[ADMIN.as_ref()], program_id);
    let (campaign_pda, _) = Pubkey::find_program_address(
        &[
            lottery_machine_pda.as_ref(),
            campaign_index.to_le_bytes().as_ref(),
        ],
        program_id,
    );
    assert_with_message(
        &campaign_pda == accounts.campaign_pda.key,
        ProgramError::InvalidAccountData,
        "Wrong campaign PDA or Campaign index.",
    )?;

    // Get lottery campaign to chek and change tickets amount.
    let mut campaign: LotteryCampaign =
        LotteryCampaign::try_from_slice(*(*accounts.campaign_pda.data).borrow())?;

    assert_with_message(
        accounts.ticket_buyer.is_signer,
        ProgramError::InvalidAccountData,
        "Payer should be a signer.",
    )?;

    assert_with_message(
        !campaign.is_finished,
        ProgramError::InvalidAccountData,
        "Campaign finished.",
    )?;

    // Validate ticket amount.
    campaign.is_enough_tickets(amount)?;

    // Data hold user index.
    let user_account_data: UserData =
        UserData::try_from_slice(*(*accounts.user_data_pda.data).borrow())?;

    // Get PDA from campaign and user index.
    let (user_ticket_index_pda, bump) = Pubkey::find_program_address(
        &[
            accounts.campaign_pda.key.as_ref(),
            &user_account_data.user_index.to_le_bytes(),
        ],
        program_id,
    );
    assert_with_message(
        &user_ticket_index_pda == accounts.ticket_buyer_pda.key,
        ProgramError::InvalidAccountData,
        "Not corresponding buyer's PDA",
    )?;
    // Create new tickets account if it not exist,
    // or take old with new amount of tickets.
    let ticket_data = if accounts.ticket_buyer_pda.data_is_empty() {
        let account_data = TicketData::new(*accounts.ticket_buyer.key, amount, campaign_index);

        let seed = &[
            accounts.campaign_pda.key.as_ref(),
            &user_account_data.user_index.to_le_bytes(),
            &[bump],
        ];

        let sign = [&seed[..]];

        account_data.create_account(
            accounts.ticket_buyer,
            accounts.ticket_buyer_pda,
            accounts.system_account,
            program_id,
            &sign,
        )?;

        account_data.pay_rent(
            accounts.ticket_buyer,
            accounts.ticket_buyer_pda,
            accounts.rent_account,
        )?;

        account_data
    } else {
        let data = (*accounts.ticket_buyer_pda.data).borrow();
        let mut ticket_data: TicketData = TicketData::try_from_slice(data.borrow())?;

        ticket_data.tickets += amount;
        ticket_data
    };

    assert_with_message(
        &campaign.wallet == accounts.wallet.key,
        ProgramError::InvalidAccountData,
        "Wrong wallet.",
    )?;

    let total_price = campaign.price as u64 * amount as u64;

    let payment_ix =
        &system_instruction::transfer(accounts.ticket_buyer.key, &campaign.wallet, total_price);

    invoke(
        payment_ix,
        &[accounts.ticket_buyer.clone(), accounts.wallet.clone()],
    )?;

    ticket_data.serialize(&mut &mut *(*accounts.ticket_buyer_pda.data).borrow_mut())?;

    campaign.bought_tickets += amount;
    campaign.serialize(&mut &mut *(*accounts.campaign_pda.data).borrow_mut())?;

    msg!(
        "Account saved. Account info {:?}. Current campaign state {:?} ",
        &ticket_data,
        &campaign
    );

    Ok(())
}

struct Accounts<'a, 'b> {
    pub ticket_buyer: &'a AccountInfo<'b>,
    pub ticket_buyer_pda: &'a AccountInfo<'b>,
    pub user_data_pda: &'a AccountInfo<'b>,
    pub wallet: &'a AccountInfo<'b>,
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
            user_data_pda: next_account_info(account_iter)?,
            wallet: next_account_info(account_iter)?,
            campaign_pda: next_account_info(account_iter)?,
            system_account: next_account_info(account_iter)?,
            rent_account: next_account_info(account_iter)?,
        })
    }
}
