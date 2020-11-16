// E - UI - styles
// Desmond Germans, 2020

// All style structs in a separate module.

use {
    crate::*,
};

/// Book style.
pub struct Book {
    pub font: Rc<Font>,
    pub text_color: u32,
    pub disabled_text_color: u32,
    pub color: u32,
    pub hover_color: u32,
    pub current_color: u32,
    pub background_color: u32,
}

/// Button Style.
pub struct Button {
    pub font: Rc<Font>,
    pub text_color: u32,
    pub disabled_text_color: u32,
    pub color: u32,
    pub hover_color: u32,
    pub disabled_color: u32,
    pub pressed_color: u32,
}

/// Text input field style.
pub struct Field {
    pub font: Rc<Font>,
    pub color: u32,
    pub text_color: u32,
    pub disabled_text_color: u32,
}

/// List style.
pub struct List {
    pub font: Rc<Font>,
}

/// Menu style.
pub struct Menu {
    pub font: Rc<Font>,
    pub item_text_color: u32,
    pub item_disabled_text_color: u32,
    pub item_color: u32,
    pub item_hover_color: u32,
    pub item_disabled_color: u32,
    pub item_current_color: u32,
}

/// Menu bar style.
pub struct MenuBar {
    pub font: Rc<Font>,
    pub item_text_color: u32,
    pub item_color: u32,
    pub item_hover_color: u32,
    pub item_current_color: u32,
}

/// Progress indicator style.
pub struct Progress {
    pub full_color: u32,
    pub empty_color: u32,
    pub disabled_color: u32,
}

/// Horizontal or vertical scroll bar style.
pub struct ScrollBar {
    pub step_color: u32,
    pub step_hover_color: u32,
    pub page_color: u32,
    pub page_hover_color: u32,
    pub tab_color: u32,
    pub tab_hover_color: u32,
}

/// Horizontal or vertical slider style.
pub struct Slider {
    pub color: u32,
    pub empty_color: u32,
    pub full_color: u32,
    pub tab_color: u32,
    pub tab_hover_color: u32,
    pub disabled_color: u32,
}

/// Horizontal or vertical splitter style.
pub struct Splitter {
    pub color: u32,
    pub hover_color: u32,
    pub disabled_color: u32,
}

/// Horizontal or vertical stack style.
pub struct Stack {

}

/// Text style.
pub struct Text {
    pub font: Rc<Font>,
    pub color: u32,
    pub text_color: u32,
}

/// On/off toggle style.
pub struct Toggle {
    pub color: u32,
    pub empty_color: u32,
    pub full_color: u32,
    pub tab_color: u32,
    pub tab_hover_color: u32,
    pub disabled_color: u32,
}

/// Tool bar style.
pub struct ToolBar {
    pub item_text_color: u32,
    pub item_color: u32,
    pub item_hover_color: u32,
}

/// Tool tip style.
pub struct ToolTip {
    pub font: Rc<Font>,
    pub text_color: u32,
}

/// Tree style.
pub struct Tree {
    pub font: Rc<Font>,
}
