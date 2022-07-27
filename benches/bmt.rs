#![feature(test)]

extern crate test;

use std::{fs::File, io::Read};

use bmt::{chunk::ChunkOptions, file::ChunkedFile};
use test::Bencher;

fn setup_bos_chunk_file() -> (Vec<u8>, usize) {
    let mut f = File::open("./test-files/bos.pdf").unwrap();
    let mut payload = Vec::<u8>::new();

    f.read_to_end(&mut payload).ok();

    let file_length = payload.len();

    (payload, file_length)
}

#[bench]
fn big_file_bench(b: &mut Bencher) {
    let (payload, _file_length) = setup_bos_chunk_file();
    let chunked_file = ChunkedFile::new(payload.clone(), ChunkOptions::default());

    b.iter(|| chunked_file.bmt());
}
