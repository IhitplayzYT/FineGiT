use std::process::exit;

use crate::helper::Helper::{CLI, get_username, print_commit_history};

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






}
