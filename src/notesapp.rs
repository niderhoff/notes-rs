use crate::db::Datastore;
use crate::Note;
pub struct NotesApp {
    db: Datastore,
}

impl NotesApp {
    pub fn new() -> NotesApp {
        NotesApp::new_at(None)
    }
    pub fn new_at(db_name: Option<String>) -> NotesApp {
        let db_name = db_name.unwrap_or(String::from("notes.txt"));
        let db = crate::db::Datastore::new(db_name);
        NotesApp { db }
    }

    pub fn add(&mut self, buf: String) -> Result<(), Box<dyn std::error::Error>> {
        let r = self.db.add(&buf)?;
        println!("added new note: {:?}", buf);
        Ok(r)
    }

    pub fn get_note(&mut self, id: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.db
            .get_note_by_id(&id)
            .map(|x| println!("{:?}", x.text));
        Ok(())
    }

    pub fn get_all_notes(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(notes) = &self.db.notes {
            if notes.is_empty() {
                println!("nothing to show.");
            } else {
                for note in notes {
                    println!("{}: {}", note.id, note.text);
                }
            }
        }
        Ok(())
    }

    pub fn delete(&mut self, id: &usize) -> Result<(), Box<dyn std::error::Error>> {
        let note = self.db.delete(id)?;
        println!("removed note {}: {}", id, note.text);
        Ok(())
    }
    pub fn update(self, id: usize, buf: String) -> Result<Note, Box<dyn std::error::Error>> {
        let oldnote = self.db.replace(
            id,
            Note {
                id,
                text: buf.clone(),
            },
        )?;
        println!(
            "updated note {} from '{}' to '{}",
            oldnote.id, oldnote.text, buf
        );
        Ok(oldnote)
    }
}

#[cfg(test)]
mod tests {
    use crate::notesapp::NotesApp;

    #[test]
    fn it_works() {
        let mut n = NotesApp::new();
        let newtext = String::from("this my first note text");
        n.add(newtext).unwrap();
    }

    #[test]
    fn get() {
        let mut n = NotesApp::new();
        n.get_all_notes().unwrap();
    }
}
