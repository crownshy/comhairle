use translator::languages;

fn main() -> Result<(), Box<dyn core::error::Error>> {
    let main_language = languages::get_main_language();
    let file = languages::get_file(&main_language)?;

    for key in file.keys() {
        println!("{}: {}", key, file[key]);
    }

    Ok(())
}
