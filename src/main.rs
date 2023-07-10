use std::env::args;
use std::fs::{OpenOptions, File};
use std::io::{Write, BufReader, BufRead};

const FILE_PATH: &str = "src/database.txt";

#[derive(Debug)]
struct Todo {
    id: usize,
    title: String, //The title of the todo
    description: String, //The description of the todo
    completed: bool, //The completion status of the todo
    // timestamp: String
}

fn main() {
    let first_arg = args().nth(1);
    let second_arg = args().nth(2);
    let third_arg = args().nth(3);

    match first_arg.as_ref().map(String::as_ref) {
        Some("create") => if let Some(title) = second_arg{
            if let Some(description) = third_arg {
                create_todo(title, description);
            }
        },
        Some("list") => list_todos(),
        Some("delete") => if let Some(id) = second_arg{
            delete_todo(id)
        },
        Some("get") => if let Some(id) = second_arg{
            get_todo(id)
        },
        Some("complete") => if let Some(id) = second_arg{
            complete_todo(id)
        },
        Some("wipe") => {
            wipe_database()
        }
        _ => println!("Please pass a valid todo command"),
    }
}

fn create_todo(title: String, description: String) {
    let database = get_db_info().0;
    let id = get_db_info().1 + 1;

    let todo = Todo{
        id,
        title,
        description,
        completed: false 
    };

    let todo_as_string= format!("{} '{}' '{}' {}\n",todo.id, todo.title, todo.description, todo.completed);
    save_to_db(database, todo_as_string);
}

fn list_todos(){
    let database = BufReader::new(File::open(FILE_PATH).unwrap());
    for line in database.lines(){
        println!("{}", line.unwrap());
    }
}

fn delete_todo(id: String) {
    let database = BufReader::new(File::open(FILE_PATH).unwrap());
    let mut not_removed = true;

    let mut overwrite = String::new();

    for line in database.lines(){
        let line_val = line.unwrap();
        let line_val: Vec<String> = line_val.split_whitespace().map(String::from).collect();
        if line_val[0] != id {
            overwrite = overwrite+ &line_val.join(" ") + "\n";
        } else {
            not_removed = false;
        }
    }

    if not_removed {
        println!("Todo with the given ID not found");
        return;
    }
    
    println!("Deleting...");

    let database = OpenOptions::new()
    .write(true)
    .truncate(true)
    .open(FILE_PATH)
    .expect("cannot open file");

    save_to_db(database, overwrite)

}

fn get_todo(id: String) {
    let database = BufReader::new(File::open(FILE_PATH).unwrap());
    let id = match id.parse::<usize>() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid ID");
            0
        }
    };

    if id == 0 {return;}

    let mut not_found = true;
    for line in database.lines(){
        let line_val: Vec<String> = line.unwrap().split_whitespace().map(|x| x.to_string()).collect();
        let todo_id: usize = line_val[0].parse().unwrap();

        if todo_id == id {
            println!("{:?}", line_val.join(" "));
            not_found = false;
            break
        }
    }

    if not_found {
        println!("Entry with the given ID not found")
    }

}

fn complete_todo(id: String) {
    let database = BufReader::new(File::open(FILE_PATH).unwrap());
    let id = match id.parse::<usize>() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid ID");
            0
        }
    };

    if id == 0 {return;}

    let mut not_found = true;
    let mut overwrite = String::new();
    for line in database.lines(){
        let mut line_val: Vec<String> = line.unwrap().split_whitespace().map(String::from).collect();
        let len = line_val.len();
        
        if line_val[0].parse::<usize>().unwrap() == id {
            line_val[len - 1] = String::from("true");
            not_found = false;
        }
        overwrite = overwrite+ &line_val.join(" ") + "\n";
    }

    if not_found {
        println!("Entry with the given ID not found")
    } else {
        println!("Completing...");
        let database = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(FILE_PATH)
        .expect("cannot open file");

        save_to_db(database, overwrite)
    }

}

fn wipe_database() {
    println!("Wiping database");
    File::create(FILE_PATH).unwrap();
    println!("Database wiped!")
}
fn get_db_info() -> (std::fs::File, usize) {
    // Open a file with append option
    let database = OpenOptions::new()
        .append(true)
        .open(FILE_PATH)
        .expect("cannot open file");
    
    let f = BufReader::new(File::open(FILE_PATH).unwrap());
    let g = BufReader::new(File::open(FILE_PATH).unwrap()).lines().count();

    let mut last_id = 0;
    let mut i = 0;
    for line in f.lines(){
        i += 1;

        if i == g{
            let line_val: Vec<String> = line.unwrap().split_whitespace().map(|x| x.to_string()).collect();
            last_id = line_val[0].parse().unwrap();
        }
    }
    return (database, last_id);
}

fn save_to_db(mut db: File, content: String) {
    db
        .write(content.as_bytes())
        .expect("Unable to save to database");

    println!("Saved to db!")
}