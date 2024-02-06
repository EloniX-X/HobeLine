use colored::Colorize;
use std::{io::{self, Write}};
use peekmore::{PeekMore};
use std::fs;
use std::env;
use std::path::Path;
mod coeditor;



fn main () {
    println!("");
    println!("Welcome to the {} {} terminal", "special".red(), "HobeOS".blue());
    println!("");
    actual_main()
}

fn actual_main() {
//send me back
   // let _traversal: Vec<String> = vec!["bring"]
    //let _intention: Vec<String> = vec!["list".to_string(), "delete".to_string(), "copy".to_string(), "paste".to_string(), "send".to_string(), "bring".to_string(), "create".to_string(), "move".to_string(), "code".to_string()];
    //let _where: Vec<String> =  vec!["this".to_string(), "back".to_string(), "forward".to_string(), "here".to_string(), "to".to_string(), "into".to_string()];
    //let _who: Vec<String> =  vec!["me".to_string(), "file".to_string(), "directory".to_string(), "directories".to_string(), "folders".to_string(), "all".to_string(), "folder".to_string()];
    let _helpcommands: Vec<String> = vec!["help".to_string(), "config".to_string(), "credits".to_string(), "code".to_string(), "blank".to_string()];
    let mut line = String::new();
    let current_dir = env::current_dir().unwrap();
    println!("\n{}{}{}", "----".red(), current_dir.display(), "----".red());

    print!("{}{}{} ", ">".red(),">".yellow(),">".green());
    let _ = io::stdout().flush();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut gnecko = line.split_whitespace().peekmore(); // removed mut
    let mut request: Vec<String> = Vec::new();

    //let mut itergnecko = gnecko.iter().peekmore();
    let cmd = line.trim();
    println!("{}", line);
    if _helpcommands.contains(&cmd.to_string()) {
       // println!("hi");
        helpprograms(cmd.to_string())
    } else {
        funcprogram(cmd.to_string())
    }
}



     //   funcprogram(&request);
      //      println!("{:?}", gnecko.peek()); // Safe to peek here
       // }
    
        //println!("{:?}", request);
   // }

    
    //if you want it to do something you have to say please
fn helpprograms(todo: String) {
    match todo.as_str() {
        "help" => {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
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
                _ => {
                    println!("canceling, change in config to not");
                    actual_main()
                    }, // Handle unexpected input
            }
        

        //    println!("-------{}--------", "Commands".green());
        },
        "code" => {
            let options = eframe::NativeOptions::default();
            match eframe::run_native(
                "Egui Quickstart",
                options,
                Box::new(|_cc| Box::new(coeditor::MyEguiApp::default())),
            ) {
                Ok(_) => println!("Application exited successfully"),
                Err(e) => eprintln!("Application exited with an error: {:?}", e),
            }
            
        }
        "config" => println!("config"),
        "blank" => {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            actual_main()
        },
        _ => println!("incompattible"),
       // _ => actual_main(),
        
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
    actual_main()
}

fn applications () {
    println!("------applications---");
    println!("type {} at any time to open {}!!:!!!!!", "code".on_green(), "coeditor".on_bright_magenta());
    println!("coeditor is a coding editor that is super cool which i use for everything\n simply because i love it so much i love everyhting i make everyone else \n should at least give it a try cus i put in some nice features")
}
//:D


// i need where here as well

fn funcprogram (todo: String) {
   // println!("{:?}", todo);
   let parts: Vec<&str> = todo.split_whitespace().collect();

    let first = parts.get(0);
    match first {
        Some(&"list") => {
            let path = ".";//if parts.contains(&"here") { "." } else { ".." };
            let paths = fs::read_dir(path).expect("Failed to read directory");
            for path in paths {
                println!("Name: {}", path.expect("Failed to read path").path().display());
            }
            
        },

        Some(&"move") => {
           // println!("moving "); // its just debug stuff
            if todo.contains(&"me".to_string()) {
               // println!("me back");
                if todo.contains(&"back".to_string()) {
                    println!("going");
                    let root = Path::new("../");
                    assert!(env::set_current_dir(&root).is_ok());

                }
                else if todo.contains(&"forward".to_string()) {
                    println!("going forward");
                }

            else  {
                let ownthis = &parts[2];
                
                let borrowed_string: &str = "./";
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










//which python3.9
//export PYO3_PYTHON=/opt/homebrew/bin/python3.9