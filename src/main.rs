use std::io::{Read, Write};
#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};

use crate::keys_manager::key_gen;

mod keys_manager;

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

            // send a response letting the client know that
            let response = "Hello, client! Your data is now being prepared, please wait...";
            stream.write_all(response.as_bytes())?;

            let num_block = 8; // 4*2 = 8 bits, 8*2 = 16 bits
            let (client_key, server_key) = key_gen(num_block).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

            stream.read_exact(&mut buffer)?;
            let gambling_percent = i32::from_le_bytes(buffer);
            println!("gambling_percent: {}", gambling_percent);

            stream.read_exact(&mut buffer)?;
            let overspending_score = i32::from_le_bytes(buffer);
            println!("overspending_score: {}", overspending_score);

            stream.read_exact(&mut buffer)?;
            let impulsive_buying_score = i32::from_le_bytes(buffer);
            println!("impulsive_buying_score: {}", impulsive_buying_score);

            stream.read_exact(&mut buffer)?;
            let mean_deposit_sum = i32::from_le_bytes(buffer);
            println!("mean_deposit_sum: {}", mean_deposit_sum);

            stream.read_exact(&mut buffer)?;
            let mean_reported_income = i32::from_le_bytes(buffer);
            println!("mean_reported_income: {}", mean_reported_income);

            stream.read_exact(&mut buffer)?;
            let no_months_deposited = i32::from_le_bytes(buffer);
            println!("no_months_deposited: {}", no_months_deposited);

            // Generate tfhe-rs client_key and server_key
            let encrypted_gambling_percent = client_key.encrypt(gambling_percent as u64);
            let encrypted_overspending_score = client_key.encrypt(overspending_score as u64);
            let encrypted_impulsive_buying_score = client_key.encrypt(impulsive_buying_score as u64);
            let encrypted_mean_deposit_sum = client_key.encrypt(mean_deposit_sum as u64);
            let encrypted_mean_reported_income = client_key.encrypt(mean_reported_income as u64);
            let encrypted_no_months_deposited = client_key.encrypt(no_months_deposited as u64);

            //send encrypted values to the client (one by one)
            println!("Sending encrypted values to client");


            // The key size is 109518176 bytes => in order to optimise performance, we will share the key
            // with the requester only once, at the beginning of the connection

                // let server_key_size = bincode::serialized_size(&server_key)
                //     .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
                // println!("Server key serialized size: {}", server_key_size);
                // bincode::serialize_into(&mut stream, &server_key_size).unwrap();
                // bincode::serialize_into(&mut stream, &server_key).unwrap();

            let bytes_size = bincode::serialized_size(&encrypted_gambling_percent)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
            println!("Gambling percent serialized size: {}", bytes_size);
            let gambling_percent_size_bytes = bytes_size.to_be_bytes();
            stream.write_all(&gambling_percent_size_bytes)?; //all the sizes are the same, so we can send only one

            bincode::serialize_into(&mut stream, &encrypted_gambling_percent).unwrap();
            bincode::serialize_into(&mut stream, &encrypted_overspending_score).unwrap();
            bincode::serialize_into(&mut stream, &encrypted_impulsive_buying_score).unwrap();
            bincode::serialize_into(&mut stream, &encrypted_mean_deposit_sum).unwrap();
            bincode::serialize_into(&mut stream, &encrypted_mean_reported_income).unwrap();
            bincode::serialize_into(&mut stream, &encrypted_no_months_deposited).unwrap();

        }
        FHEOperationType::Decrypt => {
            // do something for decryption
            println!("This is a Decrypt operation");
        }
    }

    Ok(())
}



