extern crate napi_build;

fn main() {
  println!("cargo:rustc-link-lib=dylib=datahike");
  println!("cargo:rustc-link-search=native=./");
  napi_build::setup();
}
