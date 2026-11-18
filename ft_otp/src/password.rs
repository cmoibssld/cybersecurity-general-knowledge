use argon2::Argon2;
use inquire::Password;
use rand::RngExt;

// If I had time this is where I can ask for a more robust password
pub fn retreive_password() -> String {
    loop {
        let name = Password::new("Password for encrypted key")
            .with_display_toggle_enabled()
            .prompt();
        match name {
            Ok(password) => return password,
            Err(_) => eprintln!("Nope, try again"),
        }
    }
}

pub fn passe_moi_le_sel() -> String {
    let mut rng = rand::rng();

    (0..16) // salt of 16 bytes
        .map(|_| {
            let char: u8 = rng.random();
            char as char
        })
        .collect()
}

pub fn built_aes_key(password: String, salt: &str) -> Option<String> {
    let mut output: Vec<u8> = Vec::with_capacity(32);
    let argon = Argon2::default();

    argon.hash_password_into(password.as_bytes(), salt.as_bytes(), &mut output);
    String::from_utf8(output).ok()
}
