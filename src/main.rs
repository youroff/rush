extern crate ncurses;

mod rush;

use ncurses::*;

fn main() {
  initscr();
  start_color();
  raw();
  keypad(stdscr(), true);
  scrollok(stdscr(), true);
  noecho();

  let mut shell = rush::Session::new();
  shell.run();
  endwin();
}
