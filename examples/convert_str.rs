fn main() {
    let res = iconv_native::convert_lossy("芙宁娜", "utf-8", "utf-16").unwrap();
    println!("{res:x?}"); // [ff, fe, 99, 82, 81, 5b, 1c, 5a]
}
