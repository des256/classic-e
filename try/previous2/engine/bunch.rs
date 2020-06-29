// e::engine::Bunch
// by Desmond Germans, 2019

#[derive(Clone)]
pub struct Timed<T> {
    t: u64,
    event: T,
}

impl<T> Timed<T> {
   pub fn new(t: u64,event: T) -> Timed<T> {
       Timed {
           t: t,
           event: event,
       }
   } 
}

#[derive(Clone)]
pub struct Bunch<T> {
    events: Vec<Timed<T>>,
}

impl<T> Bunch<T> {
    pub fn new() -> Bunch<T> {
        Bunch {
            events: Vec::new(),
        }
    }
}
