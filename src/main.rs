use std::{collections::HashSet, fs::{self, Metadata}, path::Path, process::{Command, exit}};

use crate::helper::Helper::{CLI, f_meta, get_username, print_commit_history, process_flags, process_flags_cond};

mod helper;

fn main() {
    let mut clargs = CLI::new();
    let mut url = None;
    clargs.Parse_Args();
    if clargs.dbg{
        println!("{clargs:?}");
    }

    let mut Git_Params:Vec<&str>;
    if clargs.params.contains("."){
        Git_Params = clargs.params.iter().filter_map(|x| {if !x.starts_with("-"){Some(&x[..])}else{None}}).collect();
    }else{
        Git_Params = vec!["add"];
    }

    if clargs.graph{
        print_commit_history();
        exit(0);
    }

    if clargs.auto_repo_link{
        url = Some(format!("https://github.com/{}/{}",get_username(),std::env::current_dir().unwrap().file_name().unwrap().to_str().unwrap().to_string()));
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
        *f_meta.try_write().unwrap() = None;

        let iterator = i.chars().skip(1);
            for j in iterator{
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

    flags = 0;
    for (i,v) in &clargs.condition_params{

    if i.starts_with("-"){
        flags = 0;
        *f_meta.try_write().unwrap() = None;

        let iterator = i.chars().skip(1);
        for j in iterator{
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

        if flags != 0{
            if process_flags_cond(flags,i){
               Git_Params.push(v);
            }
        }
        
    }


    let mut buff = String::new();
    buff += clargs.splice_files.iter().fold("".to_string(), | acc,x | {acc + "T@" +&x.path  + "\n"}).trim();
    if Path::new(".gitignore").exists(){
        buff = fs::read_to_string(".gitignore").unwrap() + &buff;
    }
        fs::write(".gitignore", buff).unwrap();
    
    // Slices and generates a T@f_name file containingf og file contrents and copies sliced content to the actual file
    clargs.splice_files.iter().for_each(|x| {
        let mut t_buff = String::new();
        if Path::new(&x.path).exists(){
            t_buff += &fs::read_to_string(&x.path).unwrap();
            fs::write("T@".to_string() + &x.path, &t_buff).unwrap();
            fs::write(&x.path, &t_buff[x.slice_idx.0..x.slice_idx.1]).unwrap();
        }
    });


    // TODO: Run the git add command here
    if !Path::new(".git").exists(){
            let output = Command::new("git").args(["init"]).output().expect("Failed to execute git init");            
    }

    
    let output = Command::new("git").args(&Git_Params).output().expect(&format!("Failed to execute git add {Git_Params:?}"));
    if let Some(x) = url{
        let output = Command::new("git").args(["remote","add","origin",&x]).output().expect(&format!("Failed to execute git remote add origin {x}"));
    }

    if let Some(x) = clargs.commit_msg{
        let output = Command::new("git").args(["commit","-m",&x]).output().expect(&format!("Failed to execute git commit -m {x}"));
    }






    // Swaps sliced and normal file to regen the actual files
    clargs.splice_files.iter().for_each(|x| {
        let mut t_buff = String::new();
        if Path::new(&x.path).exists(){
            t_buff += &fs::read_to_string(&x.path).unwrap(); // sliced
            fs::write(&x.path, fs::read_to_string("T@".to_string() + &x.path).unwrap()).unwrap();
            fs::write("T@".to_string() + &x.path, &t_buff).unwrap();
        }
    });   

    println!("Completed Workflow!!");

}
