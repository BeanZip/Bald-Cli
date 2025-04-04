use clap::Parser;
use std::process::Command;
use std::io;
use std::thread;
use std::time::Duration;
use std::env;
use third_rust_app::api::get_joke;

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
    script
}


fn setup() -> Result<(), std::env::VarError>{
    
  //TODO: Setup nvim installation 
   if cfg!(windows) {
        println!("System is Windows");
        let mut script = "Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
        Invoke-RestMethod -Uri https://get.scoop.sh | Invoke-Expression";
     println!("Beginning to Install Scoop.....");
         run(script);
        thread::sleep(Duration::from_secs(5));
        println!("Setting up Neovim..");
        script = "scoop install neovim";
        run(script);
        thread::sleep(Duration::from_secs(5));
        script = "scoop install git";
        run(script);
        let key = "USERPROFILE";
        let val = env::var(key)?;
        match env::var(key){
            Ok(val) => { println!("{}: Set as {:?}",key,val)},
            Err(e) => println!("Couldn't interpret {} because of {}. Make sure to set {} properly",key,e,key), 
        }

        run(format!(r"cd {}\AppData\Local\nvim",val).as_str());
        println!("Inject NvChad? (y/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Variable Not Found");

        match input.as_str(){
          "y" => { run("git clone https://github.com/NvChad/starter"); }
          "n" => println!("Stopping value"),

          _ => println!("invalid response")
        }
    } else if cfg!(unix){
        println!("System is a Unix-based OS (Linux or MacOS)");

    } else{
        println!("DAWG WHAT ARE YOU USING THIS ON A TOASTER!???");
        println!("Nah bro's on a {:?} ",std::env::consts::OS);
    }

    Ok(())

}

fn code_download(input: String){
let input = input.trim().to_lowercase();
thread::sleep(Duration::from_secs(2));
    match input.as_str(){
        "py" => { 
            println!("{} is being read....",input); 
            thread::sleep(Duration::from_secs(5));
            run("scoop install python");
        },
        "c" | "cpp" =>{
            println!("{} is being read..",input);
            thread::sleep(Duration::from_secs(5));
            run("scoop install gcc");
            thread::sleep(Duration::from_secs(10));
            run("scoop install clangd");
        },
        "rs" =>{
            println!("{} is being read...",input);
            thread::sleep(Duration::from_secs(5));
            run("scoop install rust");
        },
        "cs" =>{
            println!("{} is being read...",input);
            thread::sleep(Duration::from_secs(5));
            run("scoop install dotnet");
        }
        _ => { println!("Compiler not made for {}",input.trim().to_lowercase()); }
    }
}
#[tokio::main]
async fn main() {
    let args = Cli::parse();
    match args.section.as_str() {
        "bald" => match args.section_command.as_str() {
            "hello" => println!("Hello There"),
            "goodbye" => {
                run("shutdown /r /t 5 && echo your pc is going to sleep.....");
            },
            "build-my-vim" => {
                let _ = setup();            
            },
            "code" =>{
                println!("Insert desired lang by file extension");
                print!("> ");
               let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Variable Not Found");
                code_download(input);
            },
            "joke" =>{
                match get_joke().await{
                    Ok(joke) => println!("{}",joke),
                    Err(e) => println!("Error Getting The Joke: {}",e),
                }
            },
            _ => println!("Nothing There"),
        },
        _ => println!("Invalid Command Provided."),
    }
}
