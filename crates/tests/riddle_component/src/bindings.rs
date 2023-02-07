#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, clippy::all)]
#[repr(C)]
pub struct TypeName {
    pub a: i32,
    pub b: f32,
    pub c: u64,
    pub d: f64,
}
impl ::core::marker::Copy for TypeName {}
impl ::core::clone::Clone for TypeName {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TypeName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TypeName").field("a", &self.a).field("b", &self.b).field("c", &self.c).field("d", &self.d).finish()
    }
}
unsafe impl ::windows::core::Abi for TypeName {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TypeName {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Component.TypeName;i4;f4;u8;f8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for TypeName {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.c == other.c && self.d == other.d
    }
}
impl ::core::cmp::Eq for TypeName {}
impl ::core::default::Default for TypeName {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
