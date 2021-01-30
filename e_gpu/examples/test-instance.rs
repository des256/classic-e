// E - test
// Desmond Germans, 2020

use e_base::*;
use e_system::*;
use e_gpu::*;

fn main() {

    let system = System::new().expect("Unable to access system.");
    
    let window = Window::new_frame(&system,rect!(50,50,640,480),"test window").expect("Unable to create window.");
    
    let gpu = GPU::new(&system).expect("Unable to access GPU.");

    let index = 0usize;

    let session = Session::new(&gpu,index,vec![(0,1)]).expect("Unable to create session.");

    let queue = Queue::new(&session,index,0).expect("Unable to access main queue.");

    let surface = Surface::new(&gpu,&window).expect("Unable to create surface.");

    println!("Ok.");
}
