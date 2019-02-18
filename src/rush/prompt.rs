use std::env;
use std::process::Command;
use std::char;
use ncurses::*;

pub const ENTER: i32 = 10;
pub const BACKSPACE: i32 = 127;
pub const DELETE: i32 = 330;

pub fn ask(history: &[String]) -> String { //&session: Session
  let mut max_x: i32 = 0;
  let mut max_y: i32 = 0;
  getmaxyx(stdscr(), &mut max_y, &mut max_x);

  let mut cur_x: i32 = 0;
  let mut cur_y: i32 = 0;
  
  let mut line: i32 = 0;
  let mut prompt_offset: i32 = 0;
  prompt_msg(&mut line, &mut prompt_offset);
  
  let mut cmd_offset: usize = 0;
  let hist = history.clone();
  let mut hist_ctr = 0;
  let mut tmp_command = String::new();
  let mut command = String::from("");
  loop {
    match getch() {
      ENTER => break,
      KEY_UP if hist_ctr < history.len() => {
        if hist_ctr == 0 {
          tmp_command = command.clone();
        };
        hist_ctr += 1;
        command = hist[hist_ctr - 1].clone();
        cmd_offset = command.len();
      },
      KEY_DOWN if hist_ctr > 0 => {
        hist_ctr -= 1;
        if hist_ctr == 0 {
          command = tmp_command.clone();
        } else {
          command = hist[hist_ctr - 1].clone();          
        }
        cmd_offset = command.len();
      },
      KEY_LEFT if cmd_offset > 0 => cmd_offset -= 1,
      KEY_RIGHT if cmd_offset < command.len() => cmd_offset += 1,
      BACKSPACE if cmd_offset > 0 => {
        command.remove(cmd_offset - 1);
        cmd_offset -= 1;
      },
      DELETE if cmd_offset < command.len() => {
        command.remove(cmd_offset);
      },
      ch if ch > 31 && ch < 127 => {
        command.insert(cmd_offset, ch as u8 as char);
        cmd_offset += 1;
      },
      _ => ()
    }

    mv(line, prompt_offset);
    clrtobot();
    mvprintw(line, prompt_offset, command.as_ref());
    getyx(stdscr(), &mut cur_y, &mut cur_x);
    line = cur_y - (prompt_offset + command.len() as i32) / max_x;
    mv(line + (prompt_offset + cmd_offset as i32) / max_x, (prompt_offset + cmd_offset as i32) % max_x);
  }
  mvprintw(line + (prompt_offset + command.len() as i32) / max_x, (prompt_offset + command.len() as i32) % max_x, "\n");
  return command;
}

fn prompt_msg(line: &mut i32, prompt_offset: &mut i32) {
  print_name();
  printw("@");
  print_host();
  printw(":");
  print_dir();
  printw("$ ");
  getyx(stdscr(), line, prompt_offset);
}

fn print_name() {
  let user = env::var("USER").unwrap_or("unknown".to_string());
  init_pair(2, COLOR_CYAN, COLOR_BLACK);
  attron(COLOR_PAIR(2));
  printw(user.as_ref());
  attroff(COLOR_PAIR(2));    
}

fn print_dir() {
  init_pair(3, COLOR_MAGENTA, COLOR_BLACK);
  attron(COLOR_PAIR(3));
  match env::current_dir() {
    Ok(path) => match path.to_str() {
      Some(s) => match dirs::home_dir() {
        Some(path) => printw(s.replace(path.to_str().unwrap(), "~").as_ref()),
        None => printw(s),
      },
      None => printw("<error>")
    },
    Err(_) => printw("<error>")
  };
  attroff(COLOR_PAIR(3));    
}

fn print_host() {
  let out = Command::new("hostname").output().ok().expect("-?-");
  let host = String::from_utf8_lossy(out.stdout.as_slice());
  init_pair(4, COLOR_YELLOW, COLOR_BLACK);
  attron(COLOR_PAIR(4));
  printw(host.trim().as_ref());
  attroff(COLOR_PAIR(4));
}

