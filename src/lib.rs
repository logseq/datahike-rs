#![deny(clippy::all)]

use napi::*;
use std::{
  ffi::{c_char, c_void, CStr, CString},
  ptr,
};

#[macro_use]
extern crate napi_derive;

static mut ISOLATETHREAD: *mut c_void = ptr::null_mut();
static mut LAST_RESULT: Result<String> = Result::Ok(String::new());

#[napi]
pub fn init() -> Result<()> {
  let mut isolate = ptr::null_mut();
  let mut isolatethread = ptr::null_mut();
  unsafe {
    if ISOLATETHREAD.is_null() {
      ffi::graal_create_isolate(ptr::null_mut(), &mut isolate, &mut isolatethread);
      ISOLATETHREAD = isolatethread;
    }
  }
  Ok(())
}

unsafe extern "C" fn parse_return(buf: *const c_char) {
  let cstr = CStr::from_ptr(buf);
  // println!("DEBUG: => {:?}", cstr);
  let mut raw = cstr.to_bytes();
  // NOTE: fix for wrong trailling bytes
  if raw.len() >= 4 && raw.last().unwrap() != &b'}' && raw[raw.len()-4] == b'}' {
    raw = &raw[..raw.len() - 3];
  }
  let s = String::from_utf8(raw.into()).unwrap();
  if s.starts_with("exception:") {
    LAST_RESULT = Result::Err(Error::new(Status::GenericFailure, s));
  } else {
    LAST_RESULT = Result::Ok(s);
  }
}

#[napi]
pub fn database_exists(config: String) -> Result<bool> {
  let output_format = "edn\0";
  let cconfig = CString::new(config).unwrap();

  unsafe {
    ffi::database_exists(
      ISOLATETHREAD,
      cconfig.as_ptr(),
      output_format.as_bytes().as_ptr() as _,
      parse_return as *const c_void,
    );
    LAST_RESULT.clone().map(|v| v == "true")
  }
}

#[napi]
pub fn create_database(config: String) -> Result<()> {
  let output_format = "edn\0";
  let cconfig = CString::new(config).unwrap();

  unsafe {
    ffi::create_database(
      ISOLATETHREAD,
      cconfig.as_ptr(),
      output_format.as_bytes().as_ptr() as _,
      parse_return as *const c_void,
    );
    LAST_RESULT.clone().map(|_| ())
  }
}

#[napi]
pub fn delete_database(config: String) -> Result<()> {
  let output_format = "edn\0";
  let cconfig = CString::new(config).unwrap();

  unsafe {
    ffi::delete_database(
      ISOLATETHREAD,
      cconfig.as_ptr(),
      output_format.as_bytes().as_ptr() as _,
      parse_return as *const c_void,
    );
    LAST_RESULT.clone().map(|_| ())
  }
}

#[napi]
pub fn transact(config: String, tx_data: String) -> Result<String> {
  let input_format = "edn\0";
  let output_format = "edn\0";

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
    LAST_RESULT.clone()
  }
}

/// inputs: [[type, edn]] e.g. [["db", config]]
#[napi]
pub fn query(query_edn: String, inputs: Vec<(String, String)>) -> Result<String> {
  let output_format = "edn\0";

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
    LAST_RESULT.clone()
  }
}

#[napi]
pub fn pull(input_db: String, selector: String, eid: i64) -> Result<String> {
  let output_format = "edn\0";
  let input_format = "db\0";

  let cinput_db = CString::new(input_db).unwrap();
  let cselector = CString::new(selector).unwrap();
  unsafe {
    ffi::pull(
      ISOLATETHREAD,
      input_format.as_ptr() as _,
      cinput_db.as_ptr(),
      cselector.as_ptr(),
      eid,
      output_format.as_ptr() as _,
      parse_return as *const c_void,
    );
    LAST_RESULT.clone()
  }
}

