use colored::Colorize;
use std::{io::{self, Write}};
use peekmore::{PeekMore};
use std::fs;
use std::env;
use std::path::Path;
mod coeditor;



fn main () {
    println!("");
    println!("Welcome to the {} {} terminal", "speckial".red(), "HobeOS".blue());
    println!("");
    actual_main()
}

fn actual_main() {
//send me back
   // let _traversal: Vec<String> = vec!["bring"]
    let _intention: Vec<String> = vec!["list".to_string(), "delete".to_string(), "copy".to_string(), "paste".to_string(), "send".to_string(), "bring".to_string(), "create".to_string(), "move".to_string(), "code".to_string()];
    let _where: Vec<String> =  vec!["this".to_string(), "back".to_string(), "forward".to_string(), "here".to_string(), "to".to_string(), "into".to_string()];
    let _who: Vec<String> =  vec!["me".to_string(), "file".to_string(), "directory".to_string(), "directories".to_string(), "folders".to_string(), "all".to_string(), "folder".to_string()];
    let _helpcommands: Vec<String> = vec!["help".to_string(), "config".to_string(), "credits".to_string(), "code".to_string()];
    let mut line = String::new();
    let current_dir = env::current_dir().unwrap();
    println!("\n{}{}{}", "----".red(), current_dir.display(), "----".red());

    print!("{}{}{} ", ">".red(),">".yellow(),">".green());
    let _ = io::stdout().flush();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut gnecko = line.split_whitespace().peekmore(); // removed mut
    let mut request: Vec<String> = Vec::new();

    //let mut itergnecko = gnecko.iter().peekmore();
    while let Some(drop) = gnecko.peek() {

        let drop_str = drop.to_string(); 
        //println!("{:?}", drop);
        if _intention.contains(&drop.to_string()) {
            //println!("intention: {}", drop);
            request.push(drop_str.clone())
        } 
        if _who.contains(&drop.to_string()) {
            //println!("who: {}", drop.to_string());
            request.push(drop_str.clone())
        }
        gnecko.next();
        if _where.contains(&drop_str) {
         //   println!("who: {}", drop_str)
            match gnecko.peek() {
                Some(optional) => {
                   // let optional = gnecko.peek().unwrap();
                //  println!("{:?}", optional);
                    request.push(optional.to_string());
                    
                },
            None => { request.push(drop_str.to_string()); }
        }
        }
        if _helpcommands.contains(&drop_str) {
            helpprograms(drop_str.to_string())
        }

        funcprogram(&request);
          //  println!("{:?}", gnecko.peek()); // Safe to peek here
        }
    
        //println!("{:?}", request);
    }

    
    //if you want it to do something you have to say please
fn helpprograms(todo: String) {
    match todo.as_str() {
        "help" => {

            println!("");
            println!("---{} terminal---", "speckial".red());
            println!("Page(1)         {}", "Commands".green());
            println!("Page(2)         {}", "Applications".red());
            println!("Page(3)         {}", "Utility".yellow());
            println!("Page(4)         {}", "Credits".blue());
            println!("{}", "Cancel".red());

            print!("{}{}{} ", ">".red(),">".yellow(),">".green());
            let _ = io::stdout().flush();
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            match line.trim() {
                "1" => commands(),
                "one" => commands(),
                "commands" => commands(),
                "2" => applications(),
                "two" => applications(),
                "applications" =>applications(),
                _ => println!("rip"), // Handle unexpected input
            }
        

        //    println!("-------{}--------", "Commands".green());
        },
        "code" => {
            let options = eframe::NativeOptions::default();
            eframe::run_native(
                "Egui Quickstart",
                options,
                Box::new(|_cc| Box::new(coeditor::MyEguiApp::default())), // Instantiate `MyEguiApp` from the `gui` module.
            );
        }
        "config" => println!("config"),
        _ => println!("rip"),
        
    }
}

fn commands () {
    println!("---commands---");
    println!("Intention commands");
    println!("{} - list either files or directories", "list".green());
    println!("{} - move either yourself, files or directories", "move".blue());
    println!("{} - delete either files or directories", "delete".red());
    println!("{} - copy either files or directories", "copy".yellow());
    println!("{} - past either files or directories", "paste".yellow());
    println!("{} - create a new file or directory", "create".green());
    println!("\n---examples---");
    println!("{} me into {}", "send".green(), "home".blue());
    println!("{} directory {}", "create".green(), "main".blue());

}

fn applications () {
    println!("------applications---");
    println!("type {} at any time to open {}!!:!!!!!", "code".on_green(), "coeditor".on_bright_magenta());
    println!("coeditor is a coding editor that is super cool which i use for everything\n simply because i love it so much i love everyhting i make everyone else \n should at least give it a try cus i put in some nice features")
}
//:D


// i need where here as well

fn funcprogram (todo: &[String]) {
   // println!("{:?}", todo);

    let first = todo.first();
    match first {
        Some(cmd) if cmd == "list" => {
                let mut path = "../";
                if todo.contains(&"here".to_string()) {
                    path = ".";
                }
                let paths = fs::read_dir(path).unwrap();

                for path in paths {
                    println!("Name: {}", path.unwrap().path().display());
                }
            }
        Some(cmd) if cmd == "move" => {
            if todo.contains(&"me".to_string()) {
                if todo.contains(&"back".to_string()) {
                    println!("going");
                    let root = Path::new("../");
                    assert!(env::set_current_dir(&root).is_ok());

                }
                else if todo.contains(&"forward".to_string()) {
                    println!("going forward");
                }

            else  {
                let ownthis = &todo[2];
                
                let borrowed_string: &str = "../";
                let together = borrowed_string.to_owned() + &ownthis;

                println!("{}", together);
                let root = Path::new(&together);
                assert!(env::set_current_dir(&root).is_ok());
                println!("Successfully changed working directory to {}!", root.display());
            }
            }
        },
        _ => actual_main(),
    };

        actual_main()
       // Some(_) => actual_main(), 
        //None => println!("No command provided."), 
    } 