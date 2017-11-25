use std::io;
use std::io::Write;
fn main (){
    loop {
        print!(">");
        io::stdout().flush().unwrap();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        interpret_line(&buf);
    }

}
fn interpret_line (line:&str){
    let line = String::from(line);
    let opts = line.split(' ').map(|x| String::from(x.to_lowercase().trim())).collect::<Vec<String>>();
    if opts.len() < 1 {
        print!("No commands entered. help for help.");
    } else {
    match opts[0].as_ref() {
        "help" => print_help(),
        _ => println!("No known command entered. help for help")
    }
    
    
    }
}
fn print_help() {
    println!("Commands:");
    println!("help: shows this screen");
}
