use std::path::PathBuf;
use directories::UserDirs;

use color_eyre::eyre::{eyre, Result, WrapErr};

use structopt::StructOpt;

use digital_garden::write;


/// A CLI for growing and maintaining a digital garden.
#[derive(StructOpt, Debug)]
#[structopt(name = "garden")]
struct Opt {
    #[structopt(
            parse(from_os_str),
            short = "p",
            long,
            env
        )]
        garden_path: Option<PathBuf>,

    #[structopt(subcommand)]
    cmd: Command,
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

// Joins the OS home directory with garden dir
fn get_default_garden_dir () -> Result<PathBuf> {
    let user_dirs = UserDirs::new().ok_or_else(|| {
        eyre!("Could not find home directory")
    })?;
    Ok(user_dirs.home_dir().join(".garden"))
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::from_args();
    dbg!(&opt);
    let garden_path = match opt.garden_path {
        Some(path) => Ok(path),
        None => get_default_garden_dir()
            .wrap_err("`garden_path` was not specified")
    };
    match opt.cmd {
        Command::Write { title } => write(garden_path, title),
    };
    todo!();
}
