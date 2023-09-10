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

pub struct GenCanceled();
impl Logger for GenCanceled{
    fn print_stats() {
        println!("generate canceled!")
    }
}

pub struct GenWrapping();

impl Logger for GenWrapping {
    fn print_stats() {
        println!("writing to file...")
    }
}