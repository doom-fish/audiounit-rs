mod support;

use audiounit::prelude::*;

#[test]
fn component_manager_predicate_and_metadata_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let components =
        ComponentManager::components_matching_predicate(&ComponentPredicate::all(vec![
            ComponentPredicate::type_name_contains("Effect"),
            ComponentPredicate::manufacturer_name_contains("Apple"),
            ComponentPredicate::name_contains("Peak"),
        ]))?;
    assert!(!components.is_empty());

    let limiter = ComponentManager::components_matching(support::peak_limiter_description())?
        .into_iter()
        .next()
        .ok_or("peak limiter component not found")?;

    let user_tags = limiter.user_tag_names()?;
    let all_tags = limiter.all_tag_names()?;
    let architectures = limiter.available_architectures()?;
    let configuration = limiter.configuration_dictionary()?;

    assert_eq!(limiter.tags(), all_tags);
    assert!(architectures.iter().all(|arch| *arch != 0));
    assert!(configuration.is_object());
    assert!(limiter.supports_number_input_channels(2, 2));
    assert!(user_tags.len() <= all_tags.len());
    Ok(())
}

#[test]
fn component_manager_passing_test_can_short_circuit() -> Result<(), Box<dyn std::error::Error>> {
    let components = ComponentManager::components_passing_test(|component, stop| {
        let matches = component.type_name().contains("Effect")
            && component.manufacturer_name().contains("Apple");
        if matches {
            *stop = true;
        }
        matches
    })?;

    assert_eq!(components.len(), 1);
    assert!(components[0].manufacturer_name().contains("Apple"));
    Ok(())
}
