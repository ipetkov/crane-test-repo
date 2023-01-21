use std::{
    any::TypeId,
    env, fs,
    io::{self, Write},
    path::Path,
};

fn main() -> io::Result<()> {
    let data = format!("some data: {:?}", TypeId::of::<byteorder::NetworkEndian>());

    let out_dir = env::var("OUT_DIR").unwrap();
    fs::create_dir_all(&out_dir)?;
    fs::File::create(Path::new(&out_dir).join("mydata"))?.write_all(data.as_bytes())?;

    Ok(())
}
