use byteorder::{LittleEndian, ReadBytesExt};
use config::{Config, DataType};
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};

pub fn print_values(mut f: File, config: Config) -> std::io::Result<()> {
    f.seek(SeekFrom::Start(config.offset))?;

    let mut reader = BufReader::new(f);

    for _ in 0..config.count {
        print_single_value(&mut reader, config.datatype)?;
        for _ in 1..config.stride {
            print!(",");
            print_single_value(&mut reader, config.datatype)?;
        }
        println!("");
    }

    return Ok(());
}

fn print_single_value(reader: &mut Read, dt: DataType) -> std::io::Result<()> {
    match dt {
        DataType::Float => {
            let x = reader.read_f32::<LittleEndian>()?;
            print!("{}", x);
        }
        DataType::Double => {
            let x = reader.read_f64::<LittleEndian>()?;
            print!("{}", x);
        }
        DataType::UInt8 => {
            let x = reader.read_u8()?;
            print!("{}", x);
        }
        DataType::UInt16 => {
            let x = reader.read_u16::<LittleEndian>()?;
            print!("{}", x);
        }
        DataType::UInt32 => {
            let x = reader.read_u32::<LittleEndian>()?;
            print!("{}", x);
        }
    }

    return Ok(());
}
