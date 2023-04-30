use std::fs;

fn write_file(v: Vec<(i32, i32, &str)>) {
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
    println!("Args: {:?}", commands);

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
            for i in v {
                println!("priority: {}, effort: {} =>  {}", i.0, i.1, i.2);
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
            write_file(v);
        }
        "-r" => {
            v.remove(0);
            write_file(v);
        }
        "-d" => {
            let mut i = 0;
            for j in &v {
                if j.0 == commands[2].parse::<i32>().unwrap() {
                    break;
                }
                i += 1;
            }
            v.remove(i);
            write_file(v);
        }
        _ => {}
    }

    // print v
}
