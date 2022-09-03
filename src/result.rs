#![deny(clippy::all)]

fn get_first_name() -> Result<String, String> {
    // Ok("John".to_string())
    // Err(())
    Err("I don't know firstname".to_owned())
}
fn get_last_name() -> Result<String, String> {
    // Ok("Doe".to_string())
    Err("I don't know lastname".to_owned())
}
fn get_full_name() -> Result<String, String> {
    let last_name = get_last_name()?;
    let first_name = get_first_name()?;
    Ok(format!("{} {}", first_name, last_name))
}

fn main() {
    let value = get_full_name();
    match value.clone() {
        Ok(name) => println!("{}", name),
        Err(err) => println!("Err {}", err),
    }
    println!("{}", value.is_ok());
    let len = value.clone().map(|s| s.len()).unwrap_or_default();
    println!("{}", len);
    let err = value.map_err(|e| format!("Err: {}", e)).unwrap_err();
    println!("{:?}", err);
}
