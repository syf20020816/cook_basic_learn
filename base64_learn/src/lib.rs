use base64::Engine;
/// # no pad
/// h -> ASCII -> 104 (10)
/// 104(10) -> 0110 1000(2) 8位
/// 011010 | 00 拆分6位为一段
/// 26 | 0
/// Base64表 -> aA ①
///
/// # use pad
/// ①
/// 补充操作：直接在结果中补充 `=`
/// aA==
///
/// 6 | 8 -> 24
/// 24 / 6 = 4
///
/// 011010|110110|110000
/// 26|54|48
/// a2w=
use base64::engine::general_purpose;
use std::str::from_utf8;
fn encode_decode() {
    //编码
    let origin_str = b"kl";
    println!("{:?}",origin_str);
    let encode_str = general_purpose::STANDARD.encode(origin_str);
    println!("{:?}",encode_str);
    //解码
    let decode_str = general_purpose::STANDARD_NO_PAD.decode("a2w").unwrap();
    let res = from_utf8(&decode_str).unwrap();
    dbg!(res);
}