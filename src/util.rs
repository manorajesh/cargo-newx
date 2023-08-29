use std::{path::Path, fs};

pub fn is_empty_or_dir(path: &Path) -> std::io::Result<bool> {
    let metadata = fs::metadata(path)?;
    if metadata.is_dir() {
        let entries = fs::read_dir(path)?;
        Ok(entries.count() == 0)
    } else {
        Ok(false)
    }
}

pub fn cargotoml(name: String) -> Vec<String> {
    vec![
    "[package]".to_string(),
    format!("name = \"{}\"", name),
    "version = \"0.1.0\"".to_string(),
    "edition = \"2021\"".to_string(),
    "".to_string(),
    "# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html".to_string(),
    "".to_string(),
    "[dependencies]".to_string(),
    "".to_string(),
]
}

pub fn helloworld_rs() -> String {
r#"fn main() {
    println!("Hello, world!");
}"#.to_string()
}

pub fn mitlicense(names: &String, year: &String) -> String {
    format!(
r#"Copyright (c) {} {}

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE."#, year, names)
}

pub trait LogExpect<T> {
    fn log_expect(self, msg: &str) -> T;
}

impl<T> LogExpect<T> for Option<T> {
    fn log_expect(self, msg: &str) -> T {
        match self {
            Some(val) => val,
            None => {
                log::error!("{}", msg);
                std::process::exit(1);
            }
        }
    }
}

impl<T, E> LogExpect<T> for Result<T, E> 
where E: std::fmt::Display
{
    fn log_expect(self, msg: &str) -> T {
        match self {
            Ok(val) => val,
            Err(e) => {
                if msg.is_empty() {
                    log::error!("{}", e);
                } else {
                    log::error!("{}", msg);
                    log::error!("{}", e);
                }
                
                std::process::exit(1);
            }
        }
    }
}