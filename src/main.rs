use pak::Pak;

mod pak;

fn main() -> Result<(), std::io::Error> {
    let pak = Pak::new("id1/PAK0.PAK")?;
    let header = Pak::read_header(&pak)?;
    let files = Pak::read_directory(&pak)?;
    println!("{:?}", files);
    Ok(())
}
