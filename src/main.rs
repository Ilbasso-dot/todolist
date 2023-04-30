use std::fs;

fn main() {
    let args = std::env::args();

    let commands: Vec<String> = args.collect();
    let binding =
        fs::read_to_string("/home/ilbasso/Documents/myscript/todolist/src/test.txt").unwrap();
    let s: Vec<&str> = binding.as_str().split("\n\n").collect();
    // commands now contains all args after the executable name
    println!("Args: {:?}", commands);

    let mut v: Vec<(i32, i32, &str, Vec<&str>)> = Vec::new();

    for i in s {
        let mut ii = i.split('\n');
        let mut first = ii.next().unwrap().splitn(3, ' ');
        // println!("ehiiiiiiii {}, {}, {} ------", first.next().unwrap(), first.next().unwrap(), first.next().unwrap());
        let first: (i32, i32, &str, Vec<&str>) = (
            first.next().unwrap().parse::<i32>().unwrap(),
            first.next().unwrap().parse::<i32>().unwrap(),
            first.next().unwrap(),
            ii.collect(),
        );
        v.append(&mut vec![first]);
    }

    match commands[1].as_str() {
        "-l" => {
            for i in v {
                println!("{} {}   {}       {:?}", i.0, i.1, i.2, i.3);
            }
        }
        "-s" => {
            v.sort_by(|a, b| a.1.cmp(&b.1));
            for i in v {
                println!("{} {}   {}       {:?}", i.0, i.1, i.2, i.3);
            }
        }
        "-i" => {
            let new = (
                commands[2].parse::<i32>().unwrap(),
                commands[3].parse::<i32>().unwrap(),
                commands[4].as_str(),
                commands[5..]
                    .iter()
                    .map(|s| s as &str)
                    .collect::<Vec<&str>>(),
            );
            v.push(new);
            v.sort_by(|a, b| a.0.cmp(&b.0));
            // write to file
            let mut s = String::new();
            for i in v {
                s.push_str("\n");
                s.push_str(&format!("{} {} {}\n", i.0, i.1, i.2));
                for j in i.3 {
                    s.push_str(&format!("{}\n", j));
                }
            }
            // println!("{}", s);
            // remove first \n and last \n
            s.remove(0);
            s.pop();
            std::fs::write("/home/ilbasso/Documents/myscript/todolist/src/test.txt", s)
                .expect("Unable to write file");
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
            // write to file
            let mut s = String::new();
            for i in v {
                s.push_str("\n");
                s.push_str(&format!("{} {} {}\n", i.0, i.1, i.2));
                for j in i.3 {
                    s.push_str(&format!("{}\n", j));
                }
            }
            // println!("{}", s);
            std::fs::write("/home/ilbasso/Documents/myscript/todolist/src/test.txt", s)
                .expect("Unable to write file");
        }
        _ => {}
    }

    // print v
}
