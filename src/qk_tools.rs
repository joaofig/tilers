

pub fn qk_char_to_i64(c: char) -> i64 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        _ => panic!("Invalid quadkey character"),
    }
}

pub fn qk_digit_to_str(digit: i32) -> &'static str {
    match digit % 4 {
        0 => "0",
        1 => "1",
        2 => "2",
        3 => "3",
        _ => panic!("Invalid quadkey digit")
    }
}

pub fn qk_str_to_i64(qk_str: &str) -> i64 {
    let mut qk_i: i64 = 0i64;

    for c in qk_str.chars() {
        qk_i = qk_i * 4 + qk_char_to_i64(c)
    }
    qk_i
}

pub fn qk_i64_to_str(qk_i64: i64, zoom: i32) -> String {
    let mut qk_str = String::from("");
    let mut qk: i64 = qk_i64;
    for _i in 0..zoom {
        qk_str.insert_str(0, qk_digit_to_str((qk & 3) as i32));
        qk >>= 2;
    }
    qk_str
}
