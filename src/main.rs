use clap::{AppSettings, Clap};
mod converter;
#[derive(Clap)]
#[clap(
    version = "1.0",
    author = "Shun Namiki aka Nash <snamiki1212@gmail.com>"
)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    input: String,
}
fn main() {
    let opts: Opts = Opts::parse();
    let md = converter::url2md(&opts.input);
    println!("{}", md);
}
