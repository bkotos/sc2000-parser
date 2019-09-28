use std::fs;
use nom::AsChar;
use nom::number::complete::{be_u8,be_u64};
use nom::number::Endianness;
use nom::{IResult,Err,Needed,named,do_parse,tag,take,u32};

pub struct Header {
    total_bytes: u32,
    file_type: [char; 4] // TODO convert char array to string
}

named!(parse_sc2000(&[u8]) -> Header,
    do_parse!(
        tag!("FORM") >>
        total_bytes: u32!(Endianness::Big) >>
        file_type: take!(4) >>
        (
            Header {
                total_bytes: total_bytes,
                file_type: [file_type[0] as char, file_type[1] as char, file_type[2] as char, file_type[3] as char]
            }
        )
    )
);

fn main() {
    // TODO dynamically pull file location
    let bin: Vec<u8> = fs::read("/home/bkotos/Projects/sc2k-parser/assets/sbend.sc2").expect("Unable to read file");

    match parse_sc2000(&bin) {
        Ok(result) => {
            println!("File is {} bytes long", result.1.total_bytes);
            println!(
                "File type is {}{}{}{}",
                result.1.file_type[0],
                result.1.file_type[1],
                result.1.file_type[2],
                result.1.file_type[3]
            );
        },
        Err(e) => println!("Error parsing file")
    }
}
