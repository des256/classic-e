// E - test
// Desmond Germans, 2020

use e::*;
use std::{
    io::prelude::*,
    fs::File,
    rc::Rc,
    cell::Cell,
};

const MAX_FRAMES_IN_FLIGHT: usize = 2;

struct SwapChainResources {
    swapchain: Rc<SwapChain>,
    command_buffers: Vec<Rc<CommandBuffer>>,
    image_useds: Vec<Option<usize>>,
    image_availables: Vec<Rc<Semaphore>>,
    render_finisheds: Vec<Rc<Semaphore>>,
    inflight_fences: Vec<Rc<Fence>>,
    current: usize,
}

impl SwapChainResources {

    fn new(
        screen: &Rc<Screen>,
        window: &Rc<Window>,
        session: &Rc<Session>,
        pipeline_layout: &Rc<PipelineLayout>,
        vertex_shader: &Rc<Shader>,
        fragment_shader: &Rc<Shader>
    ) -> SwapChainResources {

        // create render pass
        let render_pass = session.create_render_pass().expect("Unable to create render pass.");

        // create graphics pipeline
        let graphics_pipeline = session.create_graphics_pipeline(&window,&pipeline_layout,&render_pass,&vertex_shader,&fragment_shader).expect("Unable to create graphics pipeline.");

        // create swap chain for the window
        let swapchain = session.create_swapchain(&window).expect("Unable to create swap chain.");

        // get images from the swap chain
        let images = swapchain.get_images();

        // create framebuffer and command buffer for each image, as well as image used fences
        let mut framebuffers = Vec::<Rc<Framebuffer>>::new();
        let mut command_buffers = Vec::<Rc<CommandBuffer>>::new();
        let mut image_useds = Vec::<Option<usize>>::new();
        for image in &images {
            let image_view = image.get_view().expect("Unable to create image view.");
            let framebuffer = image_view.create_framebuffer(swapchain.extent,&render_pass).expect("Unable to create framebuffer.");
            let command_buffer = session.create_commandbuffer(screen.graphics_queue_id).expect("Unable to create command buffer.");
            if command_buffer.begin() {
                command_buffer.bind_pipeline(&graphics_pipeline);
                command_buffer.begin_render_pass(&render_pass,&framebuffer);
                command_buffer.draw(3,1,0,0);
                command_buffer.end_render_pass();
                if !command_buffer.end() {
                    println!("Unable to end command buffer.");
                }
            }
            else {
                println!("Unable to begin command buffer.");
            }
            framebuffers.push(framebuffer);
            command_buffers.push(command_buffer);
            image_useds.push(None);
        }
        
        // create further synchronisation objects
        let mut image_availables = Vec::<Rc<Semaphore>>::new();
        let mut render_finisheds = Vec::<Rc<Semaphore>>::new();
        let mut inflight_fences = Vec::<Rc<Fence>>::new();
        for _ in 0..MAX_FRAMES_IN_FLIGHT {
            image_availables.push(session.create_semaphore().expect("Unable to create image available semaphore."));
            render_finisheds.push(session.create_semaphore().expect("Unable to create render finished semaphore."));
            inflight_fences.push(session.create_fence().expect("Unable to create inflight fence."));
        }

        SwapChainResources {
            swapchain: swapchain,
            command_buffers: command_buffers,
            image_useds: image_useds,
            image_availables: image_availables,
            render_finisheds: render_finisheds,
            inflight_fences: inflight_fences,
            current: 0usize,
        }
    }

    pub fn draw(&mut self,application: &Application) -> bool {

        // wait for slot to become available
        self.inflight_fences[self.current].wait();

        // get index of next free swap chain image, using the current slot
        match self.swapchain.next(&self.image_availables[self.current]) {
            Next::Image(index) => {

                // verify that the image is not already being used by another slot
                if let Some(fence_index) = self.image_useds[index] {
                    self.inflight_fences[fence_index].wait();
                }

                // indicate that now we are using this image
                self.image_useds[index] = Some(self.current);

                // reset the fence for this slot
                self.inflight_fences[self.current].reset();

                // submit the command buffer
                if !application.graphics_queue.submit(
                    &self.command_buffers[index],
                    &self.image_availables[self.current],  // waits until image becomes available
                    &self.render_finisheds[self.current],  // will be signaled when rendering is complete
                    &self.inflight_fences[self.current]  // will be signaled when command buffer has completed
                ) {
                    println!("Unable to submit command buffer.");
                }

                // and present the image
                application.present_queue.present(&self.swapchain,index,&self.render_finisheds[self.current]);  // waits until rendering is complete
                application.present_queue.wait_idle();

                // next worker slot
                self.current = (self.current + 1) % MAX_FRAMES_IN_FLIGHT;

                true
            },
            Next::OutOfDate => {
                // wait for everything to settle
                application.session.wait_idle();

                // rebuild swapchain resources
                false
            },
            _ => {
                panic!("Unable to acquire next image in swap chain.");
            }
        }
    }
}

