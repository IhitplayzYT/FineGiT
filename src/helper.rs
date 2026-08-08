pub mod Helper{
    use std::{collections::{HashMap, HashSet}, path::{Path, PathBuf}, process::{Command, exit}};



    const DBG_STR: &str = "";
    const OK:i32 = 0;
    const ERR:i32 = -1;

    #[derive(Debug,Clone)]
    pub struct FileSlice{
        pub path : PathBuf,
        pub slice_idx: (usize,usize)
    }


    #[derive(Debug,Clone)]
    pub struct CLI{
        pub dbg: bool,
        pub files: HashSet<String>,
        pub splice_files: Vec<FileSlice>,
        pub nested_gitignore: bool,
        pub graph: bool,
        pub conditions: Vec<String>,
        pub local_commit_log: Option<PathBuf>,
        pub auto_repo_link: bool,
    }

    



    pub fn Help(){
        println!("{DBG_STR}");
        exit(OK);
    }


    impl CLI{
        pub fn new() -> Self{
            Self {dbg: false ,files:HashSet::new(),splice_files:vec![],nested_gitignore:false,graph:false,conditions:vec![],local_commit_log:None,auto_repo_link: false}
        }

        pub fn Parse_Args(&mut self){
            let args: Vec<String> = std::env::args().skip(1).collect();
           for i in &args{
                if i == "-d" || i == "--debug" || i == " --DEBUG" || i == "-D"{
                    self.dbg = true;
                }else if i == "-h" || i == "--help" || i == " --HELP" || i == "-H"{
                    Help();
                }else if i.contains("[") && i.contains("]"){
                    let fpath = PathBuf::from(&i[..i.find("[").unwrap()]);
                    let (mut lb,mut ub) = (0, 0);
                    for (idx,v) in (&i[i.find("[").unwrap()+1..i.find("]").unwrap()]).split("..").enumerate(){
                        if idx == 0{
                            lb = v.parse::<usize>().expect("Lower Bound is supposed to be usize");
                        } else if idx == 1{
                            if v.starts_with("="){
                                ub = v[1..].parse::<usize>().expect("Upper Bound is supposed to be usize");
                                ub += 1 ;
                            }else{
                                ub = v.parse::<usize>().expect("Upper Bound is supposed to be usize");
                            }
                        }else{
                            panic!("Invalid Range Provided")
                        }
                    }

                  self.splice_files.push(FileSlice { path: fpath, slice_idx: (lb,ub) }); 
                } else if i == "--graph" || i == "--Graph" || i == "-G" || i == "-g"{
                    self.graph = true;
                } else if i == "--nested" || i == "-n" || i == "-N" || i == "--Nested"{
                    self.nested_gitignore = true;
                } else if i.starts_with("-loc"){
                    if i.contains("="){
                        self.local_commit_log = Some(PathBuf::from(&i[i.find("=").unwrap()+1..]));
                    }else{
                        self.local_commit_log = Some(PathBuf::from("COMMIT.md"));
                    }
                } else if i == "--auto" || i == "-a" || i == "--Auto" || i == "-A" {
                    self.auto_repo_link = true
                } else if i.starts_with("--if"){
                    self.conditions.push((&i[i.find("[").unwrap()+1..i.find("]").unwrap()]).to_string());
                }else{
                    self.files.insert(i.to_string());
                }
           } 


        }



    }



        pub fn get_username() -> String{
            let output = Command::new("git").args(["config", "--get", "user.name"]).output().expect("Failed to execute git");
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
    

        pub fn print_commit_history() {

        }



}