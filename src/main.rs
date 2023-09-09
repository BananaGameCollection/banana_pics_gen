use clap::Parser;
use banana_pics_gen::{auto_gen, mahjong};
use banana_pics_gen::data::{Args, Pattern};

fn main() {
    let arg = Args::parse();

    let mut seed: u64 = 0x00ff0514;
    if let Some(r_seed) = arg.rand_seed { seed = r_seed };


    println!("{:?}", seed);

    match arg.pattern {
        Pattern::AutoGen => {
            auto_gen()
        }
        Pattern::Mahjong(matrix) => {
            if let Some(mode) = &matrix.mode {

            }
            mahjong(&matrix)
        }
    }
}


