mod automate;
mod bury;
mod buyback;
mod checkpoint;
mod claim_ore;
mod claim_sol;
// DEPRECATED: ORE Staking modules (replaced by Garage system)
// mod claim_yield;
mod close;
// mod compound_yield;
mod deploy;
// mod deposit;
mod initialize;
mod liq;
mod log;
mod new_var;
mod reload_sol;
mod reset;
mod set_admin;
// mod withdraw;
mod wrap;

// Garage modules
mod boost;
mod claim_wallet;
mod collect;
mod fuel_up;
mod stash;

use automate::*;
use bury::*;
use buyback::*;
use checkpoint::*;
use claim_ore::*;
use claim_sol::*;
// DEPRECATED: ORE Staking imports (replaced by Garage system)
// use claim_yield::*;
use close::*;
// use compound_yield::*;
use deploy::*;
// use deposit::*;
use initialize::*;
use liq::*;
use log::*;
use new_var::*;
use reload_sol::*;
use reset::*;
use set_admin::*;
// use withdraw::*;
use wrap::*;

// Garage imports
use boost::*;
use claim_wallet::*;
use collect::*;
use fuel_up::*;
use stash::*;

use speedway_api::instruction::*;
use solana_security_txt::security_txt;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&speedway_api::ID, program_id, data)?;

    match ix {
        // Miner
        OreInstruction::Automate => process_automate(accounts, data)?,
        OreInstruction::Checkpoint => process_checkpoint(accounts, data)?,
        OreInstruction::ClaimSOL => process_claim_sol(accounts, data)?,
        OreInstruction::ClaimORE => process_claim_ore(accounts, data)?,
        OreInstruction::Deploy => process_deploy(accounts, data)?,
        OreInstruction::Log => process_log(accounts, data)?,
        OreInstruction::Close => process_close(accounts, data)?,
        OreInstruction::Reset => process_reset(accounts, data)?,
        OreInstruction::ReloadSOL => process_reload_sol(accounts, data)?,

        // DEPRECATED: ORE Staker instructions (replaced by Garage system)
        // OreInstruction::Deposit => process_deposit(accounts, data)?,
        // OreInstruction::Withdraw => process_withdraw(accounts, data)?,
        // OreInstruction::ClaimYield => process_claim_yield(accounts, data)?,
        // OreInstruction::CompoundYield => process_compound_yield(accounts, data)?,

        // Garage
        OreInstruction::FuelUp => process_fuel_up(accounts, data)?,
        OreInstruction::Boost => process_boost(accounts, data)?,
        OreInstruction::Collect => process_collect(accounts, data)?,
        OreInstruction::Stash => process_stash(accounts, data)?,
        OreInstruction::ClaimWallet => process_claim_wallet(accounts, data)?,

        // Admin
        OreInstruction::Buyback => process_buyback(accounts, data)?,
        OreInstruction::Bury => process_bury(accounts, data)?,
        OreInstruction::Wrap => process_wrap(accounts, data)?,
        OreInstruction::SetAdmin => process_set_admin(accounts, data)?,
        OreInstruction::NewVar => process_new_var(accounts, data)?,
        OreInstruction::Liq => process_liq(accounts, data)?,
        OreInstruction::Initialize => process_initialize(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);

security_txt! {
    name: "Solana Speedway",
    project_url: "https://solanaspeedway.com",
    contacts: "email:security@solanaspeedway.com",
    policy: "https://github.com/solanaspeedway/speedway/blob/master/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/solanaspeedway/speedway"
}
