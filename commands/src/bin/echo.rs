use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(required = true, num_args = 1..)]
    text: Vec<String>,

    #[arg(short, long, default_value_t = false, help = "Omit newline")]
    no_newline: bool,
}

fn main() {
    let args = Args::parse();
    let term = if args.no_newline { "" } else { "\n" };
    print!("{}{}", args.text.join(" "), term);
}
