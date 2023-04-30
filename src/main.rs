use std::fs;

// help function
fn help() {
    println!("Usage: todolist [OPTION] [ARGUMENTS]");
    println!("Options:");
    // write all options
    println!("\t-l\t List all tasks");
    println!("\t-i\t Add a new task");
    println!("\t-r\t Remove a task");
    println!("\t-e\t Show all tasks that can be done in a given effort");
    println!("\tkate\t Open todolist file with kate");
    println!("\t-h\t Show this help");
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
    let args: std::env::Args = std::env::args();

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
                println!("{})  priority: {}, \teffort: {}\t=>  {}", n, i.0, i.1, i.2);
            }
        }
        "-i" => {
            let name = commands[4..].join(" ");
            let new = (
                commands[2].parse::<i32>().unwrap(),
                commands[3].parse::<i32>().unwrap(),
                name.as_str(),
            );
            v.push(new);
            v.sort_by(|a, b| a.0.cmp(&b.0));
            write_file(&v);
            for (n, i) in v.iter().enumerate() {
                println!("{})  priority: {}, \teffort: {}\t=>  {}", n, i.0, i.1, i.2);
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
                        println!("{})  priority: {}, \teffort: {}\t=>  {}", n, i.0, i.1, i.2);
                    }
                    return;
                }
            }
            v.remove(0);
            write_file(&v);
            for (n, i) in v.iter().enumerate() {
                println!("{})  priority: {}, \teffort: {}\t=>  {}", n, i.0, i.1, i.2);
            }
        }
        "-e" => {
            if commands.len() < 3 {
                let mut effort = 0;
                for i in &v {
                    effort += i.1;
                }
                println!("Total effort: {}", effort);
                return;
            }
            let max_effort = commands[2].parse::<i32>().unwrap();
            let mut effort = 0;
            let mut i = 0;
            while effort <= max_effort {
                effort += v[i].1;
                i += 1;
            }
            // print which task can be done
            for j in 0..i - 1 {
                let k = v[j];
                println!("{})  priority: {}, \teffort: {}\t=>  {}", j, k.0, k.1, k.2);
            }
        }
        "-h" => help(),
        "kate" => {
            // opening kate from command line
            std::process::Command::new("kate")
                .arg("/home/ilbasso/Documents/myscript/todolist/src/test.txt")
                .spawn()
                .expect("kate command failed to start");
        }
        _ => {}
    }

    // print v
}
