// E - UI - Styles
// Desmond Germans, 2020

// This contains all the styles for each control.

use crate::*;
use std::rc::Rc;

/// Styles struct.
pub struct Styles {
    pub proto_sans: Rc<FontProto>,
    pub proto_serif: Rc<FontProto>,
    pub proto_mono: Rc<FontProto>,
    pub font: Rc<Font>,
    pub accordeon_tab_text_color: u32,
    pub accordeon_tab_color: u32,
    pub accordeon_tab_hover_color: u32,
    pub accordeon_tab_current_color: u32,
    pub book_tab_text_color: u32,
    pub book_tab_color: u32,
    pub book_tab_hover_color: u32,
    pub book_tab_current_color: u32,
    pub book_tab_background_color: u32,
    pub button_text_color: u32,
    pub button_disabled_text_color: u32,
    pub button_color: u32,
    pub button_hover_color: u32,
    pub button_disabled_color: u32,
    pub button_pressed_color: u32,
    // field
    // list
    pub menu_item_text_color: u32,
    pub menu_item_disabled_text_color: u32,
    pub menu_item_color: u32,
    pub menu_item_hover_color: u32,
    pub menu_item_disabled_color: u32,
    pub menu_item_current_color: u32,
    pub menubar_item_text_color: u32,
    pub menubar_item_color: u32,
    pub menubar_item_hover_color: u32,
    pub menubar_item_current_color: u32,
    // progress
    // scrollbar
    // slider
    // stepper
    pub text_color: u32,
    // toggle
    pub toolbar_item_text_color: u32,
    pub toolbar_item_color: u32,
    pub toolbar_item_hover_color: u32,
    // tree
}

impl Styles {
    pub fn new_default(graphics: &Graphics,font_path: &str) -> Result<Styles,SystemError> {
        let proto_sans = Rc::new(
            FontProto::new(
                graphics,
                &format!("{}/sans.fnt",font_path)
            )?
        );
        let proto_serif = Rc::new(
            FontProto::new(
                graphics,
                &format!("{}/serif.fnt",font_path)
            )?
        );
        let proto_mono = Rc::new(
            FontProto::new(
                graphics,
                &format!("{}/mono.fnt",font_path)
            )?
        );

        let font = Rc::new(Font::new(&proto_sans,16)?);

        Ok(Styles {
            proto_sans: proto_sans,
            proto_serif: proto_serif,
            proto_mono: proto_mono,
            font: font,
            accordeon_tab_text_color: 0xAAAAAA,
            accordeon_tab_color: 0x444444,
            accordeon_tab_hover_color: 0x224488,
            accordeon_tab_current_color: 0x112244,
            book_tab_text_color: 0xAAAAAA,
            book_tab_color: 0x444444,
            book_tab_hover_color: 0x224488,
            book_tab_current_color: 0x112244,
            book_tab_background_color: 0x111111,
            button_text_color: 0xAAAAAA,
            button_disabled_text_color: 0x666666,
            button_color: 0x444444,
            button_hover_color: 0x224488,
            button_disabled_color: 0x333333,
            button_pressed_color: 0x3366CC,
            // field
            // list
            menu_item_text_color: 0xAAAAAA,
            menu_item_disabled_text_color: 0x666666,
            menu_item_color: 0x444444,
            menu_item_hover_color: 0x224488,
            menu_item_disabled_color: 0x333333,
            menu_item_current_color: 0x3366CC,
            menubar_item_text_color: 0xAAAAAA,
            menubar_item_color: 0x444444,
            menubar_item_hover_color: 0x224488,
            menubar_item_current_color: 0x112244,
            // progress
            // scrollbar
            // slider
            // stepper
            text_color: 0xAAAAAA,
            // toggle
            toolbar_item_text_color: 0xAAAAAA,
            toolbar_item_color: 0x444444,
            toolbar_item_hover_color: 0x224488,
            // tree        
        })
    }
}
