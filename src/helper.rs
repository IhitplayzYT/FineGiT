pub mod Helper{
    use std::{collections::{HashMap, HashSet}, fs::{self, Metadata}, ops::ControlFlow::Continue, path::{Path, PathBuf}, process::{Command, exit}};

    use std::sync::RwLock;
    use std::sync::LazyLock;

    pub const DBG_STR: &str = "";
    pub const OK:i32 = 0;
    pub const ERR:i32 = -1;

    pub static f_meta: LazyLock<RwLock<Option<Metadata>>> = LazyLock::new(|| {RwLock::new(None)});


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
        pub condition_params: Vec<String>,
        pub local_commit_log: Option<PathBuf>,
        pub auto_repo_link: bool,
    }

    



    pub fn Help(){
        println!("{DBG_STR}");
        exit(OK);
    }


    impl CLI{
        pub fn new() -> Self{
            Self {dbg: false ,files:HashSet::new(),splice_files:vec![],nested_gitignore:false,graph:false,conditions:vec![],local_commit_log:None,auto_repo_link: false,condition_params:vec![]}
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
                } else if i.starts_with("--if") || i.starts_with("--IF"){
                    self.conditions.push((&i[i.find("[").unwrap()+1..i.find("]").unwrap()]).to_string());
                }  else if i.starts_with("--if_then") || i.starts_with("--IF_THEN"){
                    self.condition_params.push((&i[i.find("[").unwrap()+1..i.find("]").unwrap()]).to_string());
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

        pub fn process_flags(flags:u8,param: &str) {
            if flags & (1 << 0) != 0{
                if !Path::new(param).exists(){
                    panic!("IF dependency not satisfied: {param} does not Exist!!");
                }
            }
            
            if flags & (1 << 1) != 0{
                if Path::new(param).exists(){
                    panic!("IF dependency not satisfied: {param} Exists!!");
                }
            }

            if flags & (1 << 2) != 0{
                if !Path::new(param).is_file(){
                    panic!("IF dependency not satisfied: {param} is not a File!!");
                }
            }

            if flags & (1 << 3) != 0{
                if !Path::new(param).is_dir(){
                    panic!("IF dependency not satisfied: {param} is not a Directory!!");
                }
            }
            if flags & (1 << 4) != 0{
                if !Path::new(param).exists(){
                    fs::create_dir_all(param).unwrap();
                }
            }

            if flags & (1 << 5) != 0{
                if f_meta.try_read().unwrap().is_none(){
                    let _ = f_meta.try_write().unwrap().insert(fs::metadata(param).unwrap());
                }else{
                    let f2_meta = fs::metadata(param).unwrap();
                    if let Some(x) = &*f_meta.try_read().unwrap(){
                       if f2_meta.len() == x.len() && ((f2_meta.is_file() && x.is_file()) || (f2_meta.is_dir() && x.is_dir())) {
                       } else{
                            panic!("IF dependency not satisfied: Files are not Similar!!");
                       }
                    }

                }
            }
            
            if flags & (1 << 6) != 0{
                if f_meta.try_read().unwrap().is_none(){
                    let _ = f_meta.try_write().unwrap().insert(fs::metadata(param).unwrap());
                }else{
                    let f2_meta = fs::metadata(param).unwrap();
                    if let Some(x) = &*f_meta.try_read().unwrap(){
                       if f2_meta.len() == x.len() && ((f2_meta.is_file() && x.is_file()) || (f2_meta.is_dir() && x.is_dir())) {
                            panic!("IF dependency not satisfied: Files are Similar!!");
                       }
                    }

                }
            }





        }



}