extern crate libc;

// Creates a new instance of Blake3 Hasher
#[no_mangle]
pub extern fn blake3_new() -> *mut blake3::Hasher {
  return Box::into_raw(Box::new(blake3::Hasher::new()));
}

// Deletes an existing a new instance of Blake3 Hasher
#[no_mangle]
pub extern fn blake3_delete(hasher: *mut blake3::Hasher) {
  unsafe{ Box::from_raw(hasher) };
}

// Resets Blake3 hasher
#[no_mangle]
pub extern fn blake3_reset(
  hasher: *mut blake3::Hasher)
{
  let hasher = unsafe { &mut *hasher };
  hasher.reset();  
}

// Updates Blake3 hash with data
#[no_mangle]
pub extern fn blake3_update(
  hasher: *mut blake3::Hasher,
  ptr: *const u8,
  size: libc::size_t)
{
  let hasher = unsafe { &mut *hasher };
  let slice = unsafe { std::slice::from_raw_parts(ptr as *const u8, size as usize) };
  hasher.update(slice);  
}

// Updates Blake3 hash with data
#[cfg(feature = "rayon")]
#[no_mangle]
pub extern fn blake3_update_with_join(
  hasher: *mut blake3::Hasher,
  ptr: *const u8,
  size: libc::size_t)
{
  let hasher = unsafe { &mut *hasher };
  let slice = unsafe { std::slice::from_raw_parts(ptr as *const u8, size as usize) };
  hasher.update_with_join::<blake3::join::RayonJoin>(slice);  
}

// // Finalize the hash and put the result into output.
// #[no_mangle]
// pub extern fn blake3_finalize(
//   hasher: *mut blake3::Hasher) -> blake3::Hash
// {
//   let hasher = unsafe { &mut *hasher };
//   return hasher.finalize();
// }

// Finalize the hash and put the result into output.
#[no_mangle]
pub extern fn blake3_finalize_xof(
  hasher: *mut blake3::Hasher,
  ptr: *mut u8,
  size: libc::size_t)
{
  let hasher = unsafe { &mut *hasher };
  let slice = unsafe { std::slice::from_raw_parts_mut(ptr as *mut u8, size as usize) };
  hasher.finalize_xof().fill(slice);
}
