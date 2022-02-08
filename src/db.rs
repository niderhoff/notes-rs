use crate::errors::DatastoreError;
use crate::note::Note;
use std::fs::{File, OpenOptions};
use std::io::Write;

pub struct Datastore {
    pub path: String,
    pub notes: Option<Vec<Note>>,
}

impl Datastore {
    pub fn new(path: String) -> Result<Datastore, std::io::Error> {
        let mut store = Datastore {
            path,
            notes: Some(vec![]),
        };
        // TODO: error handling?
        store.read_backup()?;
        Ok(store)
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
    pub fn replace(mut self, id: usize, newnote: Note) -> Result<Note, Box<dyn std::error::Error>> {
        // grab all notes
        let nvec = self.notes.as_mut().ok_or(DatastoreError::NotUpdateable)?;
        // find index
        let idx = &mut nvec
            .iter()
            .position(|elem| elem.id == id)
            .ok_or(DatastoreError::UnknownId { given: id })?;
        let old = std::mem::replace(&mut nvec[*idx], newnote);
        self.write_backup()?;
        Ok(old)
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
    pub fn delete(&mut self, id: &usize) -> Result<Note, Box<dyn std::error::Error>> {
        let idx = self
            .get_index_by_id(id)
            .ok_or(DatastoreError::UnknownId { given: *id })?;
        let ns = self.notes.as_mut();
        let result = match ns {
            Some(ns) => Ok(ns.remove(idx)),
            None => Err(DatastoreError::Empty),
        };
        self.write_backup()?;
        Ok(result?)
    }
}
