/*
 * Copyright 2022 E257.FI
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */
use std::env;
use std::path::Path;

use log::error;

use tackler_core::txns_file;
use tackler_api::metadata::TxnSetChecksum;
use tackler_api::metadata::Checksum;
use tackler_api::metadata::MetadataItem;


fn main() {
    let args: Vec<String> = env::args().collect();

    let txns_checksum =  TxnSetChecksum { 
        size: 1,
        hash: Checksum {
            algorithm: String::from("SHA-256"), 
            value: String::from("d8227ca355568dbc76235aa636d8797708eb7dbd9cef678a6093b470f517f8e8"),
        }, 
    };

    if args.len() != 2 {
        eprintln!("Error: Missing input file");
        eprintln!("Usage: {} <tackler txns-file>", &args[0]);
        std::process::exit(1);
    }
    let txn_file = Path::new(&args[1]);

    print!("Parsing {} ...", txn_file.display());
    match txns_file(txn_file) {
        Ok(parse_tree) => {
            println!("ok!");
            println!("{}", txns_checksum.text());
            println!("Parse tree is:\n{}\n", parse_tree);
            std::process::exit(0);
        }
        Err(err) => {
            let msg = format!("Error: {}", err);
            error!("{}", msg);
            eprintln!("{}", msg);
            std::process::exit(1);
        }
    }
}
