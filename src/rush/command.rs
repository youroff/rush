use ncurses::*;
use std::process::Stdio;
use std::process::Command as Cmd;
use std::str;
use std::io::prelude::*;
use std::error::Error;
use std::env;
use std::path::PathBuf;
use std::fs;

pub fn run(cmd: &str) {
  for segment in cmd.split("&").map(|s| s.trim()) {
    let command = segment.split("|").map(|p| p.trim()).fold(None, |prev, piped| {
      let mut c = Command::new(piped, prev);
      c.run();
      Some(c)
    });
    
    match command {
      Some(c) => {
        if !c.stderr.is_empty() {
          init_pair(5, COLOR_RED, COLOR_BLACK);
          attron(COLOR_PAIR(5));
          printw(c.stderr.as_ref());
          attroff(COLOR_PAIR(5));
        };

        if !c.stdout.is_empty() {
          printw(c.stdout.as_ref());
        };
      },
      None => ()
    }
  }
}

enum CommandType {
  Internal(String),
  External(String)
}

struct Command {
  pipe_in: Option<Box<Command>>,
  exec:    CommandType,
  args:    Vec<String>,
  stdout:  String,
  stderr:  String
}

impl Command {
  pub fn new(cmd: &str, prev: Option<Command>) -> Command {
    let mut args: Vec<String> = cmd.split_whitespace().map(|s| s.to_string()).collect();
    let exec = match args.remove(0).as_ref() {
      "cd" => CommandType::Internal("cd".to_string()),
      "ls" => CommandType::Internal("ls".to_string()),
      ext => CommandType::External(ext.to_string())
    };
    Command { exec: exec, args: args, pipe_in: prev.map(|c| Box::new(c)), stdout: "".to_string(), stderr: "".to_string() }
  }
  
  fn run(&mut self) {
    match self.exec {
      CommandType::External(ref cmd) => {
        match Cmd::new(&cmd)
          .args(&self.args)
          .stdin(Stdio::piped())
          .stdout(Stdio::piped())
          .spawn() {
            Err(why) => self.stderr = why.description().to_string() + "\n",
            Ok(process) => {
              match self.pipe_in {
                Some(ref prev) => {
                  process.stdin.unwrap().write_all(prev.stdout.as_bytes()).expect("Couldn't write to process stdin");
                },
                None => ()
              };

              if process.stdout.is_some() {
                process.stdout.unwrap().read_to_string(&mut self.stdout).expect("Couldn't read processes stdout");
              };

              if process.stderr.is_some() {
                process.stderr.unwrap().read_to_string(&mut self.stderr).expect("Couldn't read processes stderr");
              };
            }
          }
      },
      CommandType::Internal(ref int) => match int.as_ref() {
        "cd" => {
          let dir = get_relative_path(self.args.pop());
          match env::set_current_dir(&dir) {
            Err(why) => self.stderr = why.description().to_string() + "\n",
            Ok(_) => ()
          }
        },
        "ls" => {
          let dir = get_relative_path(self.args.pop().or(Some(".".to_string())));
          match fs::read_dir(dir) {
            Err(why) => self.stderr = why.description().to_string() + "\n",
            Ok(buf) => {
              let conts = buf.map(|entry| {
                match entry {
                  Ok(path) => path.file_name().to_str().unwrap().to_string(),
                  Err(why) => why.description().to_string()
                }
              });
              self.stdout = conts.collect::<Vec<String>>().join("\n").to_string() + "\n";
            }
          }
        },        
        _ => ()
      },
    }
  }
}

fn get_relative_path(segment: Option<String>) -> PathBuf {
  let mut current_dir = env::current_dir().unwrap();
  let mut home_dir = dirs::home_dir().unwrap();
  match segment {
    Some(path) => {
      if path.starts_with("~") {
        home_dir.push(path.replace("~/", ""));
        home_dir.to_path_buf()
      } else {
        current_dir.push(path);
        current_dir.to_path_buf()
      }
    },
    None => home_dir.to_path_buf()
  }
}
