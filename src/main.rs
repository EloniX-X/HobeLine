use colored::Colorize;
use std::{io::{self, Write}};
//use peekmore::{PeekMore};
use std::fs;
use std::env;
use std::path::Path;
mod coeditor;
//use std::{thread, time};
//use std::fs::File;
//use std::io::Cursor;
use serde_derive::{Serialize, Deserialize};
use std::process::exit;
use toml;


#[derive(Serialize, Deserialize)]
struct Data {
    defaults: DefaultsMenu,
    melol: Melol,
    theme: Theme,
    special: Special,
}

#[derive(Serialize, Deserialize)]
struct DefaultsMenu {
    defaultdirectory: String,
}

#[derive(Serialize, Deserialize)]
struct Special {
    ok: String,
    yes: String,
}

#[derive(Serialize, Deserialize)]
struct Melol {
    name: String,
    age: String,
}

#[derive(Serialize, Deserialize)]
struct Theme {
    maincolor: String,
    secondcolor: String,
}

//fn getinput() -> String {
    //print!("{}{}{} ", ">".red(),">".yellow(),">".green());
   // let mut line = String::new();
  //  let _ = io::stdout().flush();
 //   std::io::stdin().read_line(&mut line).unwrap();
//    line
//}


//fn getinputconfig() -> String {
   // let mut line = String::new();
  //  let _ = io::stdout().flush();
 //   std::io::stdin().read_line(&mut line).unwrap();
 //   line
//}


fn loadingconfig() {
    let configfilename = "/Users/elon/Documents/gocrazy/main/src/helloconfig.toml";
    let contents = match fs::read_to_string(configfilename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("cant read file {}", configfilename);
            exit(1);
        }
    };
    let data: Data = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("unable to load data from {}",configfilename);
            exit(1);
        }
    };
    let name = data.melol.name;
   //println!("weird but hot");
    if name == "" {
        //println!("not not");
        println!("would you like to make a new person or nah?\n(1) Yeah\n(2) No");
        let mut line = String::new();
        print!("{}{}{} ", ">".red(),">".yellow(),">".green());
        let _ = io::stdout().flush();
        std::io::stdin().read_line(&mut line).unwrap();
        match line.trim() {
            "1" => {
                println!("great, lets start makin a new person");
                createperson();
            },
            "2" => println!("ok"),
            _ => println!("please select an option")
        }
    } 
    //println!("running made ");
    actual_main();
    
}