struct Application {
    system: Rc<System>,
    screen: Rc<Screen>,
    window: Rc<Window>,
    running: Rc<Cell<bool>>,
    session: Rc<Session>,
    graphics_queue: Rc<Queue>,
    present_queue: Rc<Queue>,
    vertex_shader: Rc<Shader>,
    fragment_shader: Rc<Shader>,
    pipeline_layout: Rc<PipelineLayout>,
}

impl Application {

    pub fn new() -> Application {
        let system = System::new().expect("Unable to access system.");

        let gpus = system.find_gpus();
        println!("GPUs found:");
        for gpu in &gpus {
            println!("    {}",gpu.name);
        }

        // find all screens that are connected to these GPUs
        let screens = system.find_screens(&gpus);
        println!("Screens found:");
        for screen in &screens {
            println!("    {} ({})",screen.name,screen.gpu.name);
        }

        // choose the first screen and its GPU
        let screen = Rc::clone(&screens[0]);
        let gpu = Rc::clone(&screen.gpu);

        // open frame on this screen
        println!("Opening window on {}.",screen.name);
        let window = screen.create_frame(rect!(50,50,640,480),"test window").expect("Unable to create window.");
        let running = Rc::new(Cell::new(true));
        let window_running = Rc::clone(&running);
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

        // create session on the GPU, supporting graphics and presentation queues
        let session = if screen.graphics_queue_id == screen.present_queue_id {
            gpu.create_session(vec![
                (screen.graphics_queue_id,1),
            ]).expect("Unable to create session.")
        }
        else {
            gpu.create_session(vec![
                (screen.graphics_queue_id,1),
                (screen.present_queue_id,1),
            ]).expect("Unable to create session.")
        };

        // get graphics and presentation queues
        let graphics_queue = session.get_queue(screen.graphics_queue_id,0).expect("Unable to obtain queue.");
        let present_queue = session.get_queue(screen.present_queue_id,0).expect("Unable to obtain queue.");

        // create vertex shader
        let mut f = File::open("test-triangle-vert.spv").expect("Unable to open vertex shader.");
        let mut b = Vec::<u8>::new();
        f.read_to_end(&mut b).expect("Unable to read vertex shader.");
        let vertex_shader = session.create_shader(&b).expect("Unable to create vertex shader.");

        // create fragment shader
        let mut f = File::open("test-triangle-frag.spv").expect("Unable to open fragment shader.");
        let mut b = Vec::<u8>::new();
        f.read_to_end(&mut b).expect("Unable to read fragment shader.");
        let fragment_shader = session.create_shader(&b).expect("Unable to create fragment shader.");

        // create pipeline layout
        let pipeline_layout = session.create_pipeline_layout().expect("Unable to create pipeline layout.");

        Application {
            system: system,
            screen: screen,
            window: window,
            running: running,
            session: session,
            graphics_queue: graphics_queue,
            present_queue: present_queue,
            vertex_shader: vertex_shader,
            fragment_shader: fragment_shader,
            pipeline_layout: pipeline_layout,
        }
    }
}

fn main() {

    let application = Application::new();

    let mut swapchain_resources = SwapChainResources::new(
        &application.screen,
        &application.window,
        &application.session,
        &application.pipeline_layout,
        &application.vertex_shader,
        &application.fragment_shader
    );

    while application.running.get() {

        application.system.flush();

        while (application.window.r.get().s.x == 0) || (application.window.r.get().s.y == 0) {
            println!("minimized...");
            application.system.flush();
            application.system.wait();
        }

        if !swapchain_resources.draw(&application) {
            swapchain_resources = SwapChainResources::new(
                &application.screen,
                &application.window,
                &application.session,
                &application.pipeline_layout,
                &application.vertex_shader,
                &application.fragment_shader
            );
        }
    }
}
