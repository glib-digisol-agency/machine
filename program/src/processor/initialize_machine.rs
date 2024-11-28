use {
    borsh::BorshSerialize,
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        pubkey::Pubkey,
    },
};

use crate::processor::{
    consts::ADMIN,
    data_handlers::{Initialize, LotteryMachine},
    utils::assert_with_message,
};

/// Create new Lottery Machine.
pub fn initialize_machine(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts = Accounts::new(accounts)?;

    //Derive Machine PDA from executor address
    let (machine_pda, bump) =
        Pubkey::find_program_address(&[accounts.payer.key.as_ref()], program_id);
    assert_with_message(
        &ADMIN == accounts.payer.key,
        ProgramError::InvalidAccountData,
        "Only admin can call this instruction.",
    )?;

    assert_with_message(
        accounts.payer.is_signer,
        ProgramError::InvalidAccountData,
        "Payer should be a singer.",
    )?;

    assert_with_message(
        &machine_pda == accounts.machine_pda.key,
        ProgramError::InvalidAccountData,
        "Wrong machine PDA.",
    )?;
    let data = LotteryMachine::default();

    let seed = &[accounts.payer.key.as_ref(), &[bump]];

    let sign = [&seed[..]];

    data.create_account(
        accounts.payer,
        accounts.machine_pda,
        accounts.system_account,
        program_id,
        &sign,
    )?;

    data.pay_rent(accounts.payer, accounts.machine_pda, accounts.rent_account)?;

    data.serialize(&mut &mut *(accounts.machine_pda).try_borrow_mut_data()?)?;

    msg!("Lottery machine initialized!.");

    Ok(())
}

struct Accounts<'a, 'b> {
    pub payer: &'a AccountInfo<'b>,
    pub machine_pda: &'a AccountInfo<'b>,
    pub system_account: &'a AccountInfo<'b>,
    pub rent_account: &'a AccountInfo<'b>,
}

impl<'a, 'b> Accounts<'a, 'b> {
    fn new(accounts: &'a [AccountInfo<'b>]) -> Result<Self, ProgramError> {
        let account_iter = &mut accounts.iter();

        Ok(Self {
            payer: next_account_info(account_iter)?,
            machine_pda: next_account_info(account_iter)?,
            system_account: next_account_info(account_iter)?,
            rent_account: next_account_info(account_iter)?,
        })
    }
}
