mod keys_manager;

#[allow(unused_imports)]

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use tfhe::{ConfigBuilder, FheUint8, generate_keys};
use tfhe::prelude::*;
use crate::keys_manager::key_gen;


#[derive(Debug)]
enum FHEOperationType {
    Encrypt,
    Decrypt,
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8071")?;
    println!("Server is listening");

    // accept connections and process them serially
    for stream in listener.incoming() {
        println!("A client initiated connection");
        std::thread::spawn(move || handle_client(stream?));
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    // tfhe-rs for generating keys
    // Generate tfhe-rs client_key and server_key
    let (mut client_key, mut server_key) = key_gen().map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    })?;


    // prepare a buffer to hold the data
    let mut buffer = [0u8; 4];

    // read an int value from the stream
    stream.read_exact(&mut buffer)?;
    let value = i32::from_le_bytes(buffer);

    // map the int value to the corresponding FHEOperationType variant
    let operation = match value {
        0 => FHEOperationType::Encrypt,
        1 => FHEOperationType::Decrypt,
        _ => return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid operation type value",
        )),
    };

    // print the operation type
    println!("Received operation: {:?}", operation);

     match operation {
            FHEOperationType::Encrypt => {
                // read another int value
                stream.read_exact(&mut buffer)?;
                let num_values = i32::from_le_bytes(buffer);

                println!("Number of subsequent values: {}", num_values);

                // read each subsequent value
                for _ in 0..num_values {
                    stream.read_exact(&mut buffer)?;
                    let value = i32::from_le_bytes(buffer);
                    println!("Received value: {}", value);
                    //Encrypt values using client_key
                    let ciphertext = FheUint8::encrypt(value as u8, &client_key);
                }
            }
            FHEOperationType::Decrypt => {
                // do something for decryption
                println!("This is a Decrypt operation");
            }
        }

    // send a response back to the client
    let response = "Hello, client!";
    stream.write_all(response.as_bytes())?;
    Ok(())
}



