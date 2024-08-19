mod args;
mod prompt;

use console::style;
use prompt::prompt::show_prompt;

fn main() {
    match show_prompt() {
        Err(err) => {
            println!("{}", style(err).red().bold());
        }
        Ok(_) => (),
    }
}
