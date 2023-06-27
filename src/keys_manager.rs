use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use tfhe::integer::{gen_keys_radix, RadixClientKey, ServerKey};
use tfhe::shortint::parameters::PARAM_MESSAGE_2_CARRY_2;

// Import the RadixClientKey and ServerKey types from the appropriate crate.
// The import path may vary depending on your actual codebase.

const CLIENT_KEY_FILE_PATH: &'static str = "client_key.bin";
const SERVER_KEY_FILE_PATH: &'static str = "server_key.bin";

pub(crate) fn key_gen(num_block: u32) -> Result<(RadixClientKey, ServerKey), Box<dyn Error + Send + Sync>> {
    let client_key_path = Path::new(CLIENT_KEY_FILE_PATH);
    let server_key_path = Path::new(SERVER_KEY_FILE_PATH);

    if client_key_path.exists() && server_key_path.exists() {
        let client_key: RadixClientKey = {
            let file = File::open(client_key_path)
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
            let reader = BufReader::new(file);
            bincode::deserialize_from(reader)
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?
        };

        let server_key: ServerKey = {
            let file = File::open(server_key_path)
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
            let reader = BufReader::new(file);
            bincode::deserialize_from(reader)
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?
        };

        Ok((client_key, server_key))
    } else {
        let now = std::time::Instant::now();
        let (client_key, server_key) = gen_keys_radix(&PARAM_MESSAGE_2_CARRY_2, num_block as usize);
        println!("Key generation: {}ms", now.elapsed().as_millis());

        {
            let client_file = File::create(client_key_path)
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
            let writer = BufWriter::new(client_file);
            bincode::serialize_into(writer, &client_key)
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
        }

        {
            let server_file = File::create(server_key_path)
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
            let writer = BufWriter::new(server_file);
            bincode::serialize_into(writer, &server_key)
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
        }

        Ok((client_key, server_key))
    }
}
