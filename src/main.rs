use clap::Parser;
use std::process::Command;
use std::io;
use std::thread;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(name = "cli-tool")]
struct Cli {
    section: String,
    section_command: String,
}

fn run(script: &str) -> &str{
 println!("Running script {}", script);
 let com = Command::new("powershell").args(["/C",script]).output().expect("Failed to execute command");
 println!("{}", String::from_utf8_lossy(&com.stdout));

    return script;
}


fn setup(){
    
  //TODO: Setup nvim installation 
   if cfg!(windows) {
        println!("System is Windows");
        let mut script = "Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
        Invoke-RestMethod -Uri https://get.scoop.sh | Invoke-Expression";
     println!("Beginning to Install Scoop.....");
         run(script);
        println!("Setting up Neovim..");
        script = "scoop install neovim";
        run(script);
        //TODO: Figure out how to get user path to install plugins and such.

    } else if cfg!(unix){
        println!("System is a Unix-based OS (Linux or MacOS)");
    } else{
        println!("DAWG WHAT ARE YOU USING THIS ON A TOASTER!???");
        println!("Nah bro's on a {:?} ",std::env::consts::OS);
    }

}

fn code_download(input: String){

            thread::sleep(Duration::from_secs(2));
 match input.as_str(){
        "cpp" | "c" =>{
           run("scoop install gcc");
        },
        "py" =>{
           run("scoop install python");
        },
        _ => println!("Couldn't find compiler for {}",input)
    } 
}
fn main() {
    let args = Cli::parse();

    match args.section.as_str() {
        "bald" => match args.section_command.as_str() {
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
            "code" =>{
                println!("Insert desired lang by file extension");
               let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Variable Not Found");
                code_download(input);
            },
            _ => println!("Nothing There"),
        },
        _ => println!("Invalid Command Provided."),
    }
}
