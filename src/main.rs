use std::fs;
use nom::AsChar;
//use nom::bytes::complete::{tag, take};
use nom::number::complete::{be_u8};
use nom::{IResult,Err,Needed,named,do_parse,tag};

pub struct Header {
    file_size1: u8,
    file_size2: u8,
    file_size3: u8,
    file_size4: u8,
}

named!(parse_sc2000(&[u8]) -> Header,
    do_parse!(
        tag!("FORM") >>
        file_size1: be_u8 >>
        file_size2: be_u8 >>
        file_size3: be_u8 >>
        file_size4: be_u8 >>
        (
            Header {
                file_size1: file_size1,
                file_size2: file_size2,
                file_size3: file_size3,
                file_size4: file_size4
            }
        )
    )
);

fn main() {
    let data = fs::read("/home/bkotos/Projects/sc2k-parser/assets/sbend.sc2").expect("Unable to read file");

//    parse_sc2000(&[data[0]]);
    let foo = parse_sc2000(&data);

    match foo {
        Ok(v) => {
            println!("All good {}", v.1.file_size1);
            println!("All good {}", v.1.file_size2);
            println!("All good {}", v.1.file_size3);
            println!("All good {}", v.1.file_size4);
        },
        Err(e) => println!("All bad")
    }

//    println!("{}", x.file_size);

//    let (input, _) = tag("FORM")(data);
//    println!("{}", input)

//    for x in data {
//        if x.is_alphanum() {
//            print!("{}", x.as_char());
//        } else {
//            print!(".")
//        }
//    }
}
