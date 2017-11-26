use std::io;
use std::io::Write;
use std::fs::DirEntry;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Read;
use std::str::FromStr;
use std::collections::HashMap;
struct Npc {
    name:String,
    skills:HashMap<String, i32>,
    age:i32,
    class:String,
}
impl Npc {
    fn to_string(&self) -> String {
        let skills:String = self.skills.keys().map(|key| String::from("|")+&key+":"+&format!("{}",self.skills.get(key).unwrap())).collect::<String>();
        String::from("|\n")+&self.name+"\n"+&format!("{}", &self.age)+"\n"+&self.class+"\n"+&skills
    }
}
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
            "gen" => {
                match opts[1].to_lowercase().as_ref() {
                    "trash" => gen_npc(&get_camp(get_editing()).unwrap().1, true),
                    _ => println!("I don't know what you're trying to generate"),
                }
            }
            "info" => {
                let results = get_camp(get_editing()).unwrap();
                println!(
                    "Name:{}\nSystem:{}\nDescription:{}",
                    results.0,
                    results.1,
                    results.2
                )
            }
            "help" => print_help(),
            "use" => switch_campaign(opts[1].as_ref()),
            _ => println!("No known command entered. help for help"),
        }
    }
}
fn add_npc(npc:Npc, camp:&str){
    if fs::metadata(get_home()+"/.config/carbon/campaigns/"+camp+"/npcs").is_ok(){
        OpenOptions::new().create(true).write(true).append(true).open(get_home()+"/.config/carbon/campaigns/"+camp+"/npcs").unwrap().write_all(npc.to_string().as_bytes()).unwrap();
    }
}
fn gen_npc(sys: &str, disp: bool) {
    match sys {
        "c2020" => {

        },
        _ => println!("{} is not a system with npc generating rules",sys)
    }

}
fn get_camp(campaign: String) -> Result<(String, String, String), &'static str> {
    let mut vec: Vec<String>;
    println!(
        "{}",
        get_home() + "/.config/carbon/campaigns/" + &campaign + "/camp"
    );
    io::stdout().flush().unwrap();
    if fs::metadata(
        get_home() + "/.config/carbon/campaigns/" + &campaign + "/camp",
    ).is_ok()
    {
        let mut buf = String::new();
        fs::File::open(
                get_home() + "/.config/carbon/campaigns/" + &campaign + "/camp",
            )
            .unwrap()
            .read_to_string(&mut buf)
            .unwrap();
        vec = buf.split('\n').map(String::from).collect::<Vec<String>>();
        Ok((
            String::from(vec[0].as_ref()),
            String::from(vec[1].as_ref()),
            String::from(vec[2].as_ref()),
        ))
    } else {
        println!("This campaign doesn't exist");
        Err("DNE")
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
        switch_campaign(&name);
    } else {
        println!("There is already a campaign named {}", name);
    }
}
fn switch_campaign(name: &str) {
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(get_home() + "/.config/carbon/editing")
        .unwrap()
        .write_all(String::from(name).as_bytes())
        .unwrap();
    println!("You are now editing {}", name);
}
fn get_editing() -> String {
    let mut buf = String::new();
    fs::File::open(get_home() + "/.config/carbon/editing")
        .expect("bad")
        .read_to_string(&mut buf)
        .expect("badder");
    String::from(buf.trim())
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
