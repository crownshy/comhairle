use translator::languages;

fn main() -> Result<(), Box<dyn core::error::Error>> {
    // let main_language = languages::get_main_language();
    // let file = files::read(&main_language)?;

    let language_maps = languages::get_language_maps()?;

    languages::write_language_maps(&language_maps);

    // for key in file.keys() {
    //     println!("{}: {}", key, file[key]);
    // }

    Ok(())
}
