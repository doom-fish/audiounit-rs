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
    let group = tree.root_group();
    let info = group.info()?;
    println!("group {} has {} children / {} parameters", info.display_name, group.children()?.len(), group.all_parameters()?.len());
    Ok(())
}
