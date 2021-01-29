// E - test
// Desmond Germans, 2020

use e_base::*;
use e_system::*;
use e_gpu::*;

fn main() {

    let system = System::new().expect("Unable to access system.");
    
    let gpu = GPU::new(&system).expect("Unable to access GPU.");

    let devices = gpu.enumerate_devices();
    if devices.len() < 1 {
        println!("Unable to get available GPU devices.");
        return;
    }

    let index = 0usize;

    let queue_families = gpu.enumerate_queue_families(index);
    if queue_families.len() < 1 {
        println!("Unable to get queue families.");
        return;
    }

    let session = Session::new(&gpu,0,vec![(0,1)]).expect("Unable to create session.");

    let queue = Queue::new(&session,0,0).expect("Unable to access main queue.");
    
    let window = Window::new_frame(&system,rect!(50,50,640,480),"test window").expect("Unable to create window.");

    let surface = Surface::new(&session,&window).expect("Unable to create surface.");

    println!("Ok.");
}
