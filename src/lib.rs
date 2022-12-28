#![deny(clippy::all)]

use napi::*;
use std::{
  collections::HashMap,
  ffi::{c_char, c_void, CStr, CString},
  ptr,
};

#[macro_use]
extern crate napi_derive;

static mut ISOLATETHREAD: *mut c_void = ptr::null_mut();
static mut LAST_RESULT: String = String::new();

#[napi]
pub fn init() -> Result<()> {
  unsafe {
    let mut isolate = ptr::null_mut();
    let mut isolatethread = ptr::null_mut();
    ffi::graal_create_isolate(ptr::null_mut(), &mut isolate, &mut isolatethread);

    ISOLATETHREAD = isolatethread;
  }
  Ok(())
}

unsafe extern "C" fn parse_return(buf: *const c_char) {
  let cstr = CStr::from_ptr(buf);
  println!("DEBUG: => {:?}", cstr);
  LAST_RESULT = String::from(cstr.to_str().unwrap());
}

#[napi]
pub fn database_exists(config: String) -> Result<bool> {
  let output_format = "json\0";
  let cconfig = CString::new(config).unwrap();

  unsafe {
    ffi::database_exists(
      ISOLATETHREAD,
      cconfig.as_ptr(),
      output_format.as_bytes().as_ptr() as _,
      parse_return as *const c_void,
    );
    Ok(LAST_RESULT == "true")
  }
}

#[napi]
pub fn create_database(config: String) -> Result<String> {
  let output_format = "json\0";
  let cconfig = CString::new(config).unwrap();

  unsafe {
    ffi::create_database(
      ISOLATETHREAD,
      cconfig.as_ptr(),
      output_format.as_bytes().as_ptr() as _,
      parse_return as *const c_void,
    );
    Ok(LAST_RESULT.clone())
  }
}

#[napi]
pub fn transact(config: String, tx_data: String) -> Result<String> {
  let input_format = "edn\0";
  let output_format = "json\0";

  let cconfig = CString::new(config).unwrap();
  let ctx_data = CString::new(tx_data).unwrap();

  unsafe {
    ffi::transact(
      ISOLATETHREAD,
      cconfig.as_ptr(),
      input_format.as_ptr() as _,
      ctx_data.as_ptr(),
      output_format.as_ptr() as _,
      parse_return as *const c_void,
    );
    Ok(LAST_RESULT.clone())
  }
}

#[napi]
pub fn query(query_edn: String, inputs: Vec<(String, String)>) -> Result<String> {
  let output_format = "json\0";

  let cquery_edn = CString::new(query_edn).unwrap();
  let input_keys = inputs
    .iter()
    .map(|k| CString::new(k.0.clone()).unwrap())
    .collect::<Vec<_>>();
  let input_values = inputs
    .iter()
    .map(|v| CString::new(v.1.clone()).unwrap())
    .collect::<Vec<_>>();

  unsafe {
    ffi::query(
      ISOLATETHREAD,
      cquery_edn.as_ptr(),
      inputs.len() as _,
      input_keys.as_ptr() as _,
      input_values.as_ptr() as _,
      output_format.as_ptr() as _,
      parse_return as *const c_void,
    );
    Ok(LAST_RESULT.clone())
  }
}

mod ffi {
  use std::ffi::{c_char, c_int, c_long, c_longlong, c_void};

  extern "C" {

    pub fn graal_create_isolate(
      params: *mut c_void,
      isolate: *mut *mut c_void,
      thread: *mut *mut c_void,
    ) -> c_int;
    // int graal_create_isolate(graal_create_isolate_params_t* params, graal_isolate_t** isolate, graal_isolatethread_t** thread);

    /*
      void database_exists(long long int, const char*, const char*, const void *);

    void delete_database(long long int, const char*, const char*, const void *);

    void create_database(long long int, const char*, const char*, const void *);

    void query(long long int, const char*, long long int, const char**, const char**, const char*, const void *);

    void transact(long long int, const char*, const char*, const char*, const char*, const void *);

    void pull(long long int, const char*, const char*, const char*, long long int, const char*, const void *);

    void pull_many(long long int, const char*, const char*, const char*, const char*, const char*, const void *);

    void entity(long long int, const char*, const char*, long long int, const char*, const void *);

    void datoms(long long int, const char*, const char*, const char*, const char*, const void *);

    void schema(long long int, const char*, const char*, const char*, const void *);

    void reverse_schema(long long int, const char*, const char*, const char*, const void *);

    void metrics(long long int, const char*, const char*, const char*, const void *);
     */
    pub fn database_exists(
      context: *const c_void,
      db_config: *const c_char,
      output_cormat: *const c_char,
      output_reader: *const c_void,
    );

    pub fn delete_database(
      context: *const c_void,
      db_config: *const c_char,
      output_cormat: *const c_char,
      output_reader: *const c_void,
    );

    pub fn create_database(
      context: *const c_void,
      db_config: *const c_char,
      output_cormat: *const c_char,
      output_reader: *const c_void,
    );

    pub fn query(
      context: *const c_void,
      query_edn: *const c_char,
      num_inputs: c_longlong,
      input_formats: *const *const c_char,
      raw_inputs: *const *const c_char,
      output_format: *const c_char,
      output_reader: *const c_void,
    );

    pub fn transact(
      context: *const c_void,
      db_config: *const c_char,
      tx_format: *const c_char,
      tx_data: *const c_char,
      output_format: *const c_char,
      output_reader: *const c_void,
    );

    pub fn pull(
      context: *const c_void,
      raw_input: *const c_char,
      selector_edn: *const c_char,
      eid: c_long,
      output_format: *const c_char,
      output_reader: *const c_void,
    );

    // TODO: pull_many

  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let config = "{:store {:backend :file :path \"./path-to-db\"} :schema-flexibility :read}";
    init();
    create_database(config.into());
  }
}
