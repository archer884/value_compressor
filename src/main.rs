#![feature(box_syntax)]

extern crate bzip2;
extern crate rand;

use bzip2::Compress;
use bzip2::reader::BzCompressor;
use rand::distributions::{IndependentSample, Range};
use std::fs::File;
use std::io::{BufWriter, Cursor, Write};

fn main() {
    let args: Vec<_> = std::env::args().collect();

    match &args[..] {
        [_, ref input1, ref input2] => {
            let count = match input1.parse() {
                Ok(n) => n,
                _ => {
                    println!("Could not parse NUMBER");
                    return;
                }
            };

            let compress = match input2.parse() {
                Ok(c) => c,
                _ => {
                    println!("Could not parse BOOL");
                    return;
                }
            };

            match File::create(&Path::new(if compress { "./compress.out" } else { "./uncompress.out" })) {
                Ok(f) => {
                    write_output(count, compress, f);
                    println!("Done!");
                },
                Err(e) => {
                    println!("error: {}", e.description());
                    return;
                }
            }
        },
        _ => {
            println!("USAGE: {:?} NUMBER <number of values> COMPRESS <bool: compress output>", args[0]);
            return;
        }
    }
}

fn write_output(count: usize, compress: bool, file: File) {
    let range = Range::new(1, 1000000001);
    let mut writer = std::io::BufWriter::new(file);

    if compress {
        let values_n: Vec<_> = (0..count).map(|_| next_in_range(&range)).collect();
        let bytes: Vec<_> = (0..count)
            .flat_map(|idx| unsafe { raw_bytes(&values_n[idx]).iter().map(|byte| byte.clone()) })
            .collect();

        std::io::copy(&mut BzCompressor::new(Cursor::new(bytes), Compress::Best), &mut writer);
    } else {
        for _ in 0..count {
            unsafe { writer.write_all(raw_bytes(&next_in_range(&range))); }
        }
    }
}

fn next_in_range<T>(range: &Range<T>) -> T 
    where T: PartialOrd + rand::distributions::range::SampleRange
{
    range.ind_sample(&mut rand::thread_rng())
}

unsafe fn raw_bytes<'a, T>(ptr: &'a T) -> &'a [u8] {
    std::mem::transmute(std::raw::Slice {
        data: ptr as *const _ as *const u8,
        len: std::mem::size_of::<T>(),
    })
}
