fn unsigned_short(first: u8, second: u8) -> u16 {
    let mut result: u16 = first as u16;
    result = result << 8;
    result += second as u16;
    result
}

fn signed_short(first: u8, second: u8) -> i16 {
    let result: i16 = unsigned_short(first, second) as i16;
    result
}

fn unsigned_int(first: u8, second: u8, third: u8, fourth: u8) -> u32 {
    let mut result: u32 = first as u32;
    result = result << 8;
    result += second as u32;
    result = result << 8;
    result += third as u32;
    result = result << 8;
    result += fourth as u32;
    result
}

fn signed_int(first: u8, second: u8, third: u8, fourth: u8) -> i32 {
    let result: i32 = unsigned_int(first, second, third, fourth) as i32;
    result
}

fn unsigned_long(first: u8, second: u8, third: u8, fourth: u8, fifth: u8, sixth: u8, seventh: u8, eighth: u8) -> u64 {
    let mut result: u64 = first as u64;
    result = result << 8;
    result += second as u64;
    result = result << 8;
    result += third as u64;
    result = result << 8;
    result += fourth as u64;
    result = result << 8;
    result += fifth as u64;
    result = result << 8;
    result += sixth as u64;
    result = result << 8;
    result += seventh as u64;
    result = result << 8;
    result += eighth as u64;
    result
}

fn signed_long(first: u8, second: u8, third: u8, fourth: u8, fifth: u8, sixth: u8, seventh: u8, eighth: u8) -> i64 {
    let result: i64 = unsigned_long(first, second, third, fourth, fifth, sixth, seventh, eighth) as i64;
    result
}

#[derive(Debug)]
pub enum NbtType {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32), // TODO verify compatibility with rust
    Double(f64), // Here too
    ByteArray(Vec<i8>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
    String(String),
    List(Vec<NbtElement>),
    Compound(Vec<NbtElement>),
    None
}

#[derive(Debug)]
pub struct NbtElement {
    name: Option<String>,
    value: NbtType,
}

