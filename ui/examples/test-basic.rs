// E - Text test
// Desmond Germans, 2020

use base::*;
use platform::*;
use gpu::*;
use ui::*;
use std::rc::Rc;

const FONT_DIR: &str = "/home/desmond/e/static/fonts";
const ICON_DIR: &str = "/home/desmond/e/static/images/icons";
const IMAGE_PATH: &str = "/home/desmond/e/static/images/world.png";

fn main() -> Result<(),SystemError> {
    let system = Rc::new(System::new()?);
    let graphics = Rc::new(Graphics::new(&system)?);
    let ui = Rc::new(UI::new(&system,&graphics,FONT_DIR)?);

    let file_menu_items = vec![
        MenuItem::Action("New File".to_string()),
        MenuItem::Action("New Window".to_string()),
        MenuItem::Separator,
        MenuItem::Action("Open File...".to_string()),
        MenuItem::Action("Open Folder...".to_string()),
        MenuItem::Separator,
        MenuItem::Action("Exit".to_string()),
    ];
    let file_menu = Rc::new(Menu::new(&ui,file_menu_items)?);

    let edit_menu_items = vec![
        MenuItem::Action("Undo".to_string()),
        MenuItem::Action("Redo".to_string()),
        MenuItem::Separator,
        MenuItem::Action("Cut".to_string()),
        MenuItem::Action("Copy".to_string()),
        MenuItem::Action("Paste".to_string()),
    ];
    let edit_menu = Rc::new(Menu::new(&ui,edit_menu_items)?);

    let help_menu_items = vec![
        MenuItem::Action("Welcome".to_string()),
        MenuItem::Action("Interactive Playground".to_string()),
        MenuItem::Separator,
        MenuItem::Action("About".to_string()),
    ];
    let help_menu = Rc::new(Menu::new(&ui,help_menu_items)?);

    let menubar_items = vec![
        MenuBarItem::Menu("File".to_string(),file_menu),
        MenuBarItem::Menu("Edit".to_string(),edit_menu),
        MenuBarItem::Separator,
        MenuBarItem::Menu("Help".to_string(),help_menu),
    ];
    let menubar = Rc::new(MenuBar::new(&ui,menubar_items)?);

    let lightning_mat = imageformats::load::<pixel::ARGB8>(&format!("{}/lightning.png",ICON_DIR))?;
    let paper_airplane_mat = imageformats::load::<pixel::ARGB8>(&format!("{}/paper_airplane.png",ICON_DIR))?;
    let phone_mat = imageformats::load::<pixel::ARGB8>(&format!("{}/phone.png",ICON_DIR))?;
    let touch_phone_mat = imageformats::load::<pixel::ARGB8>(&format!("{}/phone_touch.png",ICON_DIR))?;
    let photo_mat = imageformats::load::<pixel::ARGB8>(&format!("{}/photo.png",ICON_DIR))?;
    let toolbar_items = vec![
        ToolBarItem::Action(graphics.create_texture2d_from_mat(lightning_mat)?),
        ToolBarItem::Action(graphics.create_texture2d_from_mat(paper_airplane_mat)?),
        ToolBarItem::Separator,
        ToolBarItem::Action(graphics.create_texture2d_from_mat(phone_mat)?),
        ToolBarItem::Action(graphics.create_texture2d_from_mat(touch_phone_mat)?),
        ToolBarItem::Separator,
        ToolBarItem::Action(graphics.create_texture2d_from_mat(photo_mat)?),
    ];
    let toolbar = Rc::new(ToolBar::new(&ui,toolbar_items)?);

    let text1 = Rc::new(Text::new(&ui,"This is just a line of text.")?);
    let text2 = Rc::new(Text::new(&ui,"And so is this.")?);
    let text3 = Rc::new(Text::new(&ui,"And this is too.")?);
    let text4 = Rc::new(Text::new(&ui,"It's not getting any better, really.")?);
    let text5 = Rc::new(Text::new(&ui,"Text for everyone.")?);

    let stack0_children: Vec<Rc<dyn Widget>> = vec![
        text1,
        text2,
        text3,
        text4,
        text5,
    ];
    let stack0 = Rc::new(Stack::new_vertical(&ui,stack0_children)?);

    let mat = imageformats::load::<pixel::ARGB8>(IMAGE_PATH)?;
    let image = Rc::new(Image::new(&ui,&graphics,mat)?);

    let button = Rc::new(Button::new(&ui,"Button")?);

    let field = Rc::new(Field::new(&ui)?);

    let progress1 = Rc::new(Progress::new_horizontal(&ui,100.0)?);
    progress1.set_value(40.0);

    let scrollbar1 = Rc::new(ScrollBar::new_horizontal(&ui,100.0,10.0,1.0)?);

    let slider1 = Rc::new(Slider::new_horizontal(&ui)?);

    let toggle = Rc::new(Toggle::new(&ui)?);

    let stack1_children: Vec<Rc<dyn Widget>> = vec![
        button,
        field,
        progress1,
        scrollbar1,
        slider1,
        toggle,
    ];
    let stack1 = Rc::new(Stack::new_vertical(&ui,stack1_children)?);

    let progress2 = Rc::new(Progress::new_vertical(&ui,100.0)?);
    progress2.set_value(40.0);

    let scrollbar2 = Rc::new(ScrollBar::new_vertical(&ui,100.0,10.0,1.0)?);

    let slider2 = Rc::new(Slider::new_vertical(&ui)?);

    let stack2_children: Vec<Rc<dyn Widget>> = vec![
        progress2,
        scrollbar2,
        slider2,
    ];
    let stack2 = Rc::new(Stack::new_horizontal(&ui,stack2_children)?);

    let splitter = Rc::new(Splitter::new_horizontal(&ui,stack1,stack2)?);

    let list = Rc::new(Text::new(&ui,"UNDER CONSTRUCTION")?);

    let tree = Rc::new(Text::new(&ui,"UNDER CONSTRUCTION TOO")?);

    let book_items = vec![
        BookPage { name: "Text".to_string(),child: stack0, },
        BookPage { name: "Image".to_string(),child: image, },
        BookPage { name: "Splitter".to_string(),child: splitter, },
        BookPage { name: "List".to_string(),child: list, },
        BookPage { name: "Tree".to_string(),child: tree, },
    ];
    let book = Rc::new(Book::new(&ui,book_items)?);

    let stack_children: Vec<Rc<dyn Widget>> = vec![
        menubar,
        toolbar,
        book,
    ];
    let stack = Rc::new(Stack::new_vertical(&ui,stack_children)?);

    let window = UIWindow::new_frame(&ui,rect!(50,50,640,350),"Text Test",&(stack as Rc<dyn Widget>))?;

    ui.run();

    drop(window);

    Ok(())
}
