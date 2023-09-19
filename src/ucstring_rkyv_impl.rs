#![allow(unused_imports,unused_variables,unreachable_code,dead_code,non_upper_case_globals,unused_mut)]
use crate::ucstring_rkyv::U16CStringResolver;
use crate::ucstring_rkyv::ArchivedU16CString;
use crate::ucstring::{U16CString, U32CString, WideCString};

use crate::{
  // U16Str,U16String,        	// U16String and U32String, on the other hand, are similar to (but not the same as), OsString, and are designed around working with FFI. Unlike the UTF variants, these strings do not have a defined encoding, and can work with any wide character strings, regardless of the encoding. They can be converted to and from OsString (but may require an encoding conversion depending on the platform), although that string type is an OS-specified encoding, so take special care.
  // WideString ,WideChar,    	// alias for u16|u32 to match C wchar_t size (per platform)
  // WideCString,WideCStr,    	// aliases U16CString or U32CString
  // U16CString ,U16CStr,     	// U16/U32-CString wide version of the standard CString type
  // Utf16Str   ,Utf16String, 	// UTF-16 encoded, growable owned string
  // u16str,u16cstr,utf16str  	// macros
};
use rkyv::{
  ffi::{ArchivedCString,CStringResolver},
  // primitive::ArchivedUsize,
  FixedUsize,
  to_archived,
  ser::Serializer,
  Archive, ArchivePointee, ArchiveUnsized, Archived, ArchivedMetadata,
  Deserialize, DeserializeUnsized, Fallible, Serialize, SerializeUnsized,
};
use core::{alloc::Layout, ptr};
use ptr_meta::Pointee;
use std::alloc;
use std::ffi::{CStr, CString};

// U16CString
impl PartialEq<U16CString> for ArchivedU16CString {
  #[inline] fn eq(&self, other: &U16CString) -> bool {
    PartialEq::eq(self.as_ucstr(), other.as_ucstr())
  }
}
impl PartialEq<ArchivedU16CString> for U16CString {
  #[inline] fn eq(&self, other: &ArchivedU16CString) -> bool {
    PartialEq::eq(other.as_ucstr(), self.as_ucstr())
  }
}
impl Archive for U16CString {
  type Archived = ArchivedU16CString;
  type Resolver = U16CStringResolver;
  #[inline] unsafe fn resolve(&self,
    pos     	: usize,
    resolver	:      Self::Resolver,
    out     	: *mut Self::Archived,) {
    ArchivedU16CString::resolve_from_c_str(self.as_ucstr(),
      pos,
      resolver,
      out,
    );
  }
}
impl<S: Serializer + ?Sized> Serialize<S> for U16CString {
  #[inline] fn serialize(&self,
    serializer: &mut S,) -> Result<Self::Resolver, S::Error> {
    ArchivedU16CString::serialize_from_c_str(self.as_ucstr(), serializer)
  }
}

impl<D: Fallible + ?Sized> Deserialize<U16CString, D> for Archived<U16CString>
  where CStr: DeserializeUnsized<CStr, D> {
  #[inline] fn deserialize(&self, deserializer: &mut D) -> Result<U16CString, D::Error> {
    unsafe {
      let data_address = self.as_ucstr().deserialize_unsized (deserializer, |layout| {
          alloc::alloc(layout)})?;
      let metadata     = self.as_ucstr().deserialize_metadata(deserializer)?;
      let ptr = ptr_meta::from_raw_parts_mut(data_address, metadata);
      Ok(Box::<CStr>::from_raw(ptr).into())
    }
  }
}

pub fn test_rkyv_new_type() {
  // let mut serializer = AlignedSerializer::new(AlignedVec::new());
  // const STR_VAL: &'static str = "I'm in an OwnedStr!";
  // let value = OwnedStr { inner: STR_VAL };
  // // It works!
  // serializer.serialize_value(&value).expect("failed to archive test");
  // let buf = serializer.into_inner();
  // let archived = unsafe { archived_root::<OwnedStr>(buf.as_ref()) };
  // // Let's make sure our data got written correctly
  // assert_eq!(archived.as_str(), STR_VAL);
}