/// eids: [1 2 3 4]
#[napi]
pub fn pull_many(input_db: String, selector: String, eids: String) -> Result<String> {
  let output_format = "edn\0";
  let input_format = "db\0";

  let cinput_db = CString::new(input_db).unwrap();
  let cselector = CString::new(selector).unwrap();
  let ceids = CString::new(eids).unwrap();
  unsafe {
    ffi::pull_many(
      ISOLATETHREAD,
      input_format.as_ptr() as _,
      cinput_db.as_ptr(),
      cselector.as_ptr(),
      ceids.as_ptr(),
      output_format.as_ptr() as _,
      parse_return as *const c_void,
    );
    LAST_RESULT.clone()
  }
}

#[napi]
pub fn entity(input_db: String, eid: i64) -> Result<String> {
  let output_format = "edn\0";
  let input_format = "db\0";

  let cinput_db = CString::new(input_db).unwrap();
  unsafe {
    ffi::entity(
      ISOLATETHREAD,
      input_format.as_ptr() as _,
      cinput_db.as_ptr(),
      eid,
      output_format.as_ptr() as _,
      parse_return as *const c_void,
    );
    LAST_RESULT.clone()
  }
}

/// index_edn: :avet, :aevt, :eavt
#[napi]
pub fn datoms(input_db: String, index_edn: String) -> Result<String> {
  let input_format = "db\0";
  let output_format = "edn\0";

  let cinput_db = CString::new(input_db).unwrap();
  let cindex_edn = CString::new(index_edn).unwrap();

  unsafe {
    ffi::datoms(
      ISOLATETHREAD,
      input_format.as_ptr() as _,
      cinput_db.as_ptr(),
      cindex_edn.as_ptr(),
      output_format.as_ptr() as _,
      parse_return as *const c_void,
    );
    LAST_RESULT.clone()
  }
}

#[napi]
pub fn schema(input_db: String) -> Result<String> {
  let input_format = "db\0";
  let output_format = "edn\0";

  let cinput_db = CString::new(input_db).unwrap();

  unsafe {
    ffi::schema(
      ISOLATETHREAD,
      input_format.as_ptr() as _,
      cinput_db.as_ptr(),
      output_format.as_ptr() as _,
      parse_return as *const c_void,
    );
    LAST_RESULT.clone()
  }
}

#[napi]
pub fn reverse_schema(input_db: String) -> Result<String> {
  let input_format = "db\0";
  let output_format = "edn\0";

  let cinput_db = CString::new(input_db).unwrap();

  unsafe {
    ffi::reverse_schema(
      ISOLATETHREAD,
      input_format.as_ptr() as _,
      cinput_db.as_ptr(),
      output_format.as_ptr() as _,
      parse_return as *const c_void,
    );
    LAST_RESULT.clone()
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
    /*
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
      input_format: *const c_char,
      raw_input: *const c_char,
      selector_edn: *const c_char,
      eid: c_long,
      output_format: *const c_char,
      output_reader: *const c_void,
    );

    pub fn pull_many(
      context: *const c_void,
      input_format: *const c_char,
      raw_input: *const c_char,
      selector_edn: *const c_char,
      eids_edn: *const c_char,
      output_format: *const c_char,
      output_reader: *const c_void,
    );

    pub fn entity(
      context: *const c_void,
      input_format: *const c_char,
      raw_input: *const c_char,
      eid: c_long,
      output_format: *const c_char,
      output_reader: *const c_void,
    );

    pub fn datoms(
      context: *const c_void,
      input_format: *const c_char,
      raw_input: *const c_char,
      index_edn: *const c_char,
      output_format: *const c_char,
      output_reader: *const c_void,
    );

    pub fn schema(
      context: *const c_void,
      input_format: *const c_char,
      raw_input: *const c_char,
      output_format: *const c_char,
      output_reader: *const c_void,
    );

    pub fn reverse_schema(
      context: *const c_void,
      input_format: *const c_char,
      raw_input: *const c_char,
      output_format: *const c_char,
      output_reader: *const c_void,
    );
  }
}
