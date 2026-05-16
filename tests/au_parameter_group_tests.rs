mod support;

use audiounit::prelude::*;

#[test]
fn au_parameter_group_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let unit = AuAudioUnit::instantiate(
        support::default_output_description(),
        InstantiationOptions::InProcess,
    )?;
    let tree = unit.parameter_tree().expect("missing parameter tree");
    let group = tree.root_group();
    let info = group.info()?;
    assert!(info.kind == "tree" || info.kind == "group");
    assert!(!group.children()?.is_empty() || !group.all_parameters()?.is_empty());
    Ok(())
}
