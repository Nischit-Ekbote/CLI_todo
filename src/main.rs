use clap::Parser;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct AddTaskArg {
    title: String,
    high_priority: bool,
}

impl FromStr for AddTaskArg {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err("Expected format: title,bool".to_string());
        }
        let title = parts[0].trim().to_string();
        let high_priority = parts[1].trim().parse::<bool>().map_err(|_| "Invalid boolean".to_string())?;
        Ok(AddTaskArg { title, high_priority })
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, num_args = 1..)]
    done: Vec<u32>,
    #[arg(short, long)]
    add: Vec<AddTaskArg>,
    #[arg(long, default_value_t = false)]
    get: bool,
}

#[derive(Debug)]
struct Task {
    id: usize,
    title: String,
    high_priority: bool,
    done: bool,
}

fn main() {
    let args = Args::parse();
    let mut tasks: Vec<Task> = Vec::new();
    let mut id_counter = 0; 
    
    for task_arg in args.add {
        tasks.push(Task {
            id: id_counter,
            title: task_arg.title,
            high_priority: task_arg.high_priority,
            done: false,
        });
        id_counter += 1;
    }
    
    for task in &mut tasks {
        if args.done.contains(&(task.id as u32)) {
            task.done = true;
        }
    }

    if args.get {
        println!("{:#?}", tasks);
    }
}