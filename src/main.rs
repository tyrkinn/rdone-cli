use chrono::{Datelike, Utc};
use home::home_dir;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

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

fn write_config(file_path: &str, config: &DoneConf) {
    let mut file_to_write = File::options()
        .write(true)
        .truncate(true)
        .open(file_path)
        .unwrap();
    let text = toml::to_string(&config).unwrap();
    file_to_write.write_all(text.as_bytes()).unwrap();
}

fn start_task(name: String, config: &mut DoneConf) -> Result<&DoneConf, String> {
    match &config.started {
        Some(task) => Err(format!("Task {} already started. Finish it first", task)),
        None => {
            config.started = Some(name);
            Ok(config)
        }
    }
}


fn finish_task(config: &mut DoneConf) -> Result<&DoneConf, String> {
    match &config.started {
        None => Err("Nothing to finish".to_string()),
        Some(task) => {
            config.done.push(task.to_string());
            config.started = None;
            Ok(config)
        }
    } 
}


fn list_tasks(config: &DoneConf) {
    match config.done.is_empty() {
        true => println!("Nothing to list"),
        false => config.done.iter().for_each(|task| println!("{}", task))
    }
}


fn abadon_task(config: &mut DoneConf) -> Result<&DoneConf, String> {
    match config.started {
        Some(..) => {
            config.started = None;
            Ok(config)
        },
        None => Err("Nothing to abadon".to_string())
    }
}


fn print_usage_string() {
    println!(
        r#"
 USAGE:
    rdone start <TASK_NAME> - Start task with given name
    rdone finish            - Finish started task
    rdone list              - List finished tasks
    rdone abadon            - Abadon started task
             "#
    );
}

fn clear_config(file_path: String) {
    let default_config: DoneConf = DoneConf {
        done: Vec::new(),
        date: current_timestamp(),
        started: None,
    };
    write_config(&file_path, &default_config);
}

fn current_timestamp() -> (i32, u32, u32) {
    let current_time = Utc::now();
    return (current_time.year(), current_time.month(), current_time.day());
}

fn check_date(file_path: String) {
    let current_config = read_config(&file_path);
    if current_config.date != current_timestamp() {
        clear_config(file_path);
    }
}

fn check_file(file_path: String) {
    let mut file: File = File::options().read(true).open(&file_path).unwrap();
    let mut buf: String = String::new();
    file.read_to_string(&mut buf).unwrap();
    if buf.is_empty() {
        clear_config(file_path)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let str_args: Vec<&str> = args.iter().map(|v| &v[..]).collect();
    let config_file_path: String = format!("{}/.done.toml", home_dir().unwrap().to_str().unwrap());

    check_file(config_file_path.to_owned());
    check_date(config_file_path.to_owned());

    let mut config = read_config(&config_file_path);

    match str_args[..] {
        ["start", name] => {
            match start_task(name.to_string(), &mut config) {
                Ok(new_conf) => {
                    write_config(&config_file_path, new_conf);
                    println!("Task {} started", name);
                },
                Err(msg) => eprintln!("{}", msg)
            };
        },
        ["finish"] => {
            match finish_task(&mut config) {
                Ok(new_conf) => {
                    write_config(&config_file_path, new_conf);
                    println!("Task {} finished", new_conf.done.last().unwrap());
                },
                Err(msg) => eprintln!("{}", msg)
            }
        },
        ["list"] => list_tasks(&config),
        ["abadon"] => {
            match abadon_task(&mut config) {
                Ok(new_conf) => {
                    write_config(&config_file_path, new_conf);
                    println!("Abadoned task");
                },
                Err(msg) => eprintln!("{}", msg)
            }
        },
        _ => print_usage_string(),
    };
}
