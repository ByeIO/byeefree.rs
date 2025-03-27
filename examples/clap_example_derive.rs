// 命令行解析
use clap::Parser;

// 需要启用`derive`特性
#[derive(Parser)] 
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short = 'f')]
    eff: bool,

    #[arg(short = 'p', value_name = "PEAR")]
    pea: Option<String>,

    #[arg(last = true)]
    slop: Vec<String>,
}

fn main() {
    let args = Cli::parse();

    // This is what will happen with `myprog -f -p=bob -- sloppy slop slop`...
    
    // -f used: true
    println!("-f used: {:?}", args.eff); 
    // -p's value: Some("bob")
    println!("-p's value: {:?}", args.pea); 
    // 'slops' values: Some(["sloppy", "slop", "slop"])
    println!("'slops' values: {:?}", args.slop); 
}
