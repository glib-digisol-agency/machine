use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program::invoke,
        program_error::ProgramError,
        pubkey::Pubkey,
    },
};

use crate::processor::{
    consts::ADMIN,
    data_handlers::{Initialize, LotteryCampaign, LotteryMachine},
    utils::assert_with_message,
};

/// Create new campaign.
pub fn initialize_campaign(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    price: u32,
) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    msg!("WALLET {}", accounts.wallet.key);

    // Data holds campaign indexes.
    let (lottery_pda, _) = Pubkey::find_program_address(&[ADMIN.as_ref()], program_id);
    assert_with_message(
        &lottery_pda == accounts.lottery_machine_pda.key,
        ProgramError::InvalidAccountData,
        "Wrong lottery PDA.",
    )?;

    let mut lottery_machine: LotteryMachine =
        LotteryMachine::try_from_slice(*(*accounts.lottery_machine_pda.data).borrow())?;

    assert_with_message(
        &ADMIN == accounts.payer.key || accounts.payer.key == &lottery_machine.admin,
        ProgramError::InvalidAccountData,
        "Only admin can call this instruction.",
    )?;

    assert_with_message(
        accounts.payer.is_signer,
        ProgramError::InvalidAccountData,
        "Payer should be a signer.",
    )?;

    let nft_account_pda = spl_associated_token_account::get_associated_token_address(
        accounts.pda.key,
        accounts.nft.key,
    );
    assert_with_message(
        &nft_account_pda == accounts.assoc_campaign_nft_account.key,
        ProgramError::InvalidAccountData,
        "Invalid campaign valet.",
    )?;

    // Create a token account for storing Nft during the campaign.
    let assoc_ix = spl_associated_token_account::instruction::create_associated_token_account(
        accounts.payer.key,
        accounts.pda.key,
        accounts.nft.key,
        accounts.token_program.key,
    );
    invoke(
        &assoc_ix,
        &[
            accounts.payer.clone(),
            accounts.assoc_campaign_nft_account.clone(),
            accounts.pda.clone(),
            accounts.nft.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.toke_accos.clone(),
        ],
    )?;

    // Transfer Nft from Admin account to campaign account.
    let transfer_ix = spl_token::instruction::transfer(
        accounts.token_program.key,
        accounts.assoc_nft_account.key,
        accounts.assoc_campaign_nft_account.key,
        accounts.payer.key,
        &[],
        1,
    )?;
    invoke(
        &transfer_ix,
        &[
            accounts.assoc_nft_account.clone(),
            accounts.assoc_campaign_nft_account.clone(),
            accounts.payer.clone(),
        ],
    )?;

    lottery_machine.campaign_index += 1;
    let campaign_index = lottery_machine.campaign_index;

    // Create new campaign PDA from Lottery Machine address and index.
    let (pda, bump) = Pubkey::find_program_address(
        &[
            accounts.lottery_machine_pda.key.as_ref(),
            &campaign_index.to_le_bytes(),
        ],
        program_id,
    );
    assert_with_message(
        &pda == accounts.pda.key,
        ProgramError::InvalidAccountData,
        "Wrong campaign PDA.",
    )?;

    let seed = &[
        accounts.lottery_machine_pda.key.as_ref(),
        &campaign_index.to_le_bytes()[..],
        &[bump],
    ];

    let sign = [&seed[..]];

    let campaign_data = LotteryCampaign::new(
        price,
        *accounts.nft.key,
        *accounts.wallet.key,
        lottery_machine.campaign_index,
    );

    campaign_data.create_account(
        accounts.payer,
        accounts.pda,
        accounts.system_program,
        program_id,
        &sign,
    )?;

    campaign_data.pay_rent(accounts.payer, accounts.pda, accounts.rent_account)?;

    campaign_data.serialize(&mut &mut *(accounts.pda).try_borrow_mut_data()?)?;

    lottery_machine.serialize(&mut &mut *(accounts.lottery_machine_pda).try_borrow_mut_data()?)?;

    msg!("Campaign saved, currant data {:?}.", campaign_data);

    Ok(())
}

struct Accounts<'a, 'b> {
    pub payer: &'a AccountInfo<'b>,
    pub pda: &'a AccountInfo<'b>,
    pub lottery_machine_pda: &'a AccountInfo<'b>,
    pub nft: &'a AccountInfo<'b>,
    pub assoc_nft_account: &'a AccountInfo<'b>,
    pub assoc_campaign_nft_account: &'a AccountInfo<'b>,
    pub wallet: &'a AccountInfo<'b>,
    pub token_program: &'a AccountInfo<'b>,
    pub toke_accos: &'a AccountInfo<'b>,
    pub system_program: &'a AccountInfo<'b>,
    pub rent_account: &'a AccountInfo<'b>,
}

impl<'a, 'b> Accounts<'a, 'b> {
    fn new(accounts: &'a [AccountInfo<'b>]) -> Result<Self, ProgramError> {
        let account_iter = &mut accounts.iter();

        Ok(Self {
            payer: next_account_info(account_iter)?,
            pda: next_account_info(account_iter)?,
            lottery_machine_pda: next_account_info(account_iter)?,
            nft: next_account_info(account_iter)?,
            assoc_nft_account: next_account_info(account_iter)?,
            assoc_campaign_nft_account: next_account_info(account_iter)?,
            wallet: next_account_info(account_iter)?,
            token_program: next_account_info(account_iter)?,
            toke_accos: next_account_info(account_iter)?,
            system_program: next_account_info(account_iter)?,
            rent_account: next_account_info(account_iter)?,
        })
    }
}
