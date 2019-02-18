# RuSh - Rust Shell

Little shell app written in Rust with ncurses bindings for OS class.
It's a student project and basically first thing I wrote in Rust,
so this might be pretty bad place if you're looking for Rust best practices or nice shell example.

Rush supports:
* Command editing in place with support of Backspace, Delete and cursor moves
* History with access to previous commands by UP and DOWN arrows
* Joining two or more commands by separating them with `&` sign
* Pipelining commands using `|` sign
* Showing errors in RED and critical parts of prompt in other colors
* Two internal commands: `cd` and `ls`
* External commands (RUN AND GET RESULT MODE ONLY)
