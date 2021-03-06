use std::env;
use std::io;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use getopts::Options;
use bansuri;

fn print_usage(program: &str, opts: Options) {
  let brief = format!("Usage: {} [options]", program);
  println!("{}", opts.usage(&brief));
}

fn parse_file(input: Option<String>, output: Option<String>) {
  let mut output_stream = match output {
      Some(x) => {
        let path = Path::new(&x);
        match File::create(path) {
          Err(why) => panic!("couldn't create {}: {}", path.display(), why.to_string()),
          Ok(file) => Box::new(io::BufWriter::new(file)) as Box<dyn io::Write>,
        }
      },
      None => Box::new(io::stdout()) as Box<dyn io::Write>,
    };

  let mut input_stream = match input {
      Some(x) => {
        let path = Path::new(&x);
        match File::open(path, ) {
          Err(why) => panic!("couldn't open {}: {}", path.display(), why.to_string()),
          Ok(file) => Box::new(io::BufReader::new(file)) as Box<dyn io::Read>,
        }
      },
      None => Box::new(io::stdin()) as Box<dyn io::Read>,
    };

    bansuri::parser::parse_js_reader(&mut input_stream, &mut output_stream);
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();
  opts.optopt("o", "", "set output file name", "OUTPUT_FILE_NAME");
  opts.optopt("i", "", "set input file name", "INPUT_FILE_NAME");

  opts.optflag("h", "help", "print this help menu");

  let matches = match opts.parse(&args[1..]) {
    Ok(m) => { m }
    Err(f) => { panic!(f.to_string()) }
  };

  if matches.opt_present("h") {
    print_usage(&program, opts);
    return;
  }

  parse_file(matches.opt_str("i"), matches.opt_str("o"));
}
