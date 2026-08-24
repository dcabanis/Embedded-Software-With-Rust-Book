#![no_std]
#![no_main]
#![deny(improper_ctypes)]

use core::ffi::c_char;
use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;

// The generated bindings, pulled in from `OUT_DIR` (10.5.2).
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod jsmn {
    include!(concat!(env!("OUT_DIR"), "/jsmn_bindings.rs"));
}

const JSON: &[u8] = br#"{"id":42,"name":"blue-pill","active":true}"#;
const MAX_TOKENS: usize = 16;

// `bindgen`'s default output represents a plain C `enum` as a type
// alias plus loose constants, not a Rust `enum` so this is a manual
// match against those constants rather than an exhaustive enum match.
fn token_type_name(t: jsmn::jsmntype_t) -> &'static str {
    match t {
        jsmn::jsmntype_t_JSMN_OBJECT => "OBJECT",
        jsmn::jsmntype_t_JSMN_ARRAY => "ARRAY",
        jsmn::jsmntype_t_JSMN_STRING => "STRING",
        jsmn::jsmntype_t_JSMN_PRIMITIVE => "PRIMITIVE",
        _ => "UNDEFINED",
    }
}

#[entry]
fn main() -> ! {
    // SAFETY: jsmn_parser and jsmntok_t are plain structs of integers;
    // the all-zero bit pattern is a valid value for every field, and
    // jsmn_init() overwrites it properly before first use.
    let mut parser: jsmn::jsmn_parser = unsafe { core::mem::zeroed() };
    unsafe { jsmn::jsmn_init(&mut parser) };

    let mut tokens: [jsmn::jsmntok_t; MAX_TOKENS] = unsafe { core::mem::zeroed() };

    let result = unsafe {
        jsmn::jsmn_parse(
            &mut parser,
            JSON.as_ptr() as *const c_char,
            JSON.len(),
            tokens.as_mut_ptr(),
            MAX_TOKENS as u32,
        )
    };

    if result < 0 {
        defmt::error!("jsmn_parse failed with error code {}", result);
    } else {
        let token_count = result as usize;
        defmt::info!(
            "parsed {} tokens from the embedded JSON string",
            token_count
        );
        for (i, tok) in tokens[..token_count].iter().enumerate() {
            defmt::info!(
                "  token[{}]: type={} start={} end={}",
                i,
                token_type_name(tok.type_),
                tok.start,
                tok.end
            );
        }
    }

    loop {
        cortex_m::asm::nop();
    }
}
