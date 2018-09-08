use std::path::Path;
use std::env;

fn main() {
	
	let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  //println!("cargo:rustc-link-search=native={}", Path::new(&dir).join("Clibs/test").display());
  println!("cargo:rustc-link-search={}", Path::new(&dir).join("Clibs/test").display());  
}
