use ferristype::{list_installed_language, FerrisType};
use itertools::Itertools;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// This is Ferris Type. A small typing game in your terminal.
///
/// You can download many more languages from GitHub
///
/// Link: https://github.com/ZennDev1337/ferristype-language-package
struct App {
    /// Number of words in the typing session.
    #[structopt(default_value = "10")]
    words: usize,
    /// Changes the language for the test (without .json)
    #[structopt(short, long, default_value = "english")]
    language: String,
    /// Lists all installed languages
    #[structopt(long = "list")]
    list: bool,
    /// Number of lines visible.
    #[structopt(short = "p", long, default_value = "3")]
    line_peek: u16,
}

fn main() -> std::io::Result<()> {
    let app = App::from_args();
    if let true = app.list {
        list_installed_language()?
            .iter()
            .sorted_by(|a, b| Ord::cmp(a, b))
            .for_each(|x| println!("{:?}", x));
        return Ok(());
    }
    let game = FerrisType::words(app.language, app.words).line_peek(app.line_peek);
    game.play()
}
