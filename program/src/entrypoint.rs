use {
    num_traits::FromPrimitive,
    solana_program::{
        account_info::AccountInfo,
        entrypoint::ProgramResult,
        program_error::ProgramError,
        pubkey::Pubkey,
        {entrypoint, msg},
    },
};

use crate::{error::ContractError, processor::Processor};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = Processor::process(program_id, accounts, instruction_data) {
        if let ProgramError::Custom(code) = error {
            msg!("Custom error: {:?} ", ContractError::from_u32(code));
        }
        return Err(error);
    }

    Ok(())
}
