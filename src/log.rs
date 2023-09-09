pub trait Logger {
    fn print_stats();
}

pub struct GenStart();
impl Logger for GenStart {
    fn print_stats() {
        println!("generating...")
    }
}


pub struct GenComplete();
impl Logger for GenComplete {
    fn print_stats() {
        println!("generate completed!")
    }
}