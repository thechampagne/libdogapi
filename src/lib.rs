/*
 * Copyright 2022 XXIV
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::ptr::null_mut;
use std::ffi::CStr;
use std::ffi::CString;
use std::slice;
use dogapi::breeds_list;
use dogapi::images_by_breed;
use dogapi::images_by_sub_breed;
use dogapi::multiple_random_images;
use dogapi::multiple_random_images_by_breed;
use dogapi::multiple_random_images_by_sub_breed;
use dogapi::random_image;
use dogapi::random_image_by_breed;
use dogapi::random_image_by_sub_breed;
use dogapi::sub_breeds_list;

#[repr(C)]
struct dogapi_breeds_list_t {
  length: usize,
  values_length: *mut usize,
  keys: *mut *mut c_char,
  values: *mut *mut *mut c_char
}

#[no_mangle]
unsafe extern "C" fn dogapi_breeds_list(dogapi_breeds_list: *mut dogapi_breeds_list_t) -> c_int {
  let res = match breeds_list() {
    Ok(v) => v,
    Err(_) => return -1
  };
  let mut keys = vec![];
  let mut values = vec![];
  let mut values_length = vec![];
  for (k, v) in res {
    keys.push(match CString::new(k) {
      Ok(v) => v.into_raw(),
      Err(_) => null_mut()
    });
    match v {
      None => {
        values.push(null_mut());
        values_length.push(0);
      },
      Some(v) => {
        let mut array: Vec<*mut c_char> = v.iter().map(|i| match CString::new(i.as_str()) {
          Ok(s) => s.into_raw(),
          Err(_) => null_mut()
        } ).collect();
        let carray = array.as_mut_ptr();
        values_length.push(array.len());
        std::mem::forget(array);
        values.push(carray);
      }, 
    }
  }
  (*dogapi_breeds_list).length = keys.len();
  (*dogapi_breeds_list).values_length = values_length.as_mut_ptr();
  (*dogapi_breeds_list).keys = keys.as_mut_ptr();
  (*dogapi_breeds_list).values = values.as_mut_ptr();
  std::mem::forget(keys);
  std::mem::forget(values);
  std::mem::forget(values_length);
  0
} 

#[no_mangle]
unsafe extern "C" fn dogapi_images_by_breed(breed: *const c_char, length: *mut usize) -> *mut *mut c_char {
  let breed_rs = match CStr::from_ptr(breed).to_str() {
    Ok(v) => v,
    Err(_) => return null_mut()
  };
  let res = match images_by_breed(breed_rs) {
    Ok(v) => v,
    Err(_) => return null_mut()
  };
  let mut array: Vec<*mut c_char> = res.iter().map(|i| match CString::new(i.as_str()) {
    Ok(s) => s.into_raw(),
    Err(_) => null_mut()
  } ).collect();
  let carray = array.as_mut_ptr();
  *length = array.len();
  std::mem::forget(array);
  carray
}

#[no_mangle]
unsafe extern "C" fn dogapi_images_by_sub_breed(breed: *const c_char, sub_breed: *const c_char, length: *mut usize) -> *mut *mut c_char {
  let breed_rs = match CStr::from_ptr(breed).to_str() {
    Ok(v) => v,
    Err(_) => return null_mut()
  };
  let sub_breed_rs = match CStr::from_ptr(sub_breed).to_str() {
    Ok(v) => v,
    Err(_) => return null_mut()
  };
  let res = match images_by_sub_breed(breed_rs, sub_breed_rs) {
    Ok(v) => v,
    Err(_) => return null_mut()
  };
  let mut array: Vec<*mut c_char> = res.iter().map(|i| match CString::new(i.as_str()) {
    Ok(s) => s.into_raw(),
    Err(_) => null_mut()
  } ).collect();
  let carray = array.as_mut_ptr();
  *length = array.len();
  std::mem::forget(array);
  carray
}

#[no_mangle]
unsafe extern "C" fn dogapi_multiple_random_images(images_number: i8, length: *mut usize) -> *mut *mut c_char {
  let res = match multiple_random_images(images_number) {
    Ok(v) => v,
    Err(_) => return null_mut()
  };
  let mut array: Vec<*mut c_char> = res.iter().map(|i| match CString::new(i.as_str()) {
    Ok(s) => s.into_raw(),
    Err(_) => null_mut()
  } ).collect();
  let carray = array.as_mut_ptr();
  *length = array.len();
  std::mem::forget(array);
  carray
}

#[no_mangle]
unsafe extern "C" fn dogapi_multiple_random_images_by_breed(breed: *const c_char, images_number: i64, length: *mut usize) -> *mut *mut c_char {
  let breed_rs = match CStr::from_ptr(breed).to_str() {
    Ok(v) => v,
    Err(_) => return null_mut()
  };
  let res = match multiple_random_images_by_breed(breed_rs, images_number) {
    Ok(v) => v,
    Err(_) => return null_mut()
  };
  let mut array: Vec<*mut c_char> = res.iter().map(|i| match CString::new(i.as_str()) {
    Ok(s) => s.into_raw(),
    Err(_) => null_mut()
  } ).collect();
  let carray = array.as_mut_ptr();
  *length = array.len();
  std::mem::forget(array);
  carray
}

#[no_mangle]
unsafe extern "C" fn dogapi_multiple_random_images_by_sub_breed(breed: *const c_char, sub_breed: *const c_char, images_number: i64, length: *mut usize) -> *mut *mut c_char {
  let breed_rs = match CStr::from_ptr(breed).to_str() {
    Ok(v) => v,
    Err(_) => return null_mut()
  };
  let sub_breed_rs = match CStr::from_ptr(sub_breed).to_str() {
    Ok(v) => v,
    Err(_) => return null_mut()
  };
  let res = match multiple_random_images_by_sub_breed(breed_rs, sub_breed_rs, images_number) {
    Ok(v) => v,
    Err(_) => return null_mut()
  };
  let mut array: Vec<*mut c_char> = res.iter().map(|i| match CString::new(i.as_str()) {
    Ok(s) => s.into_raw(),
    Err(_) => null_mut()
  } ).collect();
  let carray = array.as_mut_ptr();
  *length = array.len();
  std::mem::forget(array);
  carray
}

#[no_mangle]
unsafe extern "C" fn dogapi_random_image() -> *mut c_char {
  let res = match random_image() {
    Ok(v) => v,
    Err(_) => return null_mut()
  };
  match CString::new(res) {
    Ok(v) => v.into_raw(),
    Err(_) => null_mut()
  }
}

#[no_mangle]
unsafe extern "C" fn dogapi_random_image_by_breed(breed: *const c_char) -> *mut c_char {
  let breed_rs = match CStr::from_ptr(breed).to_str() {
    Ok(v) => v,
    Err(_) => return null_mut()
  };
  let res = match random_image_by_breed(breed_rs) {
    Ok(v) => v,
    Err(_) => return null_mut()
  };
  match CString::new(res) {
    Ok(v) => v.into_raw(),
    Err(_) => null_mut()
  }
}

#[no_mangle]
unsafe extern "C" fn dogapi_random_image_by_sub_breed(breed: *const c_char, sub_breed: *const c_char) -> *mut c_char {
  let breed_rs = match CStr::from_ptr(breed).to_str() {
    Ok(v) => v,
    Err(_) => return null_mut()
  };
  let sub_breed_rs = match CStr::from_ptr(sub_breed).to_str() {
    Ok(v) => v,
    Err(_) => return null_mut()
  };
  let res = match random_image_by_sub_breed(breed_rs, sub_breed_rs) {
    Ok(v) => v,
    Err(_) => return null_mut()
  };
  match CString::new(res) {
    Ok(v) => v.into_raw(),
    Err(_) => null_mut()
  }
}

#[no_mangle]
unsafe extern "C" fn dogapi_sub_breeds_list(breed: *const c_char, length: *mut usize) -> *mut *mut c_char {
  let breed_rs = match CStr::from_ptr(breed).to_str() {
    Ok(v) => v,
    Err(_) => return null_mut()
  };
  let res = match sub_breeds_list(breed_rs) {
    Ok(v) => match v {
      Some(v) => v,
      None => return null_mut()
    },
    Err(_) => return null_mut()
  };
  let mut array: Vec<*mut c_char> = res.iter().map(|i| match CString::new(i.as_str()) {
    Ok(s) => s.into_raw(),
    Err(_) => null_mut()
  } ).collect();
  let carray = array.as_mut_ptr();
  *length = array.len();
  std::mem::forget(array);
  carray
}

#[no_mangle]
unsafe extern "C" fn dogapi_string_free(ptr: *mut c_char) {
  if !ptr.is_null() {
    _ = CString::from_raw(ptr);
  }
}

#[no_mangle]
unsafe extern "C" fn dogapi_array_free(ptr: *mut *mut c_char, length: usize) {
  if !ptr.is_null() {
    let array = slice::from_raw_parts(ptr, length);
    for &i in array {
      if !i.is_null() {
        _ = CString::from_raw(i);
      }
    }
    Vec::from_raw_parts(ptr, length, length);
  }
}

#[no_mangle]
unsafe extern "C" fn dogapi_breeds_list_free(ptr: *mut dogapi_breeds_list_t) {
  if !ptr.is_null() {
    let keys = slice::from_raw_parts((*ptr).keys, (*ptr).length);
    for &i in keys {
      if !i.is_null() {
        _ = CString::from_raw(i);
      }
    }
    Vec::from_raw_parts((*ptr).keys, (*ptr).length, (*ptr).length);
    let values = slice::from_raw_parts((*ptr).values, (*ptr).length);
    let values_length = slice::from_raw_parts((*ptr).values_length, (*ptr).length);
    let mut i = 0;
    for &v in values {
      if !v.is_null() {
        let value = slice::from_raw_parts(v, values_length[i]);
        for &val in value {
          if !val.is_null() {
            _ = CString::from_raw(val);
          }
        }
        Vec::from_raw_parts(v, values_length[i], values_length[i]);
      }
      i += 1;
    }
    Vec::from_raw_parts((*ptr).values, (*ptr).length, (*ptr).length);
    Vec::from_raw_parts((*ptr).values_length, (*ptr).length, (*ptr).length);
  }
}

#[cfg(test)]
mod tests {
  use crate::*;
  use std::os::raw::c_char;
  use std::ffi::CStr;
  use std::slice;

  #[test]
  fn dogapi_breeds_list_test() {
    unsafe {
      let mut list = dogapi_breeds_list_t{
        length: 0,
        values_length: 0 as *mut usize,
        keys: 0 as *mut *mut c_char,
        values: 0 as *mut *mut *mut c_char
      };
      dogapi_breeds_list(&mut list as *mut dogapi_breeds_list_t);
        let keys = slice::from_raw_parts(list.keys, list.length);
        for &i in keys {
          if !i.is_null() {
            println!("Key: {}", CStr::from_ptr(i).to_str().unwrap());
          }
        }
        let values = slice::from_raw_parts(list.values, list.length);
        let values_length = slice::from_raw_parts(list.values_length, list.length);
        let mut i = 0;
        for &v in values {
          if !v.is_null() {
            let value = slice::from_raw_parts(v, values_length[i]);
            for &val in value {
              if !val.is_null() {
                println!("Value: {}", CStr::from_ptr(val).to_str().unwrap());
              }
            }
          }
          i += 1;
        }
        dogapi_breeds_list_free(&mut list as *mut dogapi_breeds_list_t);
    }
  }

  #[test]
  fn dogapi_images_by_breed_test() {
    unsafe {
      let mut length = 0;
      let array = dogapi_images_by_breed("hound\0".as_ptr() as *const i8, &mut length);
      let slice = slice::from_raw_parts(array, length);
      for i in slice {
        println!("{}", CStr::from_ptr(*i).to_str().unwrap());
      }
      dogapi_array_free(array, length);
    }
  }

  #[test]
  fn dogapi_images_by_sub_breed_test() {
    unsafe {
      let mut length = 0;
      let array = dogapi_images_by_sub_breed("hound\0".as_ptr() as *const i8, "afghan\0".as_ptr() as *const i8, &mut length);
      let slice = slice::from_raw_parts(array, length);
      for i in slice {
        println!("{}", CStr::from_ptr(*i).to_str().unwrap());
      }
      dogapi_array_free(array, length);
    }
  }

  #[test]
  fn dogapi_multiple_random_images_test() {
    unsafe {
      let mut length = 0;
      let array = dogapi_multiple_random_images(10, &mut length);
      let slice = slice::from_raw_parts(array, length);
      for i in slice {
        println!("{}", CStr::from_ptr(*i).to_str().unwrap());
      }
      dogapi_array_free(array, length);
    }
  }

  #[test]
  fn dogapi_multiple_random_images_by_breed_test() {
    unsafe {
      let mut length = 0;
      let array = dogapi_multiple_random_images_by_breed("hound\0".as_ptr() as *const i8, 10, &mut length);
      let slice = slice::from_raw_parts(array, length);
      for i in slice {
        println!("{}", CStr::from_ptr(*i).to_str().unwrap());
      }
      dogapi_array_free(array, length);
    }
  }

  #[test]
  fn dogapi_multiple_random_images_by_sub_breed_test() {
    unsafe {
      let mut length = 0;
      let array = dogapi_multiple_random_images_by_sub_breed("hound\0".as_ptr() as *const i8, "afghan\0".as_ptr() as *const i8, 10, &mut length);
      let slice = slice::from_raw_parts(array, length);
      for i in slice {
        println!("{}", CStr::from_ptr(*i).to_str().unwrap());
      }
      dogapi_array_free(array, length);
    }
  }

  #[test]
  fn dogapi_random_image_test() {
    unsafe {
      let string = dogapi_random_image();
      println!("{}", CStr::from_ptr(string).to_str().unwrap());
      dogapi_string_free(string);
    }
  }

  #[test]
  fn dogapi_random_image_by_breed_test() {
    unsafe {
      let string = dogapi_random_image_by_breed("hound\0".as_ptr() as *const i8);
      println!("{}", CStr::from_ptr(string).to_str().unwrap());
      dogapi_string_free(string);
    }
  }

  #[test]
  fn dogapi_random_image_by_sub_breed_test() {
    unsafe {
      let string = dogapi_random_image_by_sub_breed("hound\0".as_ptr() as *const i8, "afghan\0".as_ptr() as *const i8);
      println!("{}", CStr::from_ptr(string).to_str().unwrap());
      dogapi_string_free(string);
    }
  }

  #[test]
  fn dogapi_sub_breeds_list_test() {
    unsafe {
      let mut length = 0;
      let array = dogapi_sub_breeds_list("hound\0".as_ptr() as *const i8, &mut length);
      let slice = slice::from_raw_parts(array, length);
      for i in slice {
        println!("{}", CStr::from_ptr(*i).to_str().unwrap());
      }
      dogapi_array_free(array, length);
    }
  }
}