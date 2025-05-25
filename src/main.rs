use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::{BufReader, BufWriter}, path::Path, str::FromStr};

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
        let high_priority = parts[1]
            .trim()
            .parse::<bool>()
            .map_err(|_| "Invalid boolean".to_string())?;
        Ok(AddTaskArg {
            title,
            high_priority,
        })
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

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    title: String,
    high_priority: bool,
    done: bool,
}

#[derive(Serialize, Deserialize)]
struct Tasks {
    tasks: Vec<Task>,
}

impl Tasks {
    fn load_from_file (path : &str) -> Self {
        if Path::new(path).exists(){
            let file = File::open(path).expect("Failed to open file");
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_else(|_| Tasks { tasks: Vec::new() })
        } else {
            Tasks {
            tasks: Vec::new(),
        }
        }
    }

    fn save_to_file (&self, path : &str){
        let file = File::create(path).expect("Failed to create tasks file");
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self).expect("Failed to write tasks to file ");
    }

    fn next_id(&self) -> usize {
        self.tasks.iter().map(| t | t.id ). max().unwrap_or(0) + 1
    }
}

fn main() {
    let args = Args::parse();
    let file_name = "tasks.json";
    let mut tasks = Tasks::load_from_file(file_name);

    for task_arg in args.add {
        let id = tasks.next_id();
        tasks.tasks.push(Task {
            id,
            title: task_arg.title,
            high_priority: task_arg.high_priority,
            done: false,
        });
    }

    for task in &mut tasks.tasks {
        if args.done.contains(&(task.id as u32)) {
            task.done = true;
        }
    }

    if args.get {
        for task in &tasks.tasks {
            if !task.done {
                println!("[{}] {} (priority : {}, id: {})", 
            if task.done { "✔" } else { " " },
            task.title,
            if task.high_priority { "high" } else { "normal" },
            task.id  
        )
            }
        }
    }

    tasks.save_to_file(file_name);
}
