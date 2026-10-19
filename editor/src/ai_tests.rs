#[test]
fn ai_panel_is_available_from_the_view_menu() {
    assert!(
        crate::ui::Tab::ALL
            .iter()
            .any(|tab| tab.title() == "AI Assistant")
    );
}
