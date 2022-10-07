pub struct Todo {
id: u8,
title: String,
note: String,
complete: bool
}
impl Todo {
    pub fn add(id:u8, title: &str, note: &str) -> Todo {
        Todo {
            id,
            title: title.to_string(),
            note: String::from(note),
            complete: false
        }
    }
    pub fn view(&self) {
        let out = format!("ID: {id} Title: {title} Complete: {complete}\n * {note} ",id=self.id , title=self.title, complete=self.complete, note=self.note);
        println!("{0}\n{1}","-".repeat(out.len()), out);
    }
    pub fn obj(&self) -> (u8, &String) {
        (self.id, &self.title)
    }
    pub fn complete(&mut self) {
        self.complete= !{self.complete}
    }

}
