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
use std::io::BufReader;
use std::process;
use std::fs::File;

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

fn start_comment_block(name : &str) -> String {
    const comment_80 : &'static str = "////////////////////////////////////////////////////////////////////////////////\n";
    let mut ret = String::from(comment_80);
    let pad_size_left = (40 - (name.len()/2));
    let pad_size_right = pad_size_left + (name.len() % 2);
    ret.push_str(&comment_80[0..pad_size_left]);
    ret.push_str(&name);
    ret.push_str(&comment_80[0..pad_size_right]);
    return ret;
}


fn end_comment_block(name : &str) -> String {
    const comment_80 : &'static str = "\n////////////////////////////////////////////////////////////////////////////////";
    let mut ret = String::new();
    let pad_size_left = (40 - (name.len()/2));
    let pad_size_right = pad_size_left + (name.len() % 2);
    ret.push_str(&comment_80[1..pad_size_left]);
    ret.push_str(&name);
    ret.push_str(&comment_80[1..pad_size_right]);
    ret.push_str(&comment_80);
    return ret;
}

impl DCMake {
    fn new () -> DCMake {
        DCMake {
            magic_phrase : String::from("#require"),
            dc_make_definition : String::from("DYNAMICC_MODE"),
            pre_defines : HashSet::new(),
            pre_included_paths : HashSet::new(),

            included_paths : HashSet::new(),
        }
    }

    fn process_file(&mut self, path : String) -> Result<(), String> {
        ///takes the path included by #[magic_phrase][<|(]path_name[>|)]
        impl DCMake {
            fn extract_file_path(&mut self, directive : &str) -> Result<String, String> {
                //strip the #[magic_phrase]
                let captured_path : Vec<&str> = directive.split(self.magic_phrase.as_str()).collect();
                let mut captured_path = captured_path[1].trim().to_string();
                //remove any surrounding whitespace and get the substring between the surrounding "" or <>
                //first check for ""
                let captured_path = {
                    if captured_path.remove(0) != '\"' {
                        return Err(format!("Malfored expression {}. Must be enclosed with \"\" characters", directive))
                    }
                    let len = captured_path.len();
                    if captured_path.remove(len - 1) != '\"' {
                        return Err(format!("Malfored expression {}. Missing closing \" character", directive))
                    } else {
                        captured_path
                    }
                };
                Ok(captured_path)
            }
        }
        //get a handle to the file
        match File::open(&path.as_str()) {
            Ok(file_handle) => {
                let mut buffer = String::new();
                let mut buffered_reader = BufReader::new(file_handle);
                //check to see if we have read this file already
                if !self.included_paths.insert(path.clone()) {
                    writeln!(&mut io::stderr(), "Skipping {}", path);
                } else {
                    //begin file processing
                    println!("{}", start_comment_block(&path.as_str()));
                    let mut line_number : usize = 0;
                    while let Ok(bytes_read) = buffered_reader.read_line(&mut buffer) {
                        line_number += 1;
                        if bytes_read > 0 {
                            //check to see if we have hit an include directive
                            if buffer.starts_with(self.magic_phrase.as_str()) {
                                let extracted_result = self.extract_file_path(buffer.as_str());
                                if let Err(error_message) = extracted_result {
                                    return Err(error_message);
                                }
                                let new_file = extracted_result.unwrap();
                                //begin processing this file and check for errors
                                if let Err(error_message) = self.process_file(new_file) {
                                    //log this error
                                    writeln!(&mut io::stderr(), "{}", error_message).unwrap();
                                    //pass on the error for stack trace
                                    return Err(format!("In file included from {}:{}", path, line_number));
                                }
                            } else {
                                print!("{}", buffer);
                            }
                            buffer.clear();
                        } else {
                            //we are at the end of the file
                            break;
                        }
                    }
                    //end file processing
                    println!("{}", end_comment_block(&path.as_str()));
                }
                Ok(())
            },
            Err(e) => {
                return Err(format!("Failed to open {}\n{}", path, e));
            }
        }
    }

    pub fn make(&mut self, source : &mut BufRead) {
        //DEBUG INFORMATION
        {
            use std::fs;
            writeln!(io::stderr(), "magic phrase is {}", self.magic_phrase).unwrap();
            let paths = fs::read_dir("./").unwrap();
            writeln!(io::stderr(), "options are:").unwrap();
            for path in paths {
                writeln!(io::stderr(), "{}", path.unwrap().path().display()).unwrap();
            }
        }
        //END DEBUG INFORMATION
        let mut buffer = String::new();
        while let Ok(bytes_read) = source.read_line(&mut buffer) {
            if bytes_read > 0 {
                //DEBUG INFORMATION
                writeln!(io::stderr(), "{} {}", buffer, bytes_read).unwrap();
                //END DEBUG INFORMATION

                buffer = buffer.trim().to_string();

                if let Err(error_message) = self.process_file(buffer.clone()) {
                    writeln!(&mut io::stderr(), "{}\nERROR: failed to process {}", error_message, buffer).unwrap();
                }
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
