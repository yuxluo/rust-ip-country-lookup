extern crate csv;

use std::error::Error;
use std::fs::File;
use std::process;

pub struct IpRange {
    start_ip: u32,
    end_ip: u32,
    country_code: String
}

pub extern "C" fn rust_initialise_geo_ip_table(ptr: *mut Vec<IpRange>) {
    if let Err(err) = read_csv(ptr) {
        println!("{}", err);
        process::exit(1);
    }
}

fn read_csv(ptr: *mut Vec<IpRange>) -> Result<(), Box<dyn Error>> {
    let geo_ip_table = unsafe { &mut *ptr };
    *geo_ip_table = Vec::new();

    let file_path = "src/DB.CSV";
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        let start_ip: u32 = record.get(0).unwrap().parse().unwrap();
        let end_ip: u32 = record.get(1).unwrap().parse().unwrap();
        let country_code = record.get(2).unwrap().to_string();
        (*geo_ip_table).push(IpRange{start_ip, end_ip, country_code });
    }Ok(())
}

pub fn binary_search_geo_ip_table_for_index(ptr: *mut Vec<IpRange>, cli_ip :u32) -> usize {
    let geo_ip_table = unsafe { &mut *ptr };

    let mut mutable_head:usize = 0;
    let mut mutable_tail = (*geo_ip_table).len();
    while mutable_tail >= mutable_head {
        let mutable_mid :usize = (&mutable_head + &mutable_tail) / 2;
        if cli_ip > (*geo_ip_table)[mutable_mid].end_ip {
            mutable_head = mutable_mid + 1;
        }
        else if cli_ip < (*geo_ip_table)[mutable_mid].start_ip  {
            mutable_tail = mutable_mid - 1;
        } else {
            return mutable_mid;
        }
    }
    return usize::max_value();
}

fn main() {
    let mut ptr = Vec::new();
    if let Err(err) = read_csv(&mut ptr) {
        println!("{}", err);
        process::exit(1);
    }

    let index:usize = binary_search_geo_ip_table_for_index(&mut ptr, 16777472);
    println!("{}", ptr[index].country_code);
}