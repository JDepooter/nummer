use clap::ArgMatches;

#[derive(Clone,Copy)]
pub enum DataType {
    Float,
    Double,
    UInt8,
    UInt16,
    UInt32,
}

pub struct Config {
    pub offset: u64,
    pub count: u64,
    pub stride: u64,
    pub datatype: DataType,
}

impl Config {
    fn new(offset: u64, count: u64, stride: u64, dt: DataType) -> Config {
        Config {
            offset: offset,
            count: count,
            stride: stride,
            datatype: dt,
        }
    }
}

fn get_u64_value(matches: &ArgMatches, name: &str) -> Result<u64, ()> {
    let s = matches.value_of(name).ok_or(())?;
    return s.parse::<u64>().map_err(|_| ());
}

fn get_datatype(matches: &ArgMatches) -> Result<DataType, ()> {
    let s = matches.value_of("data type").ok_or(())?;
    match s {
        "float" => Ok(DataType::Float),
        "float32" => Ok(DataType::Float),
        "double" => Ok(DataType::Double),
        "float64" => Ok(DataType::Double),
        "uint8" => Ok(DataType::UInt8),
        "uint16" => Ok(DataType::UInt16),
        "uint32" => Ok(DataType::UInt32),
        _ => Err(()),
    }
}

pub fn get_config(matches: &ArgMatches) -> Result<Config, ()> {
    let offset = get_u64_value(&matches, "offset")?;
    let count = get_u64_value(&matches, "count")?;
    let stride = get_u64_value(&matches, "stride")?;

    let datatype = get_datatype(&matches)?;

    return Ok(Config::new(offset, count, stride, datatype));
}
