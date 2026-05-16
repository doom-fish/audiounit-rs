mod support;

use audiounit::prelude::*;

fn main() -> Result<(), AuError> {
    let unit = AuAudioUnit::instantiate(
        support::default_output_description(),
        InstantiationOptions::InProcess,
    )?;
    let tree = unit
        .parameter_tree()
        .ok_or_else(|| AuError::Unavailable("default output had no parameter tree".to_owned()))?;
    let info = tree.info()?;
    println!(
        "parameter tree kind={} children={}",
        info.kind,
        info.children.len()
    );
    println!("json bytes={}", tree.to_json().len());
    Ok(())
}
