use clap::Parser;
#[derive(Parser)]
struct Args {
    #[clap(long, alias("my_name"))]
    my_name: Option<std::String>,
}
