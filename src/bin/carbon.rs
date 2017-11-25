use std::io;
use std::io::Write;
use std::fs::DirEntry;
use std::env;
use std::fs;
use std::fs::OpenOptions;
fn main() {
    if fs::metadata(get_home() + "/.config/carbon").is_err() {
        init();
    } else {
        loop {
            print!(">");
            io::stdout().flush().unwrap();
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();
            interpret_line(&buf);
        }
    }
}
fn init() {
    fs::create_dir(get_home() + "/.config/carbon").unwrap();
    fs::create_dir(get_home() + "/.config/carbon/campaigns").unwrap();
}
fn interpret_line(line: &str) {
    let line = String::from(line);
    let opts = line.split(' ')
        .map(|x| String::from(x.to_lowercase().trim()))
        .collect::<Vec<String>>();
    if opts.len() < 1 {
        print!("No commands entered. help for help.");
    } else {
        match opts[0].to_lowercase().as_ref() {
            "new" => {
                match opts[1].to_lowercase().as_ref() {
                    "campaign" => create_campaign(),
                    "npc" => println!("wip"),
                    _ => println!("I don't know what you're trying to make"),
                }
            }
            "help" => print_help(),
            _ => println!("No known command entered. help for help"),
        }
    }
}
fn create_campaign() {
    let mut name = String::new();
    print!("name >");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();
    name = String::from(name.to_lowercase().trim());
    if fs::metadata(get_home() + "/.config/carbon/campaigns/" + &name).is_err() {
        fs::create_dir(get_home() + "/.config/carbon/campaigns/" + &name).unwrap();
        fs::create_dir(get_home() + "/.config/carbon/campaigns/" + &name + "/npcs").unwrap();
        let mut sys = String::new();
        let mut desc = String::new();
        print!("system >");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut sys).unwrap();
        sys = String::from(sys.to_lowercase().trim());
        print!("description >");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut desc).unwrap();
        desc = String::from(desc.to_lowercase().trim());
        let camp = String::from(String::from(name.as_ref()) + "\n" + &sys + "\n" + &desc);
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(get_home() + "/.config/carbon/campaigns/" + &name + "/camp")
            .unwrap()
            .write_all(camp.as_bytes())
            .unwrap();
        println!(
            "The campaign named {}, built on the system {} has been created",
            &name,
            &sys
        );
    } else {
        println!("There is already a campaign named {}", name);
    }
}
fn print_help() {
    println!("Commands:");
    println!("help: shows this screen");
}
fn get_home() -> String {
    return String::from(env::home_dir().unwrap().to_str().unwrap());
}
fn proc_path(path: DirEntry) -> String {
    //Converts DirEntry into a fully processed file/directory name
    let base = path.file_name().into_string().unwrap();
    return base;
}
