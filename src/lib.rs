mod cli;

mod process;
pub use cli::{process_decode, process_encode};
pub use cli::{Base64SubCommand, HttpSubCommand};
pub use cli::{Cli, OutputFormat, SubCommand};
pub use process::csv_convert::process_csv;
pub use process::gen_pass::process_genpass;
pub use process::http_serve::process_http_serve;
