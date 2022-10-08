pub struct Todo {
id: u8,
objective: String,
complete: bool
}
impl Todo {
    pub fn add(id:u8, objective: &str) -> Todo {
        Todo {
            id,
            objective: objective.to_string(),
            complete: false
        }
    }
    pub fn view(&self) {
        let mark = if self.complete {"x"} else {" "};
        let out:String = format!("Id:[{id}] {objective} |{complete}| ",id=self.id , objective=self.objective, complete=mark);
        println!("{0}\n{1}","-".repeat(out.len()), out);
    }
    pub fn obj(&self) -> (u8, &String, bool) {
        (self.id, &self.objective, self.complete)
    }
    pub fn complete(&mut self) {
        self.complete= !{self.complete}
    }

}
