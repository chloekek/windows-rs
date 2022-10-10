pub mod Providers;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WidgetSize(pub i32);
impl WidgetSize {
    pub const Small: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const Large: Self = Self(2i32);
}
impl ::core::marker::Copy for WidgetSize {}
impl ::core::clone::Clone for WidgetSize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WidgetSize {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WidgetSize {
    type Abi = Self;
}
impl ::core::fmt::Debug for WidgetSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WidgetSize").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WidgetSize {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Microsoft.Windows.Widgets.WidgetSize;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
::core::include!("impl.rs");
