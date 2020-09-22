#[derive(Debug)]
pub struct Todos {
    id : u32,
    task : String,
    completed : bool
}

impl Todos {
    pub fn new(id: u32, task: String)->Todos {
        Todos {
            id,
            task,
            completed : false,
        }
    }
}