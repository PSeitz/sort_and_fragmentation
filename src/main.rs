#![feature(test)]
extern crate test;

fn main() {
    println!("Hello, world!");
}

#[bench]
fn bench_sort_fragmented_largedata(b: &mut test::Bencher) {
    use std::fs;
    let foo: String = fs::read_to_string("compression_65k.txt").unwrap();
    let mut sort_data = vec![];
    for el in foo.split_whitespace() {
        let mut mybytes = el.as_bytes().to_vec();
        for _ in 0..100 {
            mybytes.extend_from_slice(el.as_bytes());
        }
        sort_data.push((el.to_string(), mybytes));
    }
    b.iter(|| sort_data.sort_by(|x, y| x.0.cmp(&y.0)))
}
#[bench]
fn bench_sort_largedata(b: &mut test::Bencher) {
    use std::fs;
    let foo: String = fs::read_to_string("compression_65k.txt").unwrap();
    let mut data = vec![];
    for el in foo.split_whitespace() {
        let mut mybytes = el.as_bytes().to_vec();
        for _ in 0..100 {
            mybytes.extend_from_slice(el.as_bytes());
        }
        data.push(mybytes);
    }
    data.reverse();
    let mut sort_data = vec![];
    for el in foo.split_whitespace() {
        sort_data.push((el.to_string(), data.pop()));
    }
    b.iter(|| sort_data.sort_by(|x, y| x.0.cmp(&y.0)))
}

#[bench]
fn bench_sort_fragmented(b: &mut test::Bencher) {
    use std::fs;
    let foo: String = fs::read_to_string("compression_65k.txt").unwrap();
    let mut sort_data = vec![];
    for el in foo.split_whitespace() {
        let mut mybytes = el.as_bytes().to_vec();
        sort_data.push((el.to_string(), mybytes));
    }
    b.iter(|| sort_data.sort_by(|x, y| x.0.cmp(&y.0)))
}
#[bench]
fn bench_sort(b: &mut test::Bencher) {
    use std::fs;
    let foo: String = fs::read_to_string("compression_65k.txt").unwrap();
    let mut data = vec![];
    for el in foo.split_whitespace() {
        let mybytes = el.as_bytes().to_vec();
        data.push(mybytes);
    }
    data.reverse();
    let mut sort_data = vec![];
    for el in foo.split_whitespace() {
        sort_data.push((el.to_string(), data.pop()));
    }
    b.iter(|| sort_data.sort_by(|x, y| x.0.cmp(&y.0)))
}
