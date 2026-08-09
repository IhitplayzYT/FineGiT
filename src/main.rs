use std::{fs::Metadata, process::exit};

use crate::helper::Helper::{CLI, f_meta, get_username, print_commit_history, process_flags};

mod helper;

fn main() {
    let mut clargs = CLI::new();
    let mut url = None;
    clargs.Parse_Args();
    if clargs.dbg{
        println!("{clargs:?}");
    }

    if clargs.graph{
        print_commit_history();
        exit(0);
    }

    if clargs.auto_repo_link{
        url = Some(&format!("https://github.com/{}/{}",get_username(),std::env::current_dir().unwrap().file_name().unwrap().to_str().unwrap().to_string())[..]);
    }

    if let Some(x) = clargs.local_commit_log{

    }

    let mut ignore_files = vec![];
    if clargs.nested_gitignore{
        ignore_files.append(&mut std::fs::read_to_string(".gitignore").unwrap().split("\n").collect());
    }

    let mut flags = 0_u8;
    for i in &clargs.conditions{
        if flags != 0{
            process_flags(flags,i);
        }
        if i.starts_with("-"){
        flags = 0;
        let iterator = i.chars().skip(1).enumerate();
            for (idx,j) in iterator{
                match j{
                    'E' => {flags |= 1 << 0;},
                    'N' => {flags |= 1 << 1;},
                    'F' => {flags |= 1 << 2;},
                    'D' => {flags |= 1 << 3;},
                    'C' => {flags |= 1 << 4;},
                    '!' => {flags |= 1 << 5;},
                    '=' => {flags |= 1 << 6;},
                    _ => {}
                }

            }
                
        
        }


    }







}
