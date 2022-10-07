
pub struct Todo {
id: u8,
title: String,
note: String,
complete: bool
}
impl Todo {
    pub fn add(id:u8, title: &str, note: &str) -> Todo {
        Todo {
            id: id,
            title: title.to_string(),
            note: String::from(note),
            complete: false
        }
    }
    pub fn view(&self) {
        println!("ID: {id} Title: {title} Complete: {complete}\n * {note} ",id=self.id , title=self.title, complete=self.complete, note=self.note)
    }
    pub fn complete(&mut self) {
        self.complete= !{self.complete}
    }

}
