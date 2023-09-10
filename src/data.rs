use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {

    /// Choose generation pattern
    #[clap(subcommand)]
    pub pattern: Pattern,

    /// Give random-seed manually
    pub rand_seed: Option<u64>,

}


#[derive(Copy, Clone, Debug, Parser)]
pub enum Pattern {

    /// Randomly generate pic
    AutoGen,

    /// Generate Mahjong pic
    Mahjong(Matrix),

}

#[derive(Copy, Clone, Debug, Parser)]
pub struct Matrix {

    /// The width of Mahjong pic
    #[arg(long, default_value_t = 600)]
    pub width: u32,

    /// The height of Mahjong pic
    #[arg(long, default_value_t = 800)]
    pub height: u32,

    /// Number of pictures per row
    #[arg(short, long, default_value_t = 6)]
    pub row: u32,

    /// Number of pictures per column
    #[arg(short, long, default_value_t = 5)]
    pub column: u32,

    /// Select generate mode
    #[clap(subcommand)]
    pub mode: Option<Mode>,
}

#[derive(Copy, Clone, Debug, Parser)]
pub enum Mode {

    /// Generate by default pictures[All]
    Default,

    /// Generate by pictures of Lu
    Lu,

    /// Generate by pictures of Banana
    Banana,

    /// Generate by pictures of strange-shape
    Strange,
}


