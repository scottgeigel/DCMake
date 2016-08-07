//MIT License
//
//Copyright (c) 2016 Scott Geigel
//
//Permission is hereby granted, free of charge, to any person obtaining a copy
//of this software and associated documentation files (the "Software"), to deal
//in the Software without restriction, including without limitation the rights
//to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//copies of the Software, and to permit persons to whom the Software is
//furnished to do so, subject to the following conditions:
//
//The above copyright notice and this permission notice shall be included in all
//copies or substantial portions of the Software.
//
//THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//SOFTWARE.
//Dynamic C is copyrighted by 2011 Digi International® Inc
//This software is a third pary work and its author is not affiliated with
//Digi International® Inc

use std::collections::HashSet;
use std::io;
use std::io::Write;
use std::io::BufRead;
use std::process;
pub struct DCMake {
    magic_phrase : String,
    dc_make_definition : String,
    pre_defines : HashSet<String>,
    pre_included_paths : HashSet<String>,

    included_paths : HashSet<String>
}

pub struct DCMakeFactory {
    product : DCMake,
}

impl DCMake {
    fn new () -> DCMake {
        DCMake {
            magic_phrase : String::from("require"),
            dc_make_definition : String::from("DYNAMICC_MODE"),
            pre_defines : HashSet::new(),
            pre_included_paths : HashSet::new(),

            included_paths : HashSet::new(),
        }
    }

    pub fn make(&mut self, source : &mut BufRead) {
        let mut buffer = String::new();
        while let Ok(bytes_read) = source.read_line(&mut buffer) {
            if bytes_read > 0 {
                println!("{}", buffer);
                buffer.clear();
            } else {
                return;
            }
        }
    }
}

impl DCMakeFactory {
    pub fn new () -> DCMakeFactory {
        DCMakeFactory {
            product : DCMake::new(),
        }
    }

    pub fn assign_magic_phrase(&mut self, magic_phrase : String) {
        if magic_phrase.len() > 0 || magic_phrase.starts_with("#") {
            writeln!(&mut io::stderr(), "Error: Ignoring invalid magic phrase {}", magic_phrase).unwrap();
            process::exit(1);
        } else {
            self.product.magic_phrase = String::from("#") + &magic_phrase.as_str();
        }
    }

    pub fn assign_dc_make_definition(&mut self, definition : String) {
        if definition.len() > 0 || definition.starts_with("#") {
            writeln!(&mut io::stderr(), "Error: Ignoring invalid DC Make Flag {}", definition).unwrap();
        } else {
            self.product.dc_make_definition = String::from("#") + &definition.as_str();
        }
    }

    pub fn add_definition (&mut self, definition : String) {
        if !self.product.pre_defines.insert(definition.clone()) {
            writeln!(&mut io::stderr(), "Warning: Ignoring duplicate definition {}", definition).unwrap();
        }
    }

    pub fn add_included_file(&mut self, file_path : String) {
        if !self.product.pre_defines.insert(file_path.clone()) {
            writeln!(&mut io::stderr(), "Warning: Ignoring duplicate file inclusion {}", file_path).unwrap();
        }
    }

    pub fn finalize(self) -> DCMake {
        self.product
    }
}
