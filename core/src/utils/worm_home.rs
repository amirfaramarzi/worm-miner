use std::path::PathBuf;

use anyhow::anyhow;

pub fn get_worm_home() -> Result<PathBuf, anyhow::Error> {
    let home = std::env::home_dir().ok_or(anyhow!("no home directory"))?;
    Ok(home.join(".worm-miner"))
}

pub fn get_proof_of_burn_dat() -> Result<PathBuf, anyhow::Error> {
    Ok(get_worm_home()?.join("proof_of_burn.dat"))
}

pub fn get_proof_of_burn_zkey() -> Result<PathBuf, anyhow::Error> {
    Ok(get_worm_home()?.join("proof_of_burn.zkey"))
}

pub fn get_spend_dat() -> Result<PathBuf, anyhow::Error> {
    Ok(get_worm_home()?.join("spend.dat"))
}

pub fn get_spend_zkey() -> Result<PathBuf, anyhow::Error> {
    Ok(get_worm_home()?.join("spend.zkey"))
}

pub fn get_config() -> Result<PathBuf, anyhow::Error> {
    Ok(get_worm_home()?.join("server_config.json"))
}
