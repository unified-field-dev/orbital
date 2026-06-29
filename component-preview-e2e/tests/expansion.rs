use component_preview_e2e::fixtures::{FixtureButton, FixtureDocPanel};

#[test]
fn e01_fixture_compiles() {
    let _ = FixtureButton;
    let _ = FixtureDocPanel;
}

#[test]
fn e02_doc_constants_exist() {
    let _doc = component_preview_e2e::fixtures::FIXTUREBUTTON_DOC;
    let _props = component_preview_e2e::fixtures::FIXTUREBUTTON_PROPS;
    let _doc_panel_doc = component_preview_e2e::fixtures::FIXTUREDOCPANEL_DOC;
    let _doc_panel_props = component_preview_e2e::fixtures::FIXTUREDOCPANEL_PROPS;
}

#[test]
fn e03_fixture_doc_panel_description_excludes_fenced_code() {
    let description = component_preview_e2e::fixtures::FIXTUREDOCPANEL_DESCRIPTION;
    assert!(description.contains("Register this fixture"));
    assert!(!description.contains("view!"));
    assert!(!description.contains("FixtureDocPanel label=\"hidden\""));
}

#[test]
fn e04_fixture_doc_panel_best_practices_separate() {
    let description = component_preview_e2e::fixtures::FIXTUREDOCPANEL_DESCRIPTION;
    let best = component_preview_e2e::fixtures::FIXTUREDOCPANEL_BEST_PRACTICES;
    assert!(best.contains("Keep fixture doc strings stable"));
    assert!(!description.contains("Keep fixture doc strings stable"));
    assert!(!best.is_empty());
    assert!(!component_preview_e2e::fixtures::FIXTUREDOCPANEL_PROPS.is_empty());
}
