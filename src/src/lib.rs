#[macro_use]
extern crate napi_derive;

use bcrypt::{hash_with_salt, Version, DEFAULT_COST};

static SALT: [u8; 16] = [
  0x14, 0x4b, 0x3d, 0x69, 0x1a, 0x7b, 0x4e, 0xcf, 0x39, 0xcf, 0x73, 0x5c, 0x7f, 0xa7,
  0xa7, 0x9c,
];

#[napi]
pub fn hash(s: String) -> String {
  hash_with_salt(&s, DEFAULT_COST, SALT).unwrap().format_for_version(Version::TwoB)
    .replace(".", "0")
    .replace("/", "1")
    .replace("$", "2")
}
