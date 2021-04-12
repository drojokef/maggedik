use std::env;

pub mod library;

fn main() -> Result<(), library::ErrMessage> {
    let args: Vec<String> = env::args().collect();
    let maggedik: library::Maggedik = library::Maggedik::new(args)?;
    let output = Box::new(maggedik.process()?);
    println!("{}", output);
    Ok(())
}
