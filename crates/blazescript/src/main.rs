#![allow(unused_must_use)]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use bincode::{deserialize, serialize};
use blaze_vm::VM;
use bzs_shared::ByteCode;
use bzsc_bytecode::ByteCodeGen;
use bzsc_lexer::Lexer;
use bzsc_parser::Parser;
use std::env::args;
use std::process::exit;
use std::time::SystemTime;

fn main() {
    let file_name = args().nth(1).expect("no path specified");
    let time = SystemTime::now();

    if file_name.ends_with(".bzs") {
        println!("----Blazescript compiler----");
        println!("Version: 0.0.1");
        println!("File: {}", file_name);
        let cnt = std::fs::read_to_string(file_name.clone()).expect("could not read script");

        let name = Box::leak(file_name.to_owned().into_boxed_str());
        let content = Box::leak(cnt.to_owned().into_boxed_str());
        let lexed = Lexer::new(name, content).lex();
        let mut tokens = vec![];
        match lexed {
            Ok(lexed) => {
                tokens.extend(lexed);
            }
            Err(error) => {
                println!("{}", error.prettify());
                exit(1);
            }
        }

        let parsed = Parser::new(tokens).parse();
        if parsed.error.is_some() || parsed.node.is_none() {
            println!("{}", parsed.error.unwrap().prettify());
            exit(1);
        }

        let mut bytecode_gen = ByteCodeGen::new();
        bytecode_gen.compile_node(parsed.node.unwrap());
        let serialized =
            serialize(&bytecode_gen.bytecode).expect("serialization of bytecode failed");
        std::fs::write(file_name.clone().replace(".bzs", ".bze"), serialized);
        println!(
            "Compilation Success: Wrote to {}",
            file_name.clone().replace(".bzs", ".bze")
        );
        match time.elapsed() {
            Ok(elapsed) => {
                println!(
                    "Time taken for Compilation Process: {} milliseconds",
                    elapsed.as_millis()
                );
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
        exit(0);
    } else if file_name.ends_with(".bze") {
        println!("----Blaze Virtual Machine----");
        println!("Version: 0.0.1");
        println!("File: {}", file_name);
        let btc_raw = std::fs::read(file_name.clone()).expect("could not read executable");
        let bytecode: ByteCode =
            deserialize(&btc_raw[..]).expect("deserialization of executable failed");
        let mut vm = VM::new(bytecode, None);
        vm.run();
        println!("Result: {:?}", vm.pop_last());
        match time.elapsed() {
            Ok(elapsed) => {
                println!(
                    "Time taken for Interpretation Process: {} milliseconds",
                    elapsed.as_millis()
                );
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
        exit(0)
    } else {
        eprintln!("Error: File name should end with .bzs(Script) or .bze(Executable)");
        exit(1);
    };
}