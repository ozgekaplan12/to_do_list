#[derive(Clone)]

struct Task {
    id : usize,
    description : String,
    completed : bool,
}

struct ToDoList {
    tasks: Vec<Task>,
    next_id: usize, // Sonraki görevin ID'sini tutmak için bir sayaç
}

impl ToDoList {
    fn add_task(&mut self, description: &str) -> Task {
        
        let new_task = Task {
            id : self.next_id,
            description: description.to_string(),
            completed: false,
        };
        self.tasks.push(new_task.clone());
        self.next_id += 1;  // Sonraki görevin ID'sini arttır
        new_task
        
    }
    
    fn complete_task(&mut self, id: usize) -> Option<&Task> {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                return Some(task);
            }
        }
        None
    }

    fn list_tasks(&self){
        println!("To Do List: ");
        for task in &self.tasks{
            println!("Task ID: {}", task.id);
            println!("Description: {}", task.description);
            println!("Completed: {}", task.completed);
            println!()
        }
    }

    
}
fn main() {
    let mut todo_list = ToDoList {
    tasks: vec![],
    next_id: 0,
};

todo_list.add_task("Task 1");
todo_list.add_task("Task 2");
todo_list.list_tasks();

todo_list.complete_task(0);
todo_list.list_tasks();
}
