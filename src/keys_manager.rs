use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use tfhe::{ClientKey, ConfigBuilder, ServerKey};

const CLIENT_KEY_FILE_PATH: &'static str = "client_key.bin";
const SERVER_KEY_FILE_PATH: &'static str = "server_key.bin";

pub fn key_gen() -> Result<(ClientKey, ServerKey), Box<dyn Error>> {
    let client_key_path = Path::new(CLIENT_KEY_FILE_PATH);

    let client_keys: ClientKey = if client_key_path.exists() {
        println!("Reading client keys from {}", CLIENT_KEY_FILE_PATH);
        let mut file = BufReader::new(File::open(client_key_path)?);
        bincode::deserialize_from(file)?
    } else {
        println!(
            "No {} found, generating new keys and saving them",
            CLIENT_KEY_FILE_PATH
        );
        let config = ConfigBuilder::all_disabled()
            .enable_default_uint8()
            .build();
        let k = ClientKey::generate(config);
        let file = BufWriter::new(File::create(client_key_path)?);
        bincode::serialize_into(file, &k)?;

        k
    };

    let server_key_path = Path::new(SERVER_KEY_FILE_PATH);
    let server_keys: ServerKey = if server_key_path.exists() {
        println!("Reading server keys from {}", CLIENT_KEY_FILE_PATH);
        let mut file = BufReader::new(File::open(server_key_path)?);
        bincode::deserialize_from(file).unwrap()
    } else {
        println!(
            "No {} found, generating new keys and saving them",
            SERVER_KEY_FILE_PATH
        );
        let k = client_keys.generate_server_key();
        let file = BufWriter::new(File::create(server_key_path)?);
        bincode::serialize_into(file, &k).unwrap();

        k
    };

    Ok((client_keys, server_keys))
}