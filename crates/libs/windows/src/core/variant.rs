use super::*;
use bindings::*;

const LOCALE_USER_DEFAULT: u32 = 1024;
const VT_EMPTY: u16 = 0;
const VT_NULL: u16 = 1;
const VT_BSTR: u16 = 8;
const VARCMP_EQ: u32 = 1;

#[repr(C)]
pub struct VARIANT {
    pub Anonymous: VARIANT_0,
}

impl VARIANT {
    pub fn vt(&self) -> u16 {
        unsafe { self.Anonymous.Anonymous.vt }
    }
}

impl std::ops::Drop for VARIANT {
    fn drop(&mut self) {
        if self.vt() != VT_EMPTY {
            unsafe { let _ = VariantClear(self); }
        }
    }
}

impl std::clone::Clone for VARIANT {
    fn clone(&self) -> Self {
        unsafe { 
            let mut copy = Self::default();
            VariantCopy(&mut copy, self).expect("Failed to copy VARIANT");
            copy
         }
    }
}

unsafe impl ::windows::core::Abi for VARIANT {
    type Abi = Self;
}

impl std::default::Default for VARIANT {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl std::fmt::Display for VARIANT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            if self.vt() == VT_BSTR {
                let bstr: &BSTR = std::mem::transmute(&self.Anonymous.Anonymous.Anonymous.bstrVal);
                return std::write!(f, "{}", bstr);
            }
            let mut buffer = PWSTR::null();
            let _ = VariantToStringAlloc(self, &mut buffer);
            std::write!(f, "{}", buffer.display())?;
            CoTaskMemFree(buffer.0 as *const _);
            Ok(())
        }
    }
}

impl std::fmt::Debug for VARIANT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::write!(f, "{}", self)
    }
}

impl std::cmp::PartialEq for VARIANT {
    fn eq(&self, other: &Self) -> bool {
        if self.vt() == VT_NULL && other.vt() == VT_NULL {
            return true;
        }
        if self.vt() != other.vt() {
            return false;
        }
        unsafe { VarCmp(self as *const _, other, LOCALE_USER_DEFAULT, 0) == VARCMP_EQ }
    }
}
impl std::cmp::Eq for VARIANT {}

#[repr(C)]
pub union VARIANT_0 {
    pub Anonymous: VARIANT_0_0,
    pub decVal: DECIMAL,
}
impl std::marker::Copy for VARIANT_0 {}
impl std::clone::Clone for VARIANT_0 {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
pub struct VARIANT_0_0 {
    pub vt: u16, //VARENUM
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: VARIANT_0_0_0,
}
impl std::marker::Copy for VARIANT_0_0 {}
impl std::clone::Clone for VARIANT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
pub union VARIANT_0_0_0 {
    pub llVal: i64,
    pub lVal: i32,
    pub bVal: u8,
    pub iVal: i16,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: i16,                  // VARIANT_BOOL
    pub __OBSOLETE__VARIANT_BOOL: i16, // VARIANT_BOOL
    pub scode: i32,
    pub cyVal: CY,
    pub date: f64,
    pub bstrVal: *mut u16,               // BSTR
    pub punkVal: *mut std::ffi::c_void,  // IUnknown
    pub pdispVal: *mut std::ffi::c_void, // IDispatch
    pub parray: *mut std::ffi::c_void,   // SAFEARRAY
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub plVal: *mut i32,
    pub pllVal: *mut i64,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut i16,                  // VARIANT_BOOL
    pub __OBSOLETE__VARIANT_PBOOL: *mut i16, // VARIANT_BOOL
    pub pscode: *mut i32,
    pub pcyVal: *mut CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut *mut u16,               // BSTR
    pub ppunkVal: *mut *mut std::ffi::c_void,  // IUnknown
    pub ppdispVal: *mut *mut std::ffi::c_void, // IDispatch
    pub pparray: *mut *mut std::ffi::c_void,   // SAFEARRAY
    pub pvarVal: *mut VARIANT,
    pub byref: *mut std::ffi::c_void,
    pub cVal: u8, // CHAR
    pub uiVal: u16,
    pub ulVal: u32,
    pub ullVal: u64,
    pub intVal: i32,
    pub uintVal: u32,
    pub pdecVal: *mut DECIMAL,
    pub pcVal: ::windows::core::PSTR,
    pub puiVal: *mut u16,
    pub pulVal: *mut u32,
    pub pullVal: *mut u64,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub Anonymous: VARIANT_0_0_0_0,
}
impl std::marker::Copy for VARIANT_0_0_0 {}
impl std::clone::Clone for VARIANT_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
pub struct VARIANT_0_0_0_0 {
    pub pvRecord: *mut std::ffi::c_void,
    pub pRecInfo: *mut std::ffi::c_void, // IRecordInfo
}
impl std::marker::Copy for VARIANT_0_0_0_0 {}
impl std::clone::Clone for VARIANT_0_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
pub union CY {
    pub Anonymous: CY_0,
    pub int64: i64,
}
impl std::marker::Copy for CY {}
impl std::clone::Clone for CY {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
pub struct CY_0 {
    pub Lo: u32,
    pub Hi: i32,
}
impl std::marker::Copy for CY_0 {}
impl std::clone::Clone for CY_0 {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
pub struct DECIMAL {
    pub wReserved: u16,
    pub Anonymous1: DECIMAL_0,
    pub Hi32: u32,
    pub Anonymous2: DECIMAL_1,
}
impl std::marker::Copy for DECIMAL {}
impl std::clone::Clone for DECIMAL {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
pub union DECIMAL_0 {
    pub Anonymous: DECIMAL_0_0,
    pub signscale: u16,
}
impl std::marker::Copy for DECIMAL_0 {}
impl std::clone::Clone for DECIMAL_0 {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
pub struct DECIMAL_0_0 {
    pub scale: u8,
    pub sign: u8,
}
impl std::marker::Copy for DECIMAL_0_0 {}
impl std::clone::Clone for DECIMAL_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
pub union DECIMAL_1 {
    pub Anonymous: DECIMAL_1_0,
    pub Lo64: u64,
}
impl std::marker::Copy for DECIMAL_1 {}
impl std::clone::Clone for DECIMAL_1 {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
pub struct DECIMAL_1_0 {
    pub Lo32: u32,
    pub Mid32: u32,
}
impl std::marker::Copy for DECIMAL_1_0 {}
impl std::clone::Clone for DECIMAL_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
