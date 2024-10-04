const INPUT_BYTES: &[u8] = b"\xce\xaa\xca\xb2\xc3\xb4\xd2\xaa\xd1\xdd\xd7\xe0\xb4\xba\xc8\xd5\xd3\xb0\xa3\xbf\xa3\xa1\xa3\xa1";

fn main() {
    let res = iconv_native::decode_lossy(INPUT_BYTES, "gb18030").unwrap();
    println!("{res}");
}