pub fn parse(data: &Vec<u8>, i: usize, tag_type: u8, named: bool) -> (NbtElement, usize) {
    let mut i = i;
    let mut element: NbtElement = NbtElement{
        name: None,
        value: NbtType::None,
    };
    match tag_type {
        1u8 => {
            if named {
                let name_lenght: u16 = unsigned_short(data[i], data[i+1]);
                i += 2;
                let mut name = String::new();
                let mut bytes_processed = 0;
                while bytes_processed < name_lenght as usize {
                    name.push(data[i] as char);
                    i += 1;
                    bytes_processed += 1;
                }
                element.name = Some(name);
            }
            element.value = NbtType::Byte(data[i] as i8);
            i += 1;
        },
        2u8 => {
            if named {
                let name_lenght: u16 = unsigned_short(data[i], data[i+1]);
                i += 2;
                let mut name = String::new();
                let mut bytes_processed = 0;
                while bytes_processed < name_lenght as usize {
                    name.push(data[i] as char);
                    i += 1;
                    bytes_processed += 1;
                }
                element.name = Some(name);
            }
            element.value = NbtType::Short(signed_short(data[i], data[i+1]));
            i += 2;
        },
        3u8 => {
            if named {
                let name_lenght: u16 = unsigned_short(data[i], data[i+1]);
                i += 2;
                let mut name = String::new();
                let mut bytes_processed = 0;
                while bytes_processed < name_lenght as usize {
                    name.push(data[i] as char);
                    i += 1;
                    bytes_processed += 1;
                }
                element.name = Some(name);
            }
            element.value = NbtType::Int(signed_int(data[i], data[i+1], data[i+2], data[i+3]));
            i += 4;
        },
        4u8 => {
            if named {
                let name_lenght: u16 = unsigned_short(data[i], data[i+1]);
                i += 2;
                let mut name = String::new();
                let mut bytes_processed = 0;
                while bytes_processed < name_lenght as usize {
                    name.push(data[i] as char);
                    i += 1;
                    bytes_processed += 1;
                }
                element.name = Some(name);
            }
            element.value = NbtType::Long(signed_long(data[i], data[i+1], data[i+2], data[i+3], data[i+4], data[i+5], data[i+6], data[i+7]));
            i += 8;
        },
        5u8 => {
            panic!("5 Unsupported yet! at {}", i);
        },
        6u8 => {
            panic!("6 Unsupported yet! at {}", i);
        },
        7u8 => {
            if named {
                let name_lenght: u16 = unsigned_short(data[i], data[i+1]);
                i += 2;
                let mut name = String::new();
                let mut bytes_processed = 0;
                while bytes_processed < name_lenght as usize {
                    name.push(data[i] as char);
                    i += 1;
                    bytes_processed += 1;
                }
                element.name = Some(name);
            }
            let lenght = signed_int(data[i], data[i+1], data[i+2], data[i+3]);
            i += 4;
            let mut values: Vec<i8> = Vec::new();
            while values.len() < lenght as usize {
                values.push(data[i] as i8);
                i += 1;
            }
            element.value = NbtType::ByteArray(values);
        },
        8u8 => {
            if named {
                let name_lenght: u16 = unsigned_short(data[i], data[i+1]);
                i += 2;
                let mut name = String::new();
                let mut bytes_processed = 0;
                while bytes_processed < name_lenght as usize {
                    name.push(data[i] as char);
                    i += 1;
                    bytes_processed += 1;
                }
                element.name = Some(name);
            }
            let lenght = unsigned_short(data[i], data[i+1]);
            i += 2;
            let mut value = String::new();
            let mut bytes_processed = 0;
            while bytes_processed < lenght as usize {
                value.push(data[i] as char);
                i += 1;
                bytes_processed += 1;
            }
            element.value = NbtType::String(value);
        },
        9u8 => {
            if named {
                let name_lenght: u16 = unsigned_short(data[i], data[i+1]);
                i += 2;
                let mut name = String::new();
                let mut bytes_processed = 0;
                while bytes_processed < name_lenght as usize {
                    name.push(data[i] as char);
                    i += 1;
                    bytes_processed += 1;
                }
                element.name = Some(name);
            }
            let tag_type = data[i];
            i += 1;
            let lenght = signed_int(data[i], data[i+1], data[i+2], data[i+3]);
            i += 4;
            let mut values: Vec<NbtElement> = Vec::new();
            let mut items_processed = 0;
            while items_processed < lenght {
                let (value, incremented_i) = parse(data, i, tag_type, false);
                values.push(value);
                i = incremented_i;
                items_processed += 1;
            }
            element.value = NbtType::List(values);
        },
        10u8 => {
            if named {
                let name_lenght: u16 = unsigned_short(data[i], data[i+1]);
                i += 2;
                let mut name = String::new();
                let mut bytes_processed = 0;
                while bytes_processed < name_lenght as usize {
                    name.push(data[i] as char);
                    i += 1;
                    bytes_processed += 1;
                }
                element.name = Some(name);
            }
            let mut values: Vec<NbtElement> = Vec::new();
            while data[i] != 0u8 {
                let (value, incremented_i) = parse(data, i+1, data[i], true);
                values.push(value);
                i = incremented_i;
            }
            i += 1;
            element.value = NbtType::Compound(values);
        },
        11u8 => {
            if named {
                let name_lenght: u16 = unsigned_short(data[i], data[i+1]);
                i += 2;
                let mut name = String::new();
                let mut bytes_processed = 0;
                while bytes_processed < name_lenght as usize {
                    name.push(data[i] as char);
                    i += 1;
                    bytes_processed += 1;
                }
                element.name = Some(name);
            }
            let lenght = signed_int(data[i], data[i+1], data[i+2], data[i+3]);
            i += 4;
            let mut values: Vec<i32> = Vec::new();
            while values.len() < lenght as usize {
                values.push(signed_int(data[i], data[i+1], data[i+2], data[i+3]));
                i += 4;
            }
            element.value = NbtType::IntArray(values);
        },
        12u8 => {
            if named {
                let name_lenght: u16 = unsigned_short(data[i], data[i+1]);
                i += 2;
                let mut name = String::new();
                let mut bytes_processed = 0;
                while bytes_processed < name_lenght as usize {
                    name.push(data[i] as char);
                    i += 1;
                    bytes_processed += 1;
                }
                element.name = Some(name);
            }
            let lenght = signed_int(data[i], data[i+1], data[i+2], data[i+3]);
            i += 4;
            let mut values: Vec<i64> = Vec::new();
            while values.len() < lenght as usize {
                values.push(signed_long(data[i], data[i+1], data[i+2], data[i+3], data[i+4], data[i+5], data[i+6], data[i+7]));
                i += 8;
            }
            element.value = NbtType::LongArray(values);
        },
        _ => {
            panic!("Unknow type {} at {}", tag_type, i);
        },
    }

    (element, i)
}