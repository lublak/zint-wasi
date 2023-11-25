use std::ffi::CStr;

use zint_wasm_sys::{
    free_svg_plot_string, svg_plot_string, zint_symbol, ZBarcode_Encode_and_Buffer_Vector,
};

use crate::error::ZintErrorWarning;

pub struct Symbol {
    inner: *mut zint_symbol,
}

impl Symbol {
    pub unsafe fn from_ptr(ptr: *mut zint_symbol) -> Self {
        Self { inner: ptr }
    }

    pub unsafe fn get_ptr(&self) -> *mut zint_symbol {
        self.inner
    }

    pub unsafe fn get_mut(&mut self) -> &mut zint_symbol {
        self.inner.as_mut().unwrap()
    }

    pub fn encode(self, data: &[u8], length: i32, rotate_angle: i32) -> Result<String, String> {
        let error_code = unsafe {
            ZBarcode_Encode_and_Buffer_Vector(self.inner, data.as_ptr(), length, rotate_angle)
        };
        let error: ZintErrorWarning = error_code.into();
        match error {
            ZintErrorWarning::Error(error) => return Err(format!("Error: {:#?}", error)),
            ZintErrorWarning::Warning(_warn) => {} // ZintErrorWarning::Warning(warn) => return Err(format!("Warning: {:#?}", warn)),
        };
        let mut err_code: i32 = 0;
        let res = unsafe {
            let err_code_ptr = &mut err_code as *mut i32;
            let svg_cstr = svg_plot_string(self.inner, err_code_ptr);
            let svg_str = CStr::from_ptr(svg_cstr).to_string_lossy().into_owned();
            free_svg_plot_string(svg_cstr);
            svg_str
        };
        let error: ZintErrorWarning = err_code.into();
        match error {
            ZintErrorWarning::Error(error) => return Err(format!("Error: {:#?}", error)),
            ZintErrorWarning::Warning(_warn) => {} // ZintErrorWarning::Warning(warn) => return Err(format!("Warning: {:#?}", warn)),
        };
        Ok(res)
    }
}

impl Drop for Symbol {
    fn drop(&mut self) {
        unsafe {
            zint_wasm_sys::ZBarcode_Delete(self.inner);
        }
    }
}
