use color_eyre::{eyre::WrapErr, owo_colors::OwoColorize, Result};
use edit::{edit_file, Builder};
use std::fs;
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::PathBuf;

const TEMPLATE: &[u8; 2] = b"# ";

pub fn write(garden_path: PathBuf, title: Option<String>) -> Result<()> {
    let (mut file, filepath) = Builder::new()
        .suffix(".md")
        .rand_bytes(5)
        .tempfile_in(&garden_path)
        .wrap_err("Failed to create wip file")?
        .keep()
        .wrap_err("Failed to keep tempfile")?;
    file.write_all(TEMPLATE)?;

    // let the user write whatever they want in their favorite editor
    // before returning to the cli and finishing up
    edit_file(&filepath)?;

    // Read the user's changes back from the file into a string
    let mut contents = String::new();
    file.seek(SeekFrom::Start(0))?;
    file.read_to_string(&mut contents)?;

    let document_title = title.or_else(
        || {
            contents
                .lines()
                .find(|line| line.starts_with("# "))
                .map(
                    |line| line.trim_start_matches("# ").to_string()
                )
        }
    );

    let filename = match document_title {
        Some(raw_title) => confirm_filename(&raw_title),
        None => ask_for_filename(),
    }?;

    let mut i: usize = 0;
    loop {
        let dest_filename = format!(
            "{}{}",
            filename,
            if i == 0 { "".to_string() } else { i.to_string() }
        );

        let mut dest_path = garden_path.join(dest_filename);
        dest_path.set_extension("md");

        if dest_path.exists() {
            i += 1;
        } else {
            fs::rename(&filepath, &dest_path);
            break;
        }
    }
    Ok(())
}

fn confirm_filename(raw_title: &str) -> Result<String> {
    loop {
        let result = rprompt::prompt_reply_stderr(&format!(
            "\
{}{}
Is this OK? (Y/n): ",
"Current title: ".green().bold(),
            raw_title,
        ))
        .wrap_err("Failed to get input for y/n question")?;

        match result.as_str() {
            "n" | "N" => break ask_for_filename(),
            "y" | "Y" | "" => {
                break Ok(slug::slugify(raw_title));
            }
            _ => {
                // ask again because something went wrong
            }
        };
    }
}


fn ask_for_filename() -> Result<String> {
    rprompt::prompt_reply_stderr(&format!("{}",
    "\
Enter a filename:
> ".blue().bold()))
        .wrap_err("Failed to get filename")
        .map(slug::slugify)
}
