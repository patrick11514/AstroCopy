pub mod args {
    use clap::Parser;

    #[derive(Parser, Debug)]
    pub struct Args {
        #[arg(short, long, default_value = "<NONE_PATH>")]
        pub path: String,
    }
}