fn createperson() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("---{} lets {} configuring you---", "hi".green(), "start".on_green());
    let mut lines = vec!["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()];
    //, "main color", "second main color", "default directory"];
    println!("           --{}--\nname: {}\nage: {}\n", "you".on_green(), lines[0], lines[1]);
    let mut i = 0;
    let mut maincolchose = false;
    while i < lines.len() { 
        let mut input = String::new();
      //  print!("{}{}{} ", ">".red(),">".yellow(),">".green());

        io::stdin().read_line(&mut input).expect("Failed to read line");
        lines[i] = input.trim().to_string();
        i += 1;
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("---{} lets {} configuring you---", "hi".green(), "start".on_green());
        //println!("{:?}", lines);
       // println!("{}", i);
        match i {
            0..=2 => println!("           --{}--\nname: {}\nage: {}\n", "you".on_green(), lines[0], lines[1]),
            3..=5 => {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                println!("---{} lets {} configuring you---", "hi".green(), "start".on_green());
                println!("           --{}--",  "theming".red());
      //          println!("              (1)     (2)     (3)    (4)    (5)    (6)");
                if maincolchose == false {
                    println!("main colors: {}, {}, {}, {}, {}, {}", "orange".bright_red(), "yellow".yellow(), "green".green(), "blue".blue(), "indigo".purple(), "violet".magenta());
                    println!("second colors: ");
                    maincolchose = true;
                } else {
                    println!("main colors: {}", lines[3]);
                    println!("second colors: {}, {}, {}, {}, {}, {}", "orange".bright_red(), "yellow".yellow(), "green".green(), "blue".blue(), "indigo".purple(), "violet".magenta());
                }
                if i == 5 {
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    println!("---{} lets {} configuring you---", "hi".green(), "start".on_green());
                    println!("           --{}--",  "theming".red());    
                    println!("main colors: {}", lines[3]);
                    println!("second colors: {}", lines[4]);
                }
            },
            6..=6 => {
                println!("ok welcome thanks for configuring everything");
                println!("heres all your options, everything you chose");
                println!("           --hi this is {} lol--\nname: {}\nage: {}\n", "you".on_green(), lines[0], lines[1]);
                println!("           ---{} this is your {} lol---", "hi".green(), "configuration".on_green());
                println!("main colors: {}", lines[3]);
                println!("second colors: {}", lines[4]);
                println!("restart if it doesnt look good (match being added later)");
            },
            7..=8 => {
                let filename = "src/helloconfig.toml";

                // Read the file
                let contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                    eprintln!("Could not read file `{}`", filename);
                    exit(1);
                });
                let mut data: Data = toml::from_str(&contents).unwrap_or_else(|_| {
                    eprintln!("Unable to load data from `{}`", filename);
                    exit(1);
                });
                println!("writing to the toml now <#3");
                println!("{:?}", lines);
                data.defaults.defaultdirectory = "".to_string();
                data.melol.name = lines[0].to_string();
                data.melol.age = lines[1].to_string();
                data.theme.maincolor = lines[3].to_string();
                data.theme.secondcolor = lines[4].to_string();

                let toml_string = toml::to_string_pretty(&data).unwrap_or_else(|_| {
                    eprintln!("Could not encode TOML value");
                    exit(1);
                });
                fs::write(filename, toml_string).unwrap_or_else(|_| {
                    eprintln!("Could not write to file `{}`", filename);
                    exit(1);
                });
                println!("written successfully")
//["eloni", "15", "", "orange", "yellow", "", "", "", ""]
               // let toml_string = toml::to_string(&config).expect("Failed to serialize");

            },
          //  4 => println!("hello ok"),
            _ => { println!("out, thanks");
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char)},
        }
        
        
    }

    
    actual_main()

}

fn main () {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    loadingconfig();
    println!("");
    println!("Welcome to the {} {} terminal", "special".red(), "Hobe".blue());
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
    //let mut gnecko = line.split_whitespace().peekmore(); // removed mut
    //let mut request: Vec<String> = Vec::new();

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
            println!("  -{}-   ", "HobeLine terminal".green());
            println!("    --by {}--  ", "Eloni".green());
            println!("---{} terminal---", "speckial".red());
            println!("Page(1)         {}", "Commands".green());
            println!("Page(2)         {}", "Applications".red());
            println!("Page(3)         {}(configurations)", "Utility".yellow());
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
                "3" => configmenu(),
                "three" => configmenu(),
                "utility" => configmenu(),
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
        "config" => configmenu(),
        "blank" => {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            actual_main()
        },
        _ => println!("incompattible"),
       // _ => actual_main(),
        
    }
}


fn configmenu () {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("--- Configuration Menu's ---");
    let homedir = dirs_next::home_dir().expect("Home directory not found");
    println!("under {}", "construction".on_red());
    println!("(1) set start {}       |      {:?}", "directory".on_blue(), homedir.display());
    println!("(2) show directory     |      {:?}", "true");



    print!("{}{}{} ", ">".red(),">".yellow(),">".green());
    let _ = io::stdout().flush();
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    match line.trim() {
        "1" => println!("one"),
        _ => {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            actual_main()},

    } 
   // println!("{:?}", homedir)
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
        Some(&"create") => {
            if todo.contains(&"folder".to_string()) {
                println!("creating string");
                let ownthis = &parts[2];
                fs::create_dir(ownthis).expect("uh oh");   
            }
            if todo.contains(&"text".to_string()) {
                let ownthis = &parts[2];
                let fileextnt: &str = ".txt";
                let thistofile = ownthis.to_string() + fileextnt.to_owned().as_str();
                println!("creating text {}", thistofile);
              //  let file = File::create(thistofile.to_string());
                let data = " hi ";
                fs::write(thistofile.to_string(), data).expect("Unable to write file");

            } else {
                println!("not an option");
                actual_main()
            }
        }

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
        None => println!("rpoblem"),
        _ => actual_main(),
    };

        actual_main()
       // Some(_) => actual_main(), 
        //None => println!("No command provided."), 
    } 










//which python3.9
//export PYO3_PYTHON=/opt/homebrew/bin/python3.9