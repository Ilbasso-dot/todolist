use std::fs;
// use std::path::PathBuf;
//let file_path = dotenv::var("FILE_PATH").unwrap();

/// help function
fn help() {
    println!("Usage: todolist [OPTION] [ARGUMENTS]");
    println!("Options:");
    println!("\t-l\t List all tasks");
    println!("\t-i\t Add a new task");
    println!("\t-r\t Remove a task");
    println!("\t-e\t Show all tasks that can be done in a given effort");
    println!("\tkate\t Open todolist file with kate");
    println!("\t-h\t Show this help");
}

/// read and parse file
fn read_file() -> Vec<(i32, i32, String)> {
    let file_path = "/home/ilbasso/Documents/myscript/todolist/src/test.txt";
    let binding = fs::read_to_string(file_path).unwrap();
    let s: Vec<String> = binding.split('\n').map(String::from).collect();
    let mut v: Vec<(i32, i32, String)> = Vec::new();
    for i in s {
        let mut first = i.splitn(3, ' ');
        let first: (i32, i32, String) = (
            first.next().unwrap().parse::<i32>().unwrap(),
            first.next().unwrap().parse::<i32>().unwrap(),
            String::from(first.next().unwrap()),
        );
        v.append(&mut vec![first]);
    }
    v
}

/// write file
fn write_file(v: &Vec<(i32, i32, &str)>) {
    let file_path = "/home/ilbasso/Documents/myscript/todolist/src/test.txt";
    let mut s = String::new();
    for i in v {
        s.push_str(&format!("{} {} {}\n", i.0, i.1, i.2));
    }
    s.pop();
    std::fs::write(file_path, s).expect("Unable to write file");
}

/// print all tasks
fn print_all(v: &[(i32, i32, &str)]) {
    for (n, i) in v.iter().enumerate() {
        println!("{})  priority: {}, \teffort: {}\t=>  {}", n, i.0, i.1, i.2);
    }
}

fn main() {
    // commands parser
    let args: std::env::Args = std::env::args();
    let commands: Vec<String> = args.collect();

    // read file
    let v: Vec<(i32, i32, String)> = read_file();
    let mut v: Vec<(i32, i32, &str)> = v.iter().map(|(a, b, c)| (*a, *b, c.as_str())).collect();

    match commands[1].as_str() {
        "-l" => {
            // -l           -> list all tasks
            v.sort_by(|a, b| {
                if a.0 == b.0 {
                    a.1.cmp(&b.1)
                } else {
                    a.0.cmp(&b.0)
                }
            });
            print_all(&v);
        }
        "-i" => {
            // -i [priority] [effort] [name]    -> add a new task
            let name = commands[4..].join(" ");
            let new: (i32, i32, &str) = (
                commands[2].parse::<i32>().unwrap(),
                commands[3].parse::<i32>().unwrap(),
                name.as_str(),
            );
            v.push(new);
            v.sort_by(|a, b| {
                if a.0 == b.0 {
                    a.1.cmp(&b.1)
                } else {
                    a.0.cmp(&b.0)
                }
            });
            write_file(&v);
            print_all(&v);
        }
        "-r" => {
            // -r           -> remove first task
            // -r [index]   -> remove task at index
            // -r [index] [index] ... -> remove tasks at index
            match commands.len() {
                n if n < 2 => {
                    println!("Invalid command");
                }
                2 => {
                    v.remove(0);
                }
                3 => {
                    let index = commands[2].parse::<i32>().unwrap();
                    if index < v.len() as i32 {
                        v.remove(index as usize);
                    } else {
                        println!("Invalid index");
                    }
                }
                _ => {
                    for i in &commands[2..] {
                        let index = i.parse::<i32>().unwrap();
                        if index < v.len() as i32 {
                            v.remove(index as usize);
                        } else {
                            println!("Invalid index");
                        }
                    }
                }
            }
            
            write_file(&v);
            print_all(&v);
        }
        "-e" => {
            // -e           -> sum all effort
            // -e [effort]  -> show all tasks that can be done in a given effort
            if commands.len() < 3 {
                let mut effort = 0;
                for i in &v {
                    effort += i.1;
                }
                println!("Total effort: {}", effort);
            } else {
                let max_effort = commands[2].parse::<i32>().unwrap();
                let mut effort = 0;
                let mut i = 0;
                while effort <= max_effort {
                    effort += v[i].1;
                    i += 1;
                }
                for (j, k) in v.iter().enumerate().take(i - 1) {
                    println!("{})  priority: {}, \teffort: {}\t=>  {}", j, k.0, k.1, k.2);
                }
            }
        }
        "-h" => help(),
        "kate" => {
            // kate         -> open todolist file with kate
            let file_path = "/home/ilbasso/Documents/myscript/todolist/src/test.txt";
            std::process::Command::new("kate")
                .arg(file_path)
                .spawn()
                .expect("kate command failed to start");
        }
        _ => println!("Invalid command"),
    }
}
