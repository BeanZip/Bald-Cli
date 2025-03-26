use clap::Parser;
use std::process::Command;

// Search for a file pattern and display lines that it's contained in.
#[derive(Parser, Debug)]
#[command(name = "cli-tool")]
struct Cli {
    section: String,
    section_command: String,
}

fn run(script: &str){
 let com = Command::new("powershell").args(["/C",script]).output().expect("Failed to execute command");
 println!("{}", String::from_utf8_lossy(&com.stdout));
}


fn setup(){
    
  //TODO: Setup nvim installation 
   if cfg!(windows) {
        println!("System is Windows");
        let mut script = "Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
        Invoke-RestMethod -Uri https://get.scoop.sh | Invoke-Expression";
         println!("Beginning to Install Scoop.....");
         run(script);
        

    } else if cfg!(unix){
        println!("System is a Unix-based OS (Linux or MacOS)");
    } else{
        println!("DAWG WHAT ARE YOU USING THIS ON A TOASTER!???");
        println!("Nah bro's on a {:?} ",std::env::consts::OS);
    }

}
fn main() {
    let args = Cli::parse();

    match args.section.as_str() {
        "first-sec" => match args.section_command.as_str() {
            "hello" => println!("Hello There"),
            "goodbye" => {
                let output = Command::new("cmd")
                    .args(["/C", "shutdown /r /t 5 && echo time for your pc's nap time"])
                    .output()
                    .expect("Failed to execute command");
                println!("{}", String::from_utf8_lossy(&output.stdout));
            },
            "build-my-vim" => {
                setup();
            },
            _ => println!("Nothing There"),
        },
        _ => println!("Invalid Command Provided."),
    }
}
