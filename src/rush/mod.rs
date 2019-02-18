use ncurses::*;

mod prompt;
mod command;

pub struct Session {
  history: Vec<String>
}

impl Session {
  pub fn new() -> Session {
    Session { history: vec!() }
  }
  
  pub fn run(&mut self) {
    self.welcome();
    loop {
      match prompt::ask(&self.history).trim() {
        "exit" => break,
        "" => (),
        "!!" => {
          for cmd in self.history.iter() {
            printw(&cmd);
            printw("\n");
          };
        },
        "help" => {
          attron(COLOR_PAIR(1));
          printw("Help\n\n");
          printw("exit - Exit Rush\n");
          printw("!! - Show command history\n");
          printw("help - This help message\n");
          printw("cd arg - Change directory\n");
          printw("ls arg - List directory\n");
          attroff(COLOR_PAIR(1));
        },
        cmd => {
          command::run(cmd);
          self.history.insert(0, cmd.to_string());
        }
      }
    }
  }
  
  fn welcome(&self) {
    init_pair(1, COLOR_GREEN, COLOR_BLACK);
    attron(COLOR_PAIR(1));
    printw("Welcome to Rush!\n\n");
    attroff(COLOR_PAIR(1));
  }
}
