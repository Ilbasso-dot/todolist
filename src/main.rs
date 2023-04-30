use std::fs;

// help function
fn help() {
    println!("Usage: todolist [OPTION] [ARGUMENTS]");
    println!("Options:");
    // write all options
    println!("\t-l\t List all tasks");
    println!("\t-i\t Add a new task");
    println!("\t-r\t Remove a task");
}

fn write_file(v: &Vec<(i32, i32, &str)>) {
    let mut s = String::new();
    for i in v {
        s.push_str(&format!("{} {} {}\n", i.0, i.1, i.2));
    }
    s.pop();
    std::fs::write("/home/ilbasso/Documents/myscript/todolist/src/test.txt", s)
        .expect("Unable to write file");
}

fn main() {
    let args = std::env::args();

    let commands: Vec<String> = args.collect();
    let binding =
        fs::read_to_string("/home/ilbasso/Documents/myscript/todolist/src/test.txt").unwrap();
    let s: Vec<&str> = binding.as_str().split('\n').collect();
    // commands now contains all args after the executable name
    // println!("Args: {:?}", commands);

    let mut v: Vec<(i32, i32, &str)> = Vec::new();

    for i in s {
        let mut first = i.splitn(3, ' ');
        let first: (i32, i32, &str) = (
            first.next().unwrap().parse::<i32>().unwrap(),
            first.next().unwrap().parse::<i32>().unwrap(),
            first.next().unwrap(),
        );
        v.append(&mut vec![first]);
    }

    match commands[1].as_str() {
        "-l" => {
            v.sort_by(|a, b| a.0.cmp(&b.0));
            for (n, i) in v.iter().enumerate() {
                println!("{}) priority: {}, effort: {} =>  {}", n, i.0, i.1, i.2);
            }
        }
        "-i" => {
            let new = (
                commands[2].parse::<i32>().unwrap(),
                commands[3].parse::<i32>().unwrap(),
                commands[4].as_str(),
            );
            v.push(new);
            v.sort_by(|a, b| a.0.cmp(&b.0));
            write_file(&v);
            for (n, i) in v.iter().enumerate() {
                println!("{}) priority: {}, effort: {} =>  {}", n, i.0, i.1, i.2);
            }
        }
        "-r" => {
            // if commands[2] exists, parse to int and remove it
            if commands.len() > 2 {
                let index = commands[2].parse::<i32>().unwrap();
                if index < v.len() as i32 {
                    v.remove(index as usize);
                    write_file(&v);
                    for (n, i) in v.iter().enumerate() {
                        println!("{}) priority: {}, effort: {} =>  {}", n, i.0, i.1, i.2);
                    }
                    return;
                }
            }
            v.remove(0);
            write_file(&v);
            for (n, i) in v.iter().enumerate() {
                println!("{}) priority: {}, effort: {} =>  {}", n, i.0, i.1, i.2);
            }
        }
        "-h" => help(),
        _ => {}
    }

    // print v
}
