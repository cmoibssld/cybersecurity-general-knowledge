use inquire::Password;

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
