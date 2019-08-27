use std::net::{TcpStream};
use std::thread;
use std::io::prelude::*;
use std::io;
use std::io::BufReader;

fn main() {
    let mut sIp         = String::new();
    let mut sPort       = String::new();
    let mut sPass       = String::new();
    let mut sUser       = String::new();
    //---------grab data---------//

    println!("Starting!");
    println!("Server IP: ");
    io::stdin()
        .read_line(&mut sIp)
        .expect("Somehow couldn't read what you typed..");
                                

    println!("\nServer Port: ");
    io::stdin()
        .read_line(&mut sPort)
        .expect("Somehow couldn't read what you typed..");

    println!("\nServer Password: ");
    io::stdin()
        .read_line(&mut sPass)
        .expect("Somehow couldn't read what you typed..");

    println!("\nUsername to login as: ");
    io::stdin()
        .read_line(&mut sUser)
        .expect("Somehow couldn't read what you typed..");

    //-----------Parse----------//

    let sPort:u16 = sPort.trim().parse().unwrap();
    sIp = String::from(sIp.trim());
    sUser = String::from(sUser.trim());

    //----------Connect---------//
    
    let mut stream = TcpStream::connect((sIp.as_str(), sPort)).expect("Could not find or connect to the server!");
    
    let mut streamer = stream.try_clone().expect("Internal error, couldnt clone");
    
    stream.write(sPass.as_bytes())
        .expect("Couldnt send password");
    stream.write(["print('",sUser.as_str()," has connected using a messenger chicken')\n"].concat().as_bytes());

    //----------Chunky parts---------//

    //Read rcon
    thread::spawn(move || {
        let mut reader = BufReader::new(streamer);
        loop{
            let mut response = String::new();
            let result = reader.read_line(&mut response).expect("Could not read message from server");
            if(result == 0){
                println!("Server connection has been closed");
                break;
            }
            println!("Message : {}",response.replace('\n',""));
            
        }
    });

    //Write rcon
    loop{
        //cursor().goto(0, w_height);
        let mut userM = String::new();
        io::stdin()
        .read_line(&mut userM)
        .expect("Somehow couldn't read what you typed..");

        stream.write(userM.as_bytes()).expect("Server is down");
    };

    //----------End---------//
}
