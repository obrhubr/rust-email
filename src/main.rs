#[macro_use]
extern crate dotenv_codegen;

use std::{net::{TcpStream}, io::{Read, Write}};
use native_tls::{TlsConnector, TlsStream};
use base64::{encode};

extern crate dotenv;
use dotenv::dotenv;

trait Client {
    fn read_lines(&mut self) -> std::io::Result<String>;
    fn send(&mut self, message: String) -> std::io::Result<()>;

    fn send_message(&mut self, hostname: String, to: String, to_name: String, from: String, from_name: String, subject: String, message: String, username: String, password: String) -> std::io::Result<()>;
}

struct SMTPClient {
    stream: TlsStream<TcpStream>
}

impl Client for SMTPClient {
    fn read_lines(&mut self) -> std::io::Result<String> {
        let mut message = String::new();
    
        loop {
            let mut buffer = [0; 128];
            let read_length = self.stream.read(&mut buffer)?;
    
            let character = String::from_utf8_lossy(&buffer[..]);
            message += &character;

            if read_length < 128 {
                break;
            }
        }
        
        println!("{}", message);
        Ok(message) 
    }

    fn send(&mut self, message: String) -> std::io::Result<()> {
        self.stream.write_all(message.as_bytes())?;

        print!("{}", message);
        Ok(())
    }

    fn send_message(&mut self, hostname: String, to: String, to_name: String, from: String, from_name: String, subject: String, message: String, username: String, password: String) -> std::io::Result<()> {
        let _connection_message = self.read_lines()?;
        self.send(format!("EHLO {}\r\n", hostname))?;
        let _hello_response = self.read_lines()?;
        self.send(format!("AUTH LOGIN {}\r\n", encode(username)))?;
        let _hello_response = self.read_lines()?;
        self.send(format!("{}\r\n", encode(password)))?;
        self.send(format!("MAIL FROM:<{}>\r\n", from))?;
        let _from_ok = self.read_lines()?;
        self.send(format!("RCPT TO:<{}>\r\n", to))?;
        let _to_ok = self.read_lines()?;
        self.send("DATA\r\n".to_string())?;
        let _data_ending = self.read_lines()?;

        // Send Headers
        self.send(format!("From: \"{}\" <{}>\r\n", from_name, from))?;
        self.send(format!("To: \"{}\" <{}>\r\n", to_name, to))?;
        self.send(format!("Subject: {}\r\n", subject))?;
        // Send Message
        self.send(message)?;
        // Send Point
        self.send("\r\n.\r\n".to_string())?;

        let _message_ok = self.read_lines()?;
        self.send("QUIT\r\n".to_string())?;

        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    dotenv().ok();

    let connector = TlsConnector::new().unwrap();
    let stream = TcpStream::connect("smtp.gmail.com:465").unwrap();
    let stream = connector.connect("smtp.gmail.com", stream).unwrap();
    println!("Connected to the SMTP server!");

    let mut client = SMTPClient{stream};

    client.send_message(
        "smtp.gmail.com".to_string(), 
        dotenv!("TO").to_string(),
        dotenv!("TO_NAME").to_string(), 
        dotenv!("FROM").to_string(), 
        dotenv!("FROM_NAME").to_string(), 
        "Test Rust Client".to_string(), 
        "It's working".to_string(),
        dotenv!("FROM").to_string(),
        dotenv!("PASSWORD").to_string()
    )?;

    client.stream.shutdown().expect("shutdown call failed");
    println!("Disconnected from the SMTP server!");

    Ok(())
}
