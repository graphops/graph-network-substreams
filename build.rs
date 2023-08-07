use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("erc20", "./abis/erc20.json")?
        .generate()?
        .write_to_file("src/abi/erc20.rs")?;
    Abigen::new("staking", "./abis/staking.json")?
        .generate()?
        .write_to_file("src/abi/staking.rs")?;
    Abigen::new("rewardsManager", "./abis/rewardsManager.json")?
        .generate()?
        .write_to_file("src/abi/rewards_manager.rs")?;
    Abigen::new("curation", "./abis/curation.json")?
        .generate()?
        .write_to_file("src/abi/curation.rs")?;
    Abigen::new("gns", "./abis/gns.json")?
        .generate()?
        .write_to_file("src/abi/gns.rs")?;
    Abigen::new("controller", "./abis/controller.json")?
        .generate()?
        .write_to_file("src/abi/controller.rs")?;
    Ok(())
}
