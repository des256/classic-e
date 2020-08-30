// E - Image test
// Desmond Germans, 2020

//use e::*;
//use std::rc::Rc;
//use std::fs::File;
//use std::io::prelude::*;

fn main() {

    /*// initialize system
    let system = Rc::new(System::new().expect("Cannot open system."));

    // initialize graphics context
    let graphics = Rc::new(gpu::Graphics::new(&system).expect("Cannot open GPU."));

    // initialize UI
    let ui = Rc::new(ui::UI::new(&system,&graphics,"static/fonts").expect("Cannot open UI."));

    // load image into texture
    let mut file = File::open("static/images/world.png").expect("cannot open file");
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).expect("unable to read file");
    let mat = image::decode::<pixel::ARGB8>(&buffer).expect("unable to decode");
    let texture = Rc::new(gpu::Texture2D::new_from_mat(&graphics,mat).expect("unable to create texture"));

    // create image widget
    let widget = Rc::new(ui::Image::new(&ui,&texture).expect("Cannot create image."));

    // open window to host the text widget
    ui.open(&(widget as Rc<dyn ui::Widget>),rect!(50,50,640,360),"Test Window");

    // run UI loop
    ui.run();*/
}
