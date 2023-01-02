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
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use antlr_rust::{BailErrorStrategy, InputStream};
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::token_factory::CommonTokenFactory;
use antlr_rust::tree::ParseTree;
use log::error;

use txn_antlr::txnlexer::TxnLexer;
use txn_antlr::txnparser::TxnParser;

pub mod txn_antlr;

fn txns_file(path: &Path) -> Result<String, Box<dyn Error>> {
    let f = File::open(path);

    let mut txn_file = match f {
        Ok(file) =>  file,
        Err(err) => {
            let msg = format!("Can not open file: [{}], with error \"{}\"", path.display(), err.to_string());
            return Err(msg.into())
        }
    };

    let mut txns_str = String::new();

    txn_file.read_to_string(&mut txns_str)?;

    let tf = CommonTokenFactory::default();

    let mut _lexer = TxnLexer::new_with_token_factory(InputStream::new(txns_str.as_str()), &tf);

    let token_source = CommonTokenStream::new(_lexer);
    let mut parser = TxnParser::<'_, _, BailErrorStrategy<'_, _>>::new(token_source);

    let result = parser.txns();
    match result {
        Ok(_) => {
            let res_str = result.unwrap().to_string_tree(&*parser);
            Ok(res_str)
        },
        Err(err) => {
            let msg = format!("ANTRL error: {}", err);
            Err(msg.into())
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

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
