use color_eyre::eyre::Result;

use structopt::StructOpt;

use digital_garden::write;


/// A CLI for growing and maintaining a digital garden.
#[derive(StructOpt, Debug)]
#[structopt(name = "garden")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command
}

#[derive(StructOpt, Debug)]
enum Command {

    /// Write something in your garden
    ///
    /// Opens your editor, waits for you to write something, and saves it to your garden.
    Write {
        /// Optionally set a title for what you'll writ about
        #[structopt(short, long)]
        title: Option<String>
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::from_args();
    dbg!(&opt);
    match opt.cmd {
        Command::Write { title } => write(title),
    };
    todo!();
}
