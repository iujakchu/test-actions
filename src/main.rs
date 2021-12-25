fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let file = glob::glob("*")?;
    let _ = &file
        .into_iter()
        .map(|x| match x {
            Ok(path) => {
                println!("{:}", path.is_dir());
            }
            Err(msg) => {}
        })
        .collect::<()>();
    Ok(())
}
