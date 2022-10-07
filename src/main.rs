use serde_derive::{Serialize, Deserialize};
use home::home_dir;
use std::fs::File;
use std::io::{Read, Write};
use chrono::{Utc, Datelike};
use std::process::exit;


#[derive(Deserialize, Serialize, Clone)]
struct DoneConf {
    started: Option<String>,
    done: Vec<String>,
    date: (i32, u32, u32),
}

fn read_config(file_path: &str) -> DoneConf {
    let mut file: File = File::open(file_path).unwrap();
    let mut buf: String = String::new();
    file.read_to_string(&mut buf).unwrap();
    let config_in_toml: DoneConf = toml::from_str(&buf).unwrap();
    config_in_toml
}

fn start_task(name: String, file_path: &str) {
    let mut current_config: DoneConf = read_config(file_path);
    if let Some(task) = current_config.started {
        eprintln!("Task `{}` already started. Finish it first", task);
        exit(1);
    }
    current_config.started = Some(name.to_owned());
    let mut file_to_write = File::options().write(true).truncate(true).open(file_path).unwrap();
    let text = toml::to_string(&current_config).unwrap();
    file_to_write.write_all(text.as_bytes()).unwrap();
    println!("Task {} started", name);
}

fn finish_task(file_path: &str) {
    let mut current_config: DoneConf = read_config(file_path);
    if current_config.started.is_none() {
        eprintln!("Nothing to finish!");
        exit(1);
    }
    let task_to_finish = current_config.started.unwrap();
    current_config.started = None;
    current_config.done.push(task_to_finish.to_owned());
    let mut file_to_write = File::options().write(true).truncate(true).open(file_path).unwrap();
    file_to_write.write_all(toml::to_string(&current_config).unwrap().as_bytes()).unwrap(); 
    println!("Task {} finished", task_to_finish);

}

fn list_tasks(file_path: &str) {
    let current_config: DoneConf = read_config(file_path);
    if current_config.done.is_empty() {
        println!("Nothing to list");
        exit(0);
    }
    current_config.done.iter().for_each(
        |task| {
            println!("{}", task);
        }
    )
}

fn abadon_task(file_path: &str) {
    let mut current_config: DoneConf = read_config(file_path);
    if current_config.started.is_none() {
        eprintln!("Nothing to abadon!");
        exit(1);
    }
    
    let task_to_abadon = current_config.started.unwrap();
    current_config.started = None;
    let mut file_to_write: File = File::options().write(true).truncate(true).open(file_path).unwrap();
    file_to_write.write_all(toml::to_string(&current_config).unwrap().as_bytes()).unwrap();
    println!("Task {} abadoned", task_to_abadon);

}

fn print_usage_string() {

    println!(r#"
 USAGE:
    done! start <TASK_NAME> - Start task with given name
    done! finish            - Finish started task
    done! list              - List finished tasks
    done! abadon            - Abadon started task
             "#);
    
}

fn clear_config(file_path: String) {
    let mut file_to_clear = File::options().write(true).open(file_path).unwrap();
    let cur_date = Utc::now();
    let default_config: DoneConf = DoneConf {
        done: Vec::new(),
        date: (cur_date.year(), cur_date.month(), cur_date.day()),
        started: None,
    };
    file_to_clear.write_all(toml::to_string(&default_config).unwrap().as_bytes()).unwrap();
}

fn check_date(file_path: String) {
    let current_time = Utc::now();
    let timestamp = (current_time.year(), current_time.month(), current_time.day());

    let current_config = read_config(&file_path);
    if current_config.date != timestamp {
        clear_config(file_path);
    }
}

fn check_file(file_path: String) {
    let mut file: File = File::options()
                                .write(true)
                                .read(true)
                                .create(true)
                                .open(&file_path)
                                .unwrap();
    let mut buf: String = String::new(); 
    let cur_date = Utc::now();

    file.read_to_string(&mut buf).unwrap();
    if buf.is_empty() {
        let default_config: DoneConf = DoneConf {
            done: Vec::new(),
            date: (cur_date.year(), cur_date.month(), cur_date.day()),
            started: None,
        };
        file.write_all(toml::to_string(&default_config).unwrap().as_bytes()).unwrap();
    }
}


fn main() { 
    let args: Vec<String> = std::env::args().skip(1).collect();
    let str_args: Vec<&str> = args.iter().map(|v| &v[..]).collect();
    let config_file_path: String = format!("{}/.done.toml", home_dir().unwrap().to_str().unwrap());

    check_file(config_file_path.to_owned());
    check_date(config_file_path.to_owned());

    match str_args[..] {
        ["start", name] => start_task(name.to_string(), &config_file_path),
        ["finish"] => finish_task(&config_file_path),
        ["list"] => list_tasks(&config_file_path),
        ["abadon"] => abadon_task(&config_file_path),
        _ => print_usage_string()
    };
}
