// E - test
// Desmond Germans, 2020

use e::*;
use std::{
    io,
    io::prelude::*,
    fs::File,
};

fn main() {

    let system = System::new().expect("Unable to access system.");

    let gpu_names = system.enumerate_gpu_names();
    println!("GPUs found:");
    for i in 0..gpu_names.len() {
        println!("    {}: {}",i,gpu_names[i]);
    }

    println!("Using GPU 0.");
    let gpu = GPU::new(&system,0).expect("Unable to access GPU.");
    let mut graphics_queue: Option<usize> = None;
    println!("Available queue families:");
    for i in 0..gpu.queue_families.len() {
        let mut description = "".to_string();
        if gpu.queue_families[i].graphics {
            match graphics_queue {
                None => { graphics_queue = Some(i); },
                _ => { },
            }
            description += " graphics";
        }
        if gpu.queue_families[i].compute {
            description += " compute";
        }
        if gpu.queue_families[i].transfer {
            description += " transfer";
        }
        if gpu.queue_families[i].sparse {
            description += " sparse";
        }
        println!("    {}:{}",i,description);
    }
    if let None = graphics_queue {
        println!("Could not find graphics-capable queue family on this GPU.");
        return;
    }
    let graphics_queue = graphics_queue.unwrap();
    println!("Choosing {} as graphics-capable queue family.",graphics_queue);

    println!("Opening window.");
    let window = Window::new_frame(&system,rect!(50,50,640,480),"test window").expect("Unable to create window.");

    println!("Creating surface for this window.");
    let surface = Surface::new(&gpu,&window).expect("Unable to create surface.");
    let mut families = "".to_string();
    let mut queue_works = false;
    for i in 0..surface.queue_families.len() {
        families += &format!(" {}",surface.queue_families[i]);
        if i == graphics_queue {
            queue_works = true;
        }
    }
    println!("Available queue families for this surface:{}",families);
    if !queue_works {
        println!("Surface does not support the graphics-capable queue family.");
        return;
    }

    println!("Creating GPU session with one queue from family {}.",graphics_queue);
    let session = Session::new(&gpu,vec![(graphics_queue,1)]).expect("Unable to create session.");

    println!("Obtaining that queue from the session.");
    let queue = Queue::obtain(&session,0,0).expect("Unable to obtain queue.");

    println!("Loading vertex shader.");
    let mut f = File::open("test-triangle-vert.spv").expect("Unable to open vertex shader.");
    let mut b = Vec::<u8>::new();
    f.read_to_end(&mut b).expect("Unable to read vertex shader.");
    let vertex_shader = Shader::new(&session,&b).expect("Unable to create vertex shader.");

    println!("Loading fragment shader.");
    let mut f = File::open("test-triangle-frag.spv").expect("Unable to open fragment shader.");
    let mut b = Vec::<u8>::new();
    f.read_to_end(&mut b).expect("Unable to read fragment shader.");
    let fragment_shader = Shader::new(&session,&b).expect("Unable to create fragment shader.");

    println!("Creating graphics pipeline.");
    let graphics_pipeline = GraphicsPipeline::new(&session,&vertex_shader,&fragment_shader).expect("Unable to create graphics pipeline.");

    println!("Creating swap chain for the session, pipeline and surface.");
    let swapchain = SwapChain::new(&session,&graphics_pipeline,&surface,0).expect("Unable to create swap chain.");

    println!("Ok.");
}
