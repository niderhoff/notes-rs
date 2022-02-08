use crate::errors::DatastoreError;
use crate::note::Note;
use std::fs::{File, OpenOptions};
use std::io::Write;

pub struct Database {
    pub path: String,
    pub notes: Option<Vec<Note>>,
}

impl Database {
    pub fn new(path: String) -> Database {
        let mut db = Database {
            path,
            notes: Some(vec![]),
        };
        // TODO: error handling?
        db.read_backup().unwrap();
        db
    }
    pub fn line_as_note(&self, line: &str) -> Option<Note> {
        let split = line.split("|").collect::<Vec<&str>>();
        if split.len() == 2 {
            let id = split[0].trim().parse().ok()?;
            let text = String::from(split[1]);
            Some(Note { id, text })
        } else {
            None
        }
    }
    pub fn replace(&mut self, note: Note) -> Result<(), std::io::Error> {
        if let Some(notes) = &mut self.notes {
            let index = 1usize;
            let _ = std::mem::replace(&mut notes[index], note);
        }
        // todo return old value
        Ok(())
    }
    pub fn read_backup(&mut self) -> Result<usize, std::io::Error> {
        let notes = std::fs::read_to_string(&self.path).map_or(vec![], |buf| {
            let mut newvec = vec![];
            for line in buf.lines().filter(|l| *l != "") {
                self.line_as_note(line).map(|note| newvec.push(note));
            }
            newvec
        });
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
                .truncate(true)
                .open("notes.txt")?;
            for note in notes {
                writeln!(f, "{}|{}", note.id, note.text)?
            }
        }
        Ok(())
    }
    /// add new note, returns the Note if finished
    pub fn add(&mut self, buf: &str) -> Result<(), std::io::Error> {
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
    pub fn get_index_by_id(&self, id: &usize) -> Option<usize> {
        self.notes.as_ref()?.iter().position(|x| x.id == *id)
    }
    /// find Note by id, return ref if found
    pub fn get_note_by_id(&self, id: &usize) -> Option<&Note> {
        self.notes.as_ref()?.iter().find(|x| x.id == *id)
    }
    pub fn delete(&mut self, id: &usize) -> Result<(), Box<dyn std::error::Error>> {
        let _r = {
            let idx = self
                .get_index_by_id(id)
                .ok_or(DatastoreError::InvalidId { given: *id })?;
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
