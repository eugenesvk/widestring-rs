#![allow(unused_imports,unused_variables,unreachable_code,dead_code,non_upper_case_globals,unused_mut)]
use crate::{U16CStr};

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

// impl Pointee for U16CStr { type Metadata = usize; }

// ArchivePointee is a sealed trait
// U16CStr
impl ArchiveUnsized for U16CStr {
  type Archived        	= U16CStr;
  type MetadataResolver	= ();
  #[inline] unsafe fn resolve_metadata(
    &self,
    _  	: usize,
    _  	: Self::MetadataResolver,
    out	: *mut ArchivedMetadata<Self>,) {
    // out.write(ArchivedUsize::from_native(ptr_meta::metadata(self) as _))
    out.write(to_archived!(ptr_meta::metadata(self) as FixedUsize))
  }
}
/*
impl ArchivePointee for U16CStr {
  type ArchivedMetadata = Archived<usize>;
  #[inline] fn pointer_metadata(archived: &Self::ArchivedMetadata) -> <Self as Pointee>::Metadata {
    <[u16]>::pointer_metadata(archived)
  }
}
impl<S: Serializer + ?Sized> SerializeUnsized<S> for U16CStr {
  #[inline] fn serialize_unsized(&self, serializer: &mut S) -> Result<usize, S::Error> {
    let result = serializer.pos();
    serializer.write(self.to_bytes_with_nul())?;
    Ok(result)
  }
  #[inline] fn serialize_metadata(
    &self,
    _: &mut S,
  ) -> Result<Self::MetadataResolver, S::Error> {
    Ok(())
  }
}
*/

impl<D: Fallible + ?Sized> DeserializeUnsized<U16CStr, D>
  for <U16CStr as ArchiveUnsized>::Archived {
  #[inline] unsafe fn deserialize_unsized(&self,
    _: &mut D,
    mut alloc: impl FnMut(Layout) -> *mut u16,
  ) -> Result<*mut (), D::Error> {
    let slice = self.to_bytes_with_nul();
    let bytes = alloc(Layout::array::<u16>(slice.len()).unwrap());
    assert!(!bytes.is_null());
    ptr::copy_nonoverlapping(slice.as_ptr(), bytes, slice.len());
    Ok(bytes.cast())
  }
  #[inline] fn deserialize_metadata(&self,
    _: &mut D,
  ) -> Result<<U16CStr as Pointee>::Metadata, D::Error> {
    Ok(ptr_meta::metadata(self))
  }
}

