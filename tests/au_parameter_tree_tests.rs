mod support;

use audiounit::prelude::*;

#[test]
fn au_parameter_tree_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let unit = AuAudioUnit::instantiate(
        support::default_output_description(),
        InstantiationOptions::InProcess,
    )?;
    let tree = unit.parameter_tree().expect("missing parameter tree");
    let info = tree.info()?;
    assert_eq!(info.kind, "tree");
    assert!(!tree.to_json().is_empty());
    assert!(!tree.root_group().all_parameters()?.is_empty());
    Ok(())
}
