use std::{
    fs::{File, OpenOptions},
    io::Write,
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
    fn create_new_note(&self, line: &str) -> Note {
        let id = self.notes.as_ref().map_or_else(|| 0, |x| x.len());
        let text = line.to_owned();
        Note { id, text }
    }
    fn replace(&mut self, note: Note) -> Result<(), std::io::Error> {
        if let Some(notes) = &mut self.notes {
            let index = 1usize;
            let _ = std::mem::replace(&mut notes[index], note);
        }
        // todo return old value
        Ok(())
    }
    fn read_backup(&mut self) -> Result<usize, std::io::Error> {
        let buf = std::fs::read_to_string(&self.path)?;
        let mut notes = vec![];
        for line in buf.split("\n") {
            let note = self.create_new_note(line);
            notes.push(note);
        }
        let len = notes.len();
        self.notes = Some(notes);
        Ok(len)
    }
    fn write_backup(&mut self) -> Result<(), std::io::Error> {
        if let Some(notes) = self.notes.as_ref() {
            let mut f: File = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open("notes.txt")?;
            for note in notes {
                writeln!(f, "{}|{}", note.id, note.text)?
            }
        }
        Ok(())
    }
    /// add new note, returns the Note if finished
    fn add(&mut self, buf: &str) -> Result<(), std::io::Error> {
        let id = self.read_backup()?;
        let n = Note {
            text: buf.to_owned(),
            id,
        };
        match &mut self.notes {
            // if we have notes. push new note
            Some(notes) => {
                notes.push(n);
            }
            // else create note vector with note in it.
            None => {
                self.notes = Some(vec![n]);
            }
        };
        self.write_backup()
    }
    fn get_index_by_id(&self, id: &usize) -> Option<usize> {
        self.notes.as_ref()?.iter().position(|x| x.id == *id)
    }
    /// find Note by id, return ref if found
    fn get_note_by_id(&self, id: &usize) -> Option<&Note> {
        self.notes.as_ref()?.iter().find(|x| x.id == *id)
    }
    fn delete(&mut self, id: &usize) -> Result<(), Box<dyn std::error::Error>> {
        let _r = {
            let idx = self.get_index_by_id(id).unwrap();
            let ns = &mut self.notes;
            match ns {
                Some(ns) => Some(ns.remove(idx)),
                None => None,
            }
        };
        self.write_backup()?;
        //Todo: return deleted note
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut db = Database::new(String::from("notes.txt"));
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
            let r = db.add(&buf);
            println!("added new note: {:?}", buf);
            Ok(r?)
        }
        Some(("get", sub_m)) => match sub_m.value_of("id") {
            Some(id) => {
                let id = id.parse::<usize>().unwrap();
                db.get_note_by_id(&id).map(|x| println!("{:?}", x.text));
                Ok(())
            }
            None => {
                if let Some(notes) = db.notes {
                    for note in notes {
                        println!("{}: {}", note.id, note.text);
                    }
                }
                Ok(())
            }
        },
        Some(("delete", sub_m)) => {
            let id = &Note::get_id(sub_m);
            let _got = db.delete(id).unwrap();
            println!("removed note {}", id);
            //todo: use returned note
            //println!("removed note {}: {}", got.id, got.text);
            Ok(())
        }
        Some(("update", sub_m)) => {
            let id = Note::get_id(sub_m);
            let buf = sub_m
                .values_of("text")
                .unwrap()
                .collect::<Vec<&str>>()
                .join(" ");
            let r = db.replace(Note {
                id,
                text: buf.clone(),
            });
            println!("updated note {}: {}", id, &buf);
            Ok(r?)
        }
        _ => Ok(()),
    }
}
