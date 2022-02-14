pub mod db;
mod errors;
pub mod note;
pub mod notesapp;

use crate::note::Note;
use crate::notesapp::NotesApp;
use clap::{App, AppSettings, Arg};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut n = NotesApp::new()?;
    let app = App::new("notes")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            App::new("add")
                .about("add a new note.")
                .arg(Arg::new("text").multiple_values(true)),
        )
        .subcommand(
            App::new("delete")
                .about("delete existing note.")
                .arg(Arg::new("id")),
        )
        .subcommand(
            App::new("update")
                .about("replace note text.")
                .arg(Arg::new("id"))
                .arg(Arg::new("text").multiple_values(true)),
        )
        .subcommand(App::new("get").about("read note.").arg(Arg::new("id")));
    let m = app.get_matches();
    match m.subcommand() {
        Some(("add", sub_m)) => {
            let buf = sub_m
                .values_of("text")
                .unwrap()
                .collect::<Vec<&str>>()
                .join(" ");
            n.add(buf)
        }
        Some(("get", sub_m)) => match sub_m.value_of("id") {
            Some(id) => {
                let id = id.parse::<usize>().unwrap();
                n.get_note(id)
            }
            None => n.get_all_notes(),
        },
        Some(("delete", sub_m)) => {
            let id = Note::get_id(sub_m)?;
            n.delete(&id)
        }
        Some(("update", sub_m)) => {
            let id = Note::get_id(sub_m)?;
            let buf = sub_m
                .values_of("text")
                .unwrap()
                .collect::<Vec<&str>>()
                .join(" ");
            n.update(id, buf)?;
            Ok(())
        }
        _ => Ok(()),
    }
}
