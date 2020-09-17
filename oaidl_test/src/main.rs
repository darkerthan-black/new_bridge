extern crate oaidl;
extern crate winapi;

use std::vec::IntoIter;
use oaidl::{SafeArrayElement, SafeArrayExt, SafeArrayError};

fn main() -> Result<(), SafeArrayError> {
    let v = vec![-3i16, -2, -1, 0, 1, 2, 3];
    let arr = v.into_iter().into_safearray()?;
    let out = IntoIter::<i16>::from_safearray(arr)?;
    println!("{:?}", out);
    Ok(())
}