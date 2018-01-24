// Copyright © 2016-2018, Peter Atashian
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use std::result;
use winapi::shared::minwindef::DWORD;
use winapi::um::errhandlingapi::GetLastError;
#[derive(Clone, Copy, Debug)]
pub struct Error(DWORD);
impl Error {
    pub fn code(&self) -> u32 { self.0 }
}

pub type Result<T> = result::Result<T, Error>;

pub fn last_error<T>() -> Result<T> {
    Err(Error(unsafe { GetLastError() }))
}
