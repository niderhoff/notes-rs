#![allow(unused)]

use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Error, ErrorKind},
};

use clap::{App, AppSettings, Arg, ArgMatches};

struct Note {
    id: usize,
    text: String,
}

impl Note {
    fn get_id(sub_m: &ArgMatches) -> usize {
        sub_m
            .value_of("id")
            .expect("please enter id.")
            .parse::<usize>()
            .unwrap()
    }
}

impl Clone for Note {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            text: self.text.clone(),
        }
    }
}

struct Database {
    path: String,
    notes: Option<Vec<Note>>,
}

impl Database {
    fn new(path: String) -> Database {
        Database {
            path,
            notes: Some(vec![]),
        }
    }
    fn convert_to_note(line: &str) -> Note {
        let mut split = line.split("|");
        let id = split.nth(0).unwrap().parse::<usize>().unwrap();
        let text = split.nth(1).unwrap().to_owned();
        Note { id, text }
    }
    fn replace(&mut self, note: Note) -> Result<(), Error> {
        if let Some(notes) = &mut self.notes {
            let index = 1usize;
            std::mem::replace(&mut notes[index], note);
        }
        Ok(())
    }
    fn read_backup(&mut self) -> Result<usize, Error> {
        let file = File::open(&self.path)?;
        let buf = std::fs::read_to_string(&self.path)?;
        let mut notes = vec![];
        for line in buf.split("\n") {
            let note = Database::convert_to_note(line);
            notes.push(note);
        }
        let len = notes.len();
        self.notes = Some(notes);
        Ok(len)
    }
    fn write_backup(&self) -> Result<(), Error> {
        if self.notes.is_some() {
            let notes = self.notes.as_ref().unwrap();
            let file = File::open(&self.path)?;
            let m = notes.iter().map(|x| &x.text);
        }
        Ok(())
    }
    /// add new note, returns the Note if finished
    fn add(&mut self, buf: &str) -> Result<Note, Error> {
        let id = self.read_backup()?;
        let n = Note {
            text: buf.to_owned(),
            id,
        };
        match &mut self.notes {
            Some(notes) => {
                notes.push(n.clone());
            }
            None => {
                let notes = vec![n.clone()];
            }
        };
        Ok(n)
    }
    fn get_index_by_id(&self, id: &usize) -> Option<usize> {
        self.notes.as_ref()?.iter().position(|x| x.id == *id)
    }
    /// find Note by id, return ref if found
    fn get_note_by_id(&self, id: &usize) -> Option<&Note> {
        self.notes.as_ref()?.iter().find(|x| x.id == *id)
    }
    fn delete(&mut self, id: &usize) -> Option<Note> {
        let idx = self.notes.as_ref()?.iter().position(|x| x.id == *id)?;
        let ns = &mut self.notes;
        match ns {
            Some(ns) => Some(ns.remove(idx)),
            None => None,
        }
    }
}

fn main() {
    let mut db = Database::new(String::from("notes.txt"));
    let mut app = App::new("notes")
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
            db.add(&buf);
            println!("added new note: {:?}", buf);
        }
        Some(("get", sub_m)) => match sub_m.value_of("id") {
            Some(id) => {
                let id = id.parse::<usize>().unwrap();
                db.get_note_by_id(&id).map(|x| println!("{:?}", x.text));
            }
            None => {
                if let Some(notes) = db.notes {
                    for note in notes {
                        println!("{}: {}", note.id, note.text);
                    }
                }
            }
        },
        Some(("delete", sub_m)) => {
            let got = db.delete(&Note::get_id(sub_m)).unwrap();
            println!("removed note {}: {}", got.id, got.text);
        }
        Some(("update", sub_m)) => {
            let id = Note::get_id(sub_m);
            let buf = sub_m
                .values_of("text")
                .unwrap()
                .collect::<Vec<&str>>()
                .join(" ");
            db.replace(Note {
                id,
                text: buf.clone(),
            });
            println!("updated note {}: {}", id, &buf);
        }
        _ => (),
    }
}
