//! Shared native-runtime contract. The C header is generated from these codes
//! so native helpers and LLVM lowering cannot silently disagree about errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ErrorCode {
    Ok = 0,
    Eof = 1,
    InvalidInput = 2,
    OutOfRange = 3,
    Io = 4,
    InvalidUtf8 = 5,
    Nul = 6,
    Allocation = 7,
}

pub(crate) const FUNCTIONS: &[&str] = &[
    "cheeku_read_line",
    "cheeku_parse_int",
    "cheeku_parse_float",
    "cheeku_to_string_int",
    "cheeku_to_string_float",
    "cheeku_error_message",
];

pub fn c_source() -> String {
    let mut source = String::new();
    for (name, code) in [
        ("OK", ErrorCode::Ok),
        ("EOF", ErrorCode::Eof),
        ("INVALID_INPUT", ErrorCode::InvalidInput),
        ("OUT_OF_RANGE", ErrorCode::OutOfRange),
        ("IO", ErrorCode::Io),
        ("INVALID_UTF8", ErrorCode::InvalidUtf8),
        ("NUL", ErrorCode::Nul),
        ("ALLOCATION", ErrorCode::Allocation),
    ] {
        source.push_str(&format!("#define CHEEKU_{name} {}\n", code as i32));
    }
    source.push_str(include_str!("runtime/io.c"));
    source
}
