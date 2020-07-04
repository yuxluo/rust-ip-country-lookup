extern crate csv;

use std::error::Error;
use std::fs::File;
use std::process;

struct IpRange {
    start_ip: u32,
    end_ip: u32,
    country_code: String
}

static mut IP_MAP:Vec<IpRange> = Vec::new();

fn run() -> Result<(), Box<Error>> {
    let file_path = "src/DB.CSV";
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        let start_ip: u32 = record.get(0).unwrap().parse().unwrap();
        let end_ip: u32 = record.get(1).unwrap().parse().unwrap();
        let country_code = record.get(2).unwrap().to_string();
        unsafe {
            IP_MAP.push(IpRange{start_ip, end_ip, country_code });
        }
    }Ok(())
}

fn binary_search(cli_ip :u32, head :usize, tail :usize) -> usize {
    unsafe {
        let mut mutable_head = head;
        let mut mutable_tail = tail;
        while mutable_tail >= mutable_head {
            let mutable_mid :usize = (&mutable_head + &mutable_tail) / 2;
            if cli_ip > IP_MAP[mutable_mid].end_ip {
                mutable_head = mutable_mid + 1;
            }
            else if cli_ip < IP_MAP[mutable_mid].start_ip  {
                mutable_tail = mutable_mid - 1;
            } else {
                return mutable_mid;
            }
        }
    }
    return usize::max_value();
}


fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
    unsafe  {
        let index = binary_search(2890442453, 0, IP_MAP.len() - 1);
        println!("{}", IP_MAP[index].country_code);
    }
}