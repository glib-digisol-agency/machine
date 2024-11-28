use crate::processor::{consts::ADMIN, data_handlers::LotteryMachine, utils::assert_with_message};
use solana_program::msg;
use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        program_error::ProgramError,
        pubkey::Pubkey,
    },
};

pub fn set_manager(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    msg!(" Step 1 ");
    let admin = next_account_info(account_iter)?;
    let lottery_machine = next_account_info(account_iter)?;
    let manager = next_account_info(account_iter)?;

    msg!(" Step 2 ");
    assert_with_message(
        admin.is_signer,
        ProgramError::InvalidAccountData,
        "User must be a signer.",
    )?;

    let (lottery_machine_pda, _) = Pubkey::find_program_address(&[ADMIN.as_ref()], program_id);
    msg!(" Step 3 ");
    assert_with_message(
        &lottery_machine_pda == lottery_machine.key,
        ProgramError::InvalidAccountData,
        "Wrong lottery machine PDA.",
    )?;

    assert_with_message(
        &ADMIN == admin.key,
        ProgramError::InvalidAccountData,
        "Only for Admin.",
    )?;

    let mut machine: LotteryMachine =
        LotteryMachine::try_from_slice(*(*lottery_machine.data).borrow())?;

    machine.admin = *manager.key;

    machine.serialize(&mut *(*lottery_machine).try_borrow_mut_data()?)?;

    Ok(())
}
