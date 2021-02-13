// E - test
// Desmond Germans, 2020

use e::*;
use std::{
    io::prelude::*,
    fs::File,
    rc::Rc,
    cell::Cell,
};

fn main() {

    let system = System::new().expect("Unable to access system.");

    let gpus = system.find_gpus();
    println!("GPUs found:");
    for gpu in &gpus {
        println!("    {}",gpu.name);
    }

    let screens = system.find_screens(&gpus);
    println!("Screens found:");
    for screen in &screens {
        println!("    {} ({})",screen.name,screen.gpu.name);
    }

    let screen = &screens[0];
    let gpu = &screen.gpu;

    println!("Opening window on {}.",screen.name);
    let window = screen.create_frame(rect!(50,50,640,480),"test window").expect("Unable to create window.");

    println!("Creating running variable.");
    let running = Rc::new(Cell::new(true));
    let window_running = Rc::clone(&running);

    println!("Setting window handler.");
    window.set_handler(move |event| {
        match event {
            Event::Close => {
                window_running.set(false);
            },
            _ => {
                println!("{}",event);
            },
        }
    });

    println!("Creating GPU session.");
    let session = if screen.graphics_queue_id == screen.present_queue_id {
        gpu.create_session(vec![(screen.graphics_queue_id,1)]).expect("Unable to create session.")
    }
    else {
        gpu.create_session(vec![(screen.graphics_queue_id,1),(screen.present_queue_id,1)]).expect("Unable to create session.")
    };

    println!("Obtaining the created queues.");
    let graphics_queue = session.get_queue(screen.graphics_queue_id,0).expect("Unable to obtain queue.");
    let present_queue = session.get_queue(screen.present_queue_id,0).expect("Unable to obtain queue.");

    println!("Loading vertex shader.");
    let mut f = File::open("test-triangle-vert.spv").expect("Unable to open vertex shader.");
    let mut b = Vec::<u8>::new();
    f.read_to_end(&mut b).expect("Unable to read vertex shader.");
    let vertex_shader = session.create_shader(&b).expect("Unable to create vertex shader.");

    println!("Loading fragment shader.");
    let mut f = File::open("test-triangle-frag.spv").expect("Unable to open fragment shader.");
    let mut b = Vec::<u8>::new();
    f.read_to_end(&mut b).expect("Unable to read fragment shader.");
    let fragment_shader = session.create_shader(&b).expect("Unable to create fragment shader.");

    println!("Creating graphics pipeline.");
    let graphics_pipeline = session.create_graphics_pipeline(&vertex_shader,&fragment_shader).expect("Unable to create graphics pipeline.");

    println!("Creating swap chain.");
    let swapchain = session.create_swapchain(&window).expect("Unable to create swap chain.");

    println!("Obtaining swap chain images.");
    let images = swapchain.get_images();

    println!("Creating swap chain image views and framebuffers.");
    let mut image_views = Vec::<Rc<ImageView>>::new();
    let mut framebuffers = Vec::<Rc<Framebuffer>>::new();
    for image in &images {
        let image_view = image.get_view().expect("Unable to create image view.");
        let framebuffer = image_view.create_framebuffer(swapchain.extent,&graphics_pipeline).expect("Unable to create framebuffer.");
        image_views.push(image_view);
        framebuffers.push(framebuffer);
    }

    println!("Building command buffers.");
    let mut command_buffers = Vec::<Rc<CommandBuffer>>::new();
    for framebuffer in &framebuffers {
        let command_buffer = session.create_commandbuffer(screen.graphics_queue_id).expect("Unable to create command buffer.");
        if command_buffer.begin() {
            command_buffer.bind_pipeline(&graphics_pipeline);
            command_buffer.begin_render_pass(&graphics_pipeline,framebuffer);
            command_buffer.draw(3,1,0,0);
            command_buffer.end_render_pass();
            if !command_buffer.end() {
                println!("Unable to end command buffer.");
            }
        }
        else {
            println!("Unable to begin command buffer.");
        }
        command_buffers.push(command_buffer);
    }

    println!("Creating semaphores.");
    let image_available = Semaphore::new(&session).expect("Unable to create image available semaphore.");
    let render_finished = Semaphore::new(&session).expect("Unable to create render finished semaphore.");

    println!("Running...");
    while running.get() {
        let index = swapchain.next(&image_available);
        if !graphics_queue.submit(&command_buffers[index],&image_available,&render_finished) {
            println!("Unable to submit command buffer.");
        }
        present_queue.present(&swapchain,index,&render_finished);    
        system.wait();
        system.flush();
    }

    println!("Ok.");
}
