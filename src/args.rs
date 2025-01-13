use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "MolViz",
    author = "Adrien Pelfresne <adrien.pelfresne@gmail.com>",
    about = "A simple OpenGL molecular visualization, capable of reading protein data bank files.",
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
