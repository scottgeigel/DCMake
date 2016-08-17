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

extern crate getopts;
use getopts::Options;
use std::env;

mod dc_make;

const PROGRAM_NAME : &'static str = "derp";//env!("CARGO_PKG_NAME");
const PROGRAM_VERSION : &'static str = env!("CARGO_PKG_VERSION");

struct Variables{
    input_file_name : Option<String>,
    output_file_name : Option<String>,
}

fn print_version() {
    println!("{} version {}", PROGRAM_NAME, PROGRAM_VERSION);
}

fn print_usage(opts : Options) {
    print_version();
    let brief = "Usage: [options] [FILE]";
    println!("{}", opts.usage(&brief));
}

fn process_arguments(opts : Options, factory : &mut dc_make::DCMakeFactory, variables : &mut Variables) {
    let args : Vec<String> = env::args().collect();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(opts);
        std::process::exit(0);
    }

    if matches.opt_present("v") {
        print_version();
        std::process::exit(0);
    }

    if let Some(flag) = matches.opt_str("F") {
        factory.assign_dc_make_definition(flag);
    }

    if let Some(magic_phrase) = matches.opt_str("M") {
        factory.assign_magic_phrase(magic_phrase);
    }

    for define in matches.opt_strs("D") {
        factory.add_definition(define);
    }

    for include in matches.opt_strs("i") {
        factory.add_included_file(include);
    }
}

fn main() {
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "version", "print the version of this program");
    opts.optopt("F", "flag", "define the #define flag to signify that this source code was processed by this program. The default is #define DYNAMICC_MODE", "DCMAKE_FLAG");
    opts.optopt("M", "magic", "specify the magic phrase used to include a file. Defualt is \"require\". It is not recommended to use \"include\" if your program is meant to compile with a C compiler too.", "MAGIC");
    opts.optmulti("D", "define", "add definition in the form of a #define at the beginning of the program", "DEFINITION");
    opts.optmulti("i", "include", "sepcify a file to be included at the beginning unconditionally. This will occur after the -D and -F arguments", "FILE");

    let mut factory = dc_make::DCMakeFactory::new();
    let mut variables = Variables {
        input_file_name : None,
        output_file_name : None,
    };

    process_arguments(opts, &mut factory, &mut variables);
    let mut file = std::io::stdin();
    factory.finalize().make(&mut file.lock());
}
