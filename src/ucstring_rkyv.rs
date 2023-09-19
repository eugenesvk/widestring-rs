//! Archived versions of FFI types
#![allow(unused_imports,unused_variables,unreachable_code,dead_code,non_upper_case_globals,unused_mut)]
use crate::{
  U16Str,U16String,         	// U16String and U32String, on the other hand, are similar to (but not the same as), OsString, and are designed around working with FFI. Unlike the UTF variants, these strings do not have a defined encoding, and can work with any wide character strings, regardless of the encoding. They can be converted to and from OsString (but may require an encoding conversion depending on the platform), although that string type is an OS-specified encoding, so take special care
  WideString ,WideChar,     	// alias for u16|u32 to match C wchar_t size (per platform)
  WideCString,WideCStr,     	// aliases U16CString or U32CString
  U16CString ,U16CStr,      	// U16/U32-CString wide version of the standard CString type
  Utf16Str   ,Utf16String,  	// UTF-16 encoded, growable owned string
  // u16str,u16cstr,utf16str	// macros
};
use rkyv::{ser::Serializer,ArchiveUnsized,MetadataResolver,RelPtr,SerializeUnsized,out_field};
use core::{borrow::Borrow,cmp, fmt, hash,ops::{Deref, Index, RangeFull},pin::Pin,};
use std::ffi::CStr;
use crate::ucstring_rkyv_impl::*;

/// An archived [`U16CString`](std::ffi::U16CString). Uses a [`RelPtr`] to a `U16CStr` under the hood
#[repr(transparent)] pub struct ArchivedU16CString(RelPtr<U16CStr>);

impl ArchivedU16CString {
  /// Returns the contents of this U16CString as a slice of bytes
  /// The returned slice does **not** contain the trailing nul terminator, and it is guaranteed to not have any interior nul bytes. If you need the nul terminator, use as_bytes_with_nul`][ArchivedU16CString::as_bytes_with_nul()]
  #[inline] pub fn as_bytes         (&self) -> &[u16] {            self.as_c_str().to_bytes()  }
  /// Same as [`as_bytes`][ArchivedU16CString::as_bytes()] but includes the trailing nul terminator
  #[inline] pub fn as_bytes_with_nul(&self) -> &[u16] {            self.as_c_str().to_bytes_with_nul()}
  /// Extracts a `U16CStr` slice containing the entire string
  #[inline] pub fn as_c_str         (&self) -> &U16CStr {unsafe { &*self.0.as_ptr() } }
  /// Extracts a pinned mutable `U16CStr` slice containing the entire string
  #[inline] pub fn pin_mut_c_str    ( self: Pin<&mut Self>) -> Pin<&mut U16CStr> {
    unsafe { self.map_unchecked_mut(|s| &mut *s.0.as_mut_ptr()) } }

  /// Resolves an archived C string from the given C string and parameters
  /// # Safety
  /// - `pos` must be the position of `out` within the archive
  /// - `resolver` must be the result of serializing a C string
  #[inline] pub unsafe fn resolve_from_c_str(c_str:&U16CStr,
    pos     	: usize,
    resolver	: U16CStringResolver,
    out     	: *mut Self,) {
    let (fp, fo) = out_field!(out.0);
    #[allow(clippy::unit_arg)] c_str.resolve_unsized( // metadata_resolver is guaranteed to be (), but it's better to be explicit about it
      pos + fp,
      resolver.pos,
      resolver.metadata_resolver,
      fo,
    );
  }

  /// Serializes a C string
  #[inline] pub fn serialize_from_c_str<S: Serializer + ?Sized>(c_str:&U16CStr,serializer:&mut S) -> Result<U16CStringResolver, S::Error> {
    Ok(U16CStringResolver {
      pos              	: c_str.serialize_unsized (serializer)?,
      metadata_resolver	: c_str.serialize_metadata(serializer)?,})
  }
}

impl AsRef< U16CStr> for ArchivedU16CString {          fn as_ref(&self) -> &U16CStr         {self.as_c_str()}}
impl Borrow<U16CStr> for ArchivedU16CString {#[inline] fn borrow(&self) -> &U16CStr         {self.as_c_str()}}
impl fmt::Debug   for ArchivedU16CString {#[inline] fn fmt   (&self, f: &mut fmt::Formatter<'_>)
 ->                                                            fmt::Result         {self.as_c_str().fmt(f)}}
impl Deref for ArchivedU16CString { type Target = U16CStr;
                                       #[inline] fn deref (&self) -> &Self::Target {self.as_c_str()}}
impl Eq for ArchivedU16CString {}
impl hash::Hash for ArchivedU16CString {
  #[inline] fn hash<H: hash::Hasher>(&self, state: &mut H) {self.as_bytes_with_nul().hash(state);}}

impl Index<RangeFull> for ArchivedU16CString {
  type Output = U16CStr;
  #[inline] fn index(&self, _: RangeFull) -> &Self::Output {self.as_c_str()}}
impl Ord                        for ArchivedU16CString {#[inline] fn cmp(&self,other:&Self)  -> cmp::Ordering {self.as_bytes().cmp(other.as_bytes())}}
impl PartialEq                  for ArchivedU16CString {#[inline] fn eq (&self,other:&Self)  -> bool          {self.as_bytes() == other.as_bytes()}}
impl PartialEq<&U16CStr>           for ArchivedU16CString {#[inline] fn eq (&self,other:&&U16CStr) -> bool          {PartialEq::eq(self.as_c_str(), other)}}
impl PartialEq<ArchivedU16CString> for &U16CStr           {#[inline] fn eq (&self,other:&ArchivedU16CString)
 -> bool {PartialEq::eq(other.as_c_str(), self)}}
impl PartialOrd for ArchivedU16CString {#[inline] fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    self.as_bytes().partial_cmp(other.as_bytes())}}

/// The resolver for `U16CString`
pub struct U16CStringResolver {
  pos              	: usize,
  metadata_resolver	: MetadataResolver<U16CStr>,
}

// #[cfg(feature = "validation")]
// const _: () = {
//   use rkyv::validation::{owned::{CheckOwnedPointerError,OwnedPointerError},ArchiveContext,};
//   use bytecheck::{CheckBytes, Error};

//   impl<C: ArchiveContext + ?Sized> CheckBytes<C> for ArchivedU16CString
//   where C::Error: Error, {
//     type Error = CheckOwnedPointerError<U16CStr, C>;

//     #[inline] unsafe fn check_bytes<'a>(
//       value  	: *const Self,
//       context	: &mut C,
//     ) -> Result<&'a Self, Self::Error> {
//       let rel_ptr =
//         RelPtr::<U16CStr>::manual_check_bytes(value.cast(), context)
//           .map_err(OwnedPointerError::PointerCheckBytesError)?;
//       let ptr   = context.check_subtree_rel_ptr(rel_ptr).map_err(OwnedPointerError::ContextError)?;
//       let range = context.push_prefix_subtree  (    ptr).map_err(OwnedPointerError::ContextError)?;
//       U16CStr::check_bytes(ptr,context)                    .map_err(OwnedPointerError::ValueCheckBytesError)?;
//       context            .pop_prefix_range     (range  ).map_err(OwnedPointerError::ContextError)?;

//       Ok(&*value)
//     }
//   }
// };
