use solana_program::{entrypoint::ProgramResult, msg, program_error::ProgramError};

pub fn assert_with_message(to_assert: bool, error: ProgramError, msg: &str) -> ProgramResult {
    if !to_assert {
        msg!("{}", msg);
        return Err(error);
    }
    Ok(())
}
