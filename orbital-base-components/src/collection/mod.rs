//! Role-agnostic collection behavior — registry, expansion, selection, focus, keyboard.

pub mod primitives;
pub mod state;

pub use state::{
    collection_keyboard_action, shift_range_in_visible, typeahead_match, CollectionCheckboxState,
    CollectionExpansionState, CollectionFocusState, CollectionItemDomRegistry,
    CollectionKeyboardAction, CollectionRegistry, CollectionRegistryEntry, CollectionSelectionMode,
    CollectionSelectionState, CollectionState, CollectionStateInjection, ExpansionTrigger,
};
