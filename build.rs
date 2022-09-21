use static_files::resource_dir;
use std::{
  path::Path,
  fs::File,
  io::prelude::*,
};
use lightningcss::{
  bundler::{Bundler, FileProvider},
  stylesheet::{ParserOptions, PrinterOptions}, //MinifyOptions
};

fn main() -> std::io::Result<()> {

  // write bundled css
  let fs = FileProvider::new();
  let mut bundler = Bundler::new(&fs, None, ParserOptions::default());
  let stylesheet = bundler.bundle(Path::new("./assets/css/global.css")).unwrap();
  // figure out minify later
  // stylesheet.minify(MinifyOptions::default());
  // println!("`{:?}`", stylesheet.minify(MinifyOptions::default()));
  // println!("{}", stylesheet.to_css(PrinterOptions::default()).unwrap().code);
  let mut file = File::create("./static/global.min.css")?;
  write!(file, "{}", stylesheet.to_css(PrinterOptions {
    minify: true,
    ..PrinterOptions::default()
  }).unwrap().code)?;

  resource_dir("./static").build()
}