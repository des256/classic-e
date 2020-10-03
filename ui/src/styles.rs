// E - Styles
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;

/// Styles struct.
pub struct Styles {
    pub proto_sans: Rc<FontProto>,
    pub proto_serif: Rc<FontProto>,
    pub proto_mono: Rc<FontProto>,
    pub font: Rc<Font>,
    pub book_tab_text_color: u32,
    pub book_tab_color: u32,
    pub book_tab_hover_color: u32,
    pub book_tab_current_color: u32,
    pub book_tab_background_color: u32,
    pub button_text_color: u32,
    pub button_color: u32,
    pub button_hover_color: u32,
    pub menubar_text_color: u32,
    pub text_color: u32,
}

impl Styles {
    pub fn new_default(graphics: &Rc<Graphics>,font_path: &str) -> Result<Styles,SystemError> {
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
            book_tab_text_color: 0xAAAAAA,
            book_tab_color: 0x332211,
            book_tab_hover_color: 0x112244,
            book_tab_current_color: 0x665522,
            book_tab_background_color: 0x111111,
            button_text_color: 0xAAAAAA,
            button_color: 0x332211,
            button_hover_color: 0x112244,
            menubar_text_color: 0xAAAAAA,
            text_color: 0xAAAAAA,
        })
    }
}
