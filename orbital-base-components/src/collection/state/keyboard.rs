use leptos::ev;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionKeyboardAction {
    None,
    Next,
    Previous,
    Expand,
    Collapse,
    Select,
    Home,
    End,
    TypeAhead,
}

pub fn collection_keyboard_action(event: &ev::KeyboardEvent) -> CollectionKeyboardAction {
    let key = event.key();
    match key.as_str() {
        "ArrowDown" => CollectionKeyboardAction::Next,
        "ArrowUp" => CollectionKeyboardAction::Previous,
        "ArrowRight" => CollectionKeyboardAction::Expand,
        "ArrowLeft" => CollectionKeyboardAction::Collapse,
        "Enter" | " " => CollectionKeyboardAction::Select,
        "Home" => CollectionKeyboardAction::Home,
        "End" => CollectionKeyboardAction::End,
        text if text.len() == 1 && !event.ctrl_key() && !event.meta_key() && !event.alt_key() => {
            CollectionKeyboardAction::TypeAhead
        }
        _ => CollectionKeyboardAction::None,
    }
}
