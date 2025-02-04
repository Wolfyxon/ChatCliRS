use utils::input;

mod utils;
mod server;
mod client;

fn main() {
    println!("Welcome to ChatCliRS");
    println!("What would you like to do?:");

    println!("1. Host");
    println!("2. Join");
    println!("3. Quit");
    
    query_main_mode();
}

fn query_main_mode() {
    let action = input();

    match action.as_str() {
        "1" => {
            println!("Starting server");
            
            let port = 2400; // TODO: input port from user

            server::host(2400);
            client::join(format!("localhost:{port}"));
        }
        "2" => {
            print!("Enter address: ");
            let addr = input();

            println!("Joining...");
            client::join(addr);
        }
        "3" => {
            println!("Goodbye");
        }
        
        _ => {
            println!("Invalid action");
            query_main_mode();
        }
    }
}