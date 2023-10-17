use std::ffi::c_char;
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::prelude::*;

use paste::paste;
use polars::prelude::*;
use polars::series::*;
use polars_io::parquet::ParquetReader;
use polars_io::SerReader;

macro_rules! series_binary_op {
    ($op:ident) => {
        paste! {
            #[no_mangle]
            pub unsafe extern "C" fn [<series_ $op>](
                left_ptr: *mut Series,
                right_ptr: *mut Series,
            ) -> *mut Series {
                let left = Box::from_raw(left_ptr);
                let right = Box::from_raw(right_ptr);
                let answer = (&*left).$op((&*right));
                Box::into_raw(Box::new(answer))
            }
        }
    };
}

series_binary_op!(add);
series_binary_op!(sub);
series_binary_op!(mul);
series_binary_op!(div);
series_binary_op!(rem);

#[no_mangle]
pub unsafe extern "C" fn dataframe_read_parquet(path_ptr: *const c_char) -> *mut DataFrame {
    let path = std::ffi::CStr::from_ptr(path_ptr).to_str().unwrap();
    let file = std::fs::File::open(path).unwrap();
    let reader = ParquetReader::new(file);
    let answer = reader.finish().unwrap();
    Box::into_raw(Box::new(answer))
}

#[no_mangle]
pub unsafe extern "C" fn dataframe_limit(df_ptr: *mut DataFrame, limit: usize) -> *mut DataFrame {
    let df = Box::from_raw(df_ptr);
    Box::into_raw(Box::new(df.head(Some(limit))))
}

#[no_mangle]
pub unsafe extern "C" fn dataframe_display(df_ptr: *mut DataFrame) -> () {
    let df = Box::from_raw(df_ptr);
    println!("{:?}", df);
}
