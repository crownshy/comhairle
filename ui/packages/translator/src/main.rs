use translator::{files, languages};

fn main() -> Result<(), Box<dyn core::error::Error>> {
    // let main_language = languages::get_main_language();
    // let file = files::read(&main_language)?;

    for language in files::languages()? {
        let language = match language {
            Ok(l) => l,
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        };

        let Some(language) = language else {
            eprintln!("Could not find file name");
            continue;
        };

        let language = match language.into_string() {
            Ok(l) => l,
            Err(e) => {
                eprintln!("{e:?}");
                continue;
            }
        };

        println!("{language}");
    }

    // for key in file.keys() {
    //     println!("{}: {}", key, file[key]);
    // }

    Ok(())
}
