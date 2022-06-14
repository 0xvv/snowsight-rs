use anyhow::anyhow;
use chacha20poly1305::aead::{Aead, NewAead};
use chacha20poly1305::XChaCha20Poly1305;
use std::fs::File;
use std::io::Write;
use std::{fs, io};

pub(crate) fn read_from_home(path: std::path::PathBuf) -> Vec<u8> {
    let home = match home::home_dir() {
        Some(path) => (path),
        None => panic!("Impossible to get your home dir!"),
    };
    fs::read(home.join(path)).expect("cannot read file")
}

fn write_to_home(data: Vec<u8>, path: std::path::PathBuf) -> io::Result<()> {
    let home = match home::home_dir() {
        Some(path) => (path),
        None => panic!("Impossible to get your home dir!"),
    };
    let mut file = File::create(home.join(path))?;
    file.write_all(data.as_slice())?;
    Ok(())
}

pub fn encrypt(
    content: &str,
    dist: &str,
    key: &[u8; 32],
    nonce: &[u8; 24],
) -> Result<(), anyhow::Error> {
    let cipher = XChaCha20Poly1305::new(key.into());

    let encrypted_file = cipher
        .encrypt(nonce.into(), content.as_ref())
        .map_err(|err| anyhow!("Encrypting small file: {}", err))?;
    write_to_home(encrypted_file, dist.into())?;
    Ok(())
}

pub fn decrypt(
    encrypted_file_path: &str,
    key: &[u8; 32],
    nonce: &[u8; 24],
) -> Result<Vec<u8>, anyhow::Error> {
    let cipher = XChaCha20Poly1305::new(key.into());
    let file_data = read_from_home(encrypted_file_path.into());
    let decrypted_file = cipher
        .decrypt(nonce.into(), file_data.as_ref())
        .map_err(|err| anyhow!("Decrypting small file: {}", err))?;
    Ok(decrypted_file)
}
