use pak::Pak;

mod pak;

fn main() -> Result<(), std::io::Error> {
    let pak = Pak::new("id1/PAK0.PAK")?;
    let header = Pak::read_header(&pak)?;
    println!(
        "Pak type : {}\nDirectory offset : {}\nDirectory size : {}",
        header.id, header.dir_offset, header.dir_size
    );
    Ok(())
}
