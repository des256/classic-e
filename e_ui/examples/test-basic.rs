// E - Text test
// Desmond Germans, 2020

use e_base::*;
use e_platform::*;
use e_gpu::*;
use e_ui::*;
use std::{
    rc::Rc,
    cell::Cell,
};

const FONT_DIR: &str = "/home/desmond/e/static/fonts";
const ICON_DIR: &str = "/home/desmond/e/static/images/icons";
const IMAGE_PATH: &str = "/home/desmond/e/static/images/world.png";

fn main() -> Result<(),SystemError> {
    let system = System::new()?;
    let graphics = Graphics::new(&system)?;
    let ui = UI::new(&system,&graphics,FONT_DIR)?;

    let file_new_menu_items = vec![
        MenuItem::Action("Actually New File".to_string()),
        MenuItem::Action("Recycled File".to_string()),
        MenuItem::Action("Sort-of Kind-of New File".to_string()),
        MenuItem::Separator,
        MenuItem::Action("Old File".to_string()),
    ];
    let file_new_menu = Menu::new(&ui,file_new_menu_items)?;

    let file_menu_items = vec![
        MenuItem::Menu("New File".to_string(),file_new_menu),
        MenuItem::Action("New Window".to_string()),
        MenuItem::Separator,
        MenuItem::Action("Open File...".to_string()),
        MenuItem::Action("Open Folder...".to_string()),
        MenuItem::Separator,
        MenuItem::Action("Exit".to_string()),
    ];
    let file_menu = Menu::new(&ui,file_menu_items)?;

    let edit_menu_items = vec![
        MenuItem::Action("Undo".to_string()),
        MenuItem::Action("Redo".to_string()),
        MenuItem::Separator,
        MenuItem::Action("Cut".to_string()),
        MenuItem::Action("Copy".to_string()),
        MenuItem::Action("Paste".to_string()),
    ];
    let edit_menu = Menu::new(&ui,edit_menu_items)?;

    let help_menu_items = vec![
        MenuItem::Action("Welcome".to_string()),
        MenuItem::Action("Interactive Playground".to_string()),
        MenuItem::Separator,
        MenuItem::Action("About".to_string()),
    ];
    let help_menu = Menu::new(&ui,help_menu_items)?;

    let menubar_items = vec![
        MenuBarItem::Menu("File".to_string(),file_menu),
        MenuBarItem::Menu("Edit".to_string(),edit_menu),
        MenuBarItem::Separator,
        MenuBarItem::Menu("Help".to_string(),help_menu),
    ];
    let menubar = MenuBar::new(&ui,menubar_items)?;

    let lightning_mat = imageformats::load::<pixel::ARGB8>(&format!("{}/lightning.png",ICON_DIR))?;
    let paper_airplane_mat = imageformats::load::<pixel::ARGB8>(&format!("{}/paper_airplane.png",ICON_DIR))?;
    let phone_mat = imageformats::load::<pixel::ARGB8>(&format!("{}/phone.png",ICON_DIR))?;
    let touch_phone_mat = imageformats::load::<pixel::ARGB8>(&format!("{}/phone_touch.png",ICON_DIR))?;
    let photo_mat = imageformats::load::<pixel::ARGB8>(&format!("{}/photo.png",ICON_DIR))?;
    let toolbar_items = vec![
        ToolBarItem::Action(Texture2D::new_from_mat(&graphics,lightning_mat)?),
        ToolBarItem::Action(Texture2D::new_from_mat(&graphics,paper_airplane_mat)?),
        ToolBarItem::Separator,
        ToolBarItem::Action(Texture2D::new_from_mat(&graphics,phone_mat)?),
        ToolBarItem::Action(Texture2D::new_from_mat(&graphics,touch_phone_mat)?),
        ToolBarItem::Separator,
        ToolBarItem::Action(Texture2D::new_from_mat(&graphics,photo_mat)?),
    ];
    let toolbar = ToolBar::new(&ui,toolbar_items)?;

    let text1 = Text::new(&ui,"This is just a line of text.")?;
    let text2 = Text::new(&ui,"And so is this.")?;
    let text3 = Text::new(&ui,"And this is too.")?;
    let text4 = Text::new(&ui,"It's not getting any better, really.")?;
    let text5 = Text::new(&ui,"Text for everyone.")?;

    let stack0_children: Vec<Rc<dyn Widget>> = vec![
        text1,
        text2,
        text3,
        text4,
        text5,
    ];
    let stack0 = Stack::new_vertical(&ui,stack0_children)?;

    let mat = imageformats::load::<pixel::ARGB8>(IMAGE_PATH)?;
    let image = Image::new(&ui,&graphics,mat)?;

    let button = Button::new(&ui,"Button")?;

    let field = Field::new(&ui)?;

    let progress1 = Progress::new_horizontal(&ui,100.0)?;
    progress1.set_value(40.0);

    let scrollbar1 = ScrollBar::new_horizontal(&ui,100.0,10.0,1.0)?;

    let slider1 = Slider::new_horizontal(&ui)?;

    let toggle = Toggle::new(&ui,|_| { })?;

    let stack1_children: Vec<Rc<dyn Widget>> = vec![
        button,
        field,
        progress1,
        scrollbar1,
        slider1,
        toggle,
    ];
    let stack1 = Stack::new_vertical(&ui,stack1_children)?;

    let progress2 = Progress::new_vertical(&ui,100.0)?;
    progress2.set_value(40.0);

    let scrollbar2 = ScrollBar::new_vertical(&ui,100.0,10.0,1.0)?;

    let slider2 = Slider::new_vertical(&ui)?;

    let stack2_children: Vec<Rc<dyn Widget>> = vec![
        progress2,
        scrollbar2,
        slider2,
    ];
    let stack2 = Stack::new_horizontal(&ui,stack2_children)?;

    let splitter = Splitter::new_horizontal(&ui,stack1,stack2)?;

    let list = Text::new(&ui,"UNDER CONSTRUCTION")?;

    let tree = Text::new(&ui,"UNDER CONSTRUCTION TOO")?;

    let book_items = vec![
        BookPage { name: "Text".to_string(),child: stack0,enabled: Cell::new(true), },
        BookPage { name: "Image".to_string(),child: image,enabled: Cell::new(true), },
        BookPage { name: "Splitter".to_string(),child: splitter,enabled: Cell::new(true), },
        BookPage { name: "List".to_string(),child: list,enabled: Cell::new(true), },
        BookPage { name: "Tree".to_string(),child: tree,enabled: Cell::new(true), },
    ];
    let book = Book::new(&ui,book_items)?;

    let stack_children: Vec<Rc<dyn Widget>> = vec![
        menubar,
        toolbar,
        book,
    ];
    let stack = Stack::new_vertical(&ui,stack_children)?;

    let window = UIWindow::new_frame(&ui,rect!(50,50,640,350),"Text Test",stack as Rc<dyn Widget>)?;
    window.show();
    ui.run();
    window.hide();
    drop(window);
    Ok(())
}
