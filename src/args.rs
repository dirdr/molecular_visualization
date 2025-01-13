use core::fmt;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "molecular visualisation",
    author = "Adrien Pelfresne <adrien.pelfresne@gmail.com>",
    about = "A simple OpenGL molecular visualisation, capable of reading protein data bank files.",
    long_about = None,
    version,
    help_template = "\
    {before-help}{name} v{version} by {author}
    {about-with-newline}
    {usage-heading} {usage}
    {all-args}{after-help}"
)]
pub struct Args {
    #[arg(short, long)]
    pub file: String,
}
