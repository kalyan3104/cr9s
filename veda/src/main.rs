#[allow(dead_code)]
enum Command {
    Start,
    Stop,
    Pause,
}

fn main (){
    let paint = Command::Stop;

    match paint {
        Command::Pause=>{
            println!("Command is Pause");
        }
        Command::Start=>{
            println!("Command is Start");
        }
        Command::Stop=>{
            println!("Command is Stop");
        }
    }
}