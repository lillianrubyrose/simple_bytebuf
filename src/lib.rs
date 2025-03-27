#![no_std]
extern crate alloc;

use crate::win_alloc::WinAlloc;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ptr;

mod win_alloc;

#[global_allocator]
static GLOBAL: WinAlloc = WinAlloc;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[derive(Debug)]
pub enum ByteBufError {
    Success = 0,
    OutOfBounds,
    NullPointer,
    InvalidLength,
}

pub struct ByteBuf {
    inner: Vec<u8>,

    rp: usize,
    wp: usize,
}

impl ByteBuf {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
            rp: 0,
            wp: 0,
        }
    }

    pub fn from_slice(slice: &[u8]) -> Self {
        Self {
            inner: slice.to_vec(),
            rp: 0,
            wp: 0,
        }
    }

    pub fn read_exact<const N: usize>(&mut self) -> Result<[u8; N], ByteBufError> {
        let available = self.inner.len().saturating_sub(self.rp);

        if available < N {
            return Err(ByteBufError::OutOfBounds);
        }

        let mut arr = [0u8; N];

        // SAFETY - Bounds checking
        unsafe {
            ptr::copy_nonoverlapping(self.inner.as_ptr().add(self.rp), arr.as_mut_ptr(), N);
        }

        self.rp += N;

        Ok(arr)
    }

    pub fn write_slice(&mut self, slice: &[u8]) {
        let adj_len = self.wp + slice.len();
        if adj_len > self.inner.len() {
            self.inner.reserve(slice.len());
            self.inner.resize(adj_len, 0);
        }

        // SAFETY - We ensure there's enough space in the Vec
        unsafe {
            ptr::copy_nonoverlapping(
                slice.as_ptr(),
                self.inner.as_mut_ptr().add(self.wp),
                slice.len(),
            );
        }

        self.wp = adj_len;
    }

    pub fn read_u8(&mut self) -> Result<u8, ByteBufError> {
        Ok(self.read_exact::<1>()?[0])
    }

    pub fn read_u16(&mut self) -> Result<u16, ByteBufError> {
        Ok(u16::from_ne_bytes(self.read_exact::<2>()?))
    }

    pub fn read_u32(&mut self) -> Result<u32, ByteBufError> {
        Ok(u32::from_ne_bytes(self.read_exact::<4>()?))
    }

    pub fn read_u64(&mut self) -> Result<u64, ByteBufError> {
        Ok(u64::from_ne_bytes(self.read_exact::<8>()?))
    }

    pub fn read_i8(&mut self) -> Result<i8, ByteBufError> {
        Ok(self.read_exact::<1>()?[0] as i8)
    }

    pub fn read_i16(&mut self) -> Result<i16, ByteBufError> {
        Ok(i16::from_ne_bytes(self.read_exact::<2>()?))
    }

    pub fn read_i32(&mut self) -> Result<i32, ByteBufError> {
        Ok(i32::from_ne_bytes(self.read_exact::<4>()?))
    }

    pub fn read_i64(&mut self) -> Result<i64, ByteBufError> {
        Ok(i64::from_ne_bytes(self.read_exact::<8>()?))
    }

    pub fn read_f32(&mut self) -> Result<f32, ByteBufError> {
        Ok(f32::from_ne_bytes(self.read_exact::<4>()?))
    }

    pub fn read_f64(&mut self) -> Result<f64, ByteBufError> {
        Ok(f64::from_ne_bytes(self.read_exact::<8>()?))
    }

    pub fn write_u8(&mut self, val: u8) {
        self.write_slice(&val.to_ne_bytes());
    }

    pub fn write_u16(&mut self, val: u16) {
        self.write_slice(&val.to_ne_bytes());
    }

    pub fn write_u32(&mut self, val: u32) {
        self.write_slice(&val.to_ne_bytes());
    }

    pub fn write_u64(&mut self, val: u64) {
        self.write_slice(&val.to_ne_bytes());
    }

    pub fn write_i8(&mut self, val: i8) {
        self.write_slice(&val.to_ne_bytes());
    }

    pub fn write_i16(&mut self, val: i16) {
        self.write_slice(&val.to_ne_bytes());
    }

    pub fn write_i32(&mut self, val: i32) {
        self.write_slice(&val.to_ne_bytes());
    }

    pub fn write_i64(&mut self, val: i64) {
        self.write_slice(&val.to_ne_bytes());
    }

    pub fn write_f32(&mut self, val: f32) {
        self.write_slice(&val.to_ne_bytes());
    }

    pub fn write_f64(&mut self, val: f64) {
        self.write_slice(&val.to_ne_bytes());
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn bytebuf_create(capacity: usize) -> *mut ByteBuf {
    let buf = Box::new(ByteBuf::with_capacity(capacity));
    Box::into_raw(buf)
}

/// # Safety
///
/// We trust the caller to provide valid pointer and length
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bytebuf_from_slice(data: *const u8, len: usize) -> *mut ByteBuf {
    if data.is_null() {
        return ptr::null_mut();
    }

    let slice = unsafe { core::slice::from_raw_parts(data, len) };
    let buf = Box::new(ByteBuf::from_slice(slice));
    Box::into_raw(buf)
}

/// # Safety
///
/// We trust the caller to provide a pointer created with bytebuf_create or bytebuf_from_slice
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bytebuf_destroy(buf: *mut ByteBuf) {
    if !buf.is_null() {
        unsafe {
            drop(Box::from_raw(buf));
        }
    }
}

/// # Safety
///
/// We trust the caller to provide a valid buffer pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bytebuf_read_u8(buf: *mut ByteBuf, out: *mut u8) -> u8 {
    if buf.is_null() {
        return ByteBufError::NullPointer as u8;
    }

    unsafe {
        match (*buf).read_u8() {
            Ok(value) => {
                *out = value;
                ByteBufError::Success as u8
            }
            Err(err) => err as u8,
        }
    }
}

/// # Safety
///
/// We trust the caller to provide a valid buffer pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bytebuf_read_u16(buf: *mut ByteBuf, out: *mut u16) -> u8 {
    if buf.is_null() {
        return ByteBufError::NullPointer as u8;
    }

    unsafe {
        match (*buf).read_u16() {
            Ok(value) => {
                *out = value;
                ByteBufError::Success as u8
            }
            Err(err) => err as u8,
        }
    }
}

/// # Safety
///
/// We trust the caller to provide a valid buffer pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bytebuf_read_u32(buf: *mut ByteBuf, out: *mut u32) -> u8 {
    if buf.is_null() {
        return ByteBufError::NullPointer as u8;
    }

    unsafe {
        match (*buf).read_u32() {
            Ok(value) => {
                *out = value;
                ByteBufError::Success as u8
            }
            Err(err) => err as u8,
        }
    }
}

/// # Safety
///
/// We trust the caller to provide a valid buffer pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bytebuf_read_u64(buf: *mut ByteBuf, out: *mut u64) -> u8 {
    if buf.is_null() {
        return ByteBufError::NullPointer as u8;
    }

    unsafe {
        match (*buf).read_u64() {
            Ok(value) => {
                *out = value;
                ByteBufError::Success as u8
            }
            Err(err) => err as u8,
        }
    }
}

/// # Safety
///
/// We trust the caller to provide a valid buffer pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bytebuf_read_i8(buf: *mut ByteBuf, out: *mut i8) -> u8 {
    if buf.is_null() {
        return ByteBufError::NullPointer as u8;
    }

    unsafe {
        match (*buf).read_i8() {
            Ok(value) => {
                *out = value;
                ByteBufError::Success as u8
            }
            Err(err) => err as u8,
        }
    }
}

/// # Safety
///
/// We trust the caller to provide a valid buffer pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bytebuf_read_i16(buf: *mut ByteBuf, out: *mut i16) -> u8 {
    if buf.is_null() {
        return ByteBufError::NullPointer as u8;
    }

    unsafe {
        match (*buf).read_i16() {
            Ok(value) => {
                *out = value;
                ByteBufError::Success as u8
            }
            Err(err) => err as u8,
        }
    }
}

/// # Safety
///
/// We trust the caller to provide a valid buffer pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bytebuf_read_i32(buf: *mut ByteBuf, out: *mut i32) -> u8 {
    if buf.is_null() {
        return ByteBufError::NullPointer as u8;
    }

    unsafe {
        match (*buf).read_i32() {
            Ok(value) => {
                *out = value;
                ByteBufError::Success as u8
            }
            Err(err) => err as u8,
        }
    }
}

/// # Safety
///
/// We trust the caller to provide a valid buffer pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bytebuf_read_i64(buf: *mut ByteBuf, out: *mut i64) -> u8 {
    if buf.is_null() {
        return ByteBufError::NullPointer as u8;
    }

    unsafe {
        match (*buf).read_i64() {
            Ok(value) => {
                *out = value;
                ByteBufError::Success as u8
            }
            Err(err) => err as u8,
        }
    }
}

/// # Safety
///
/// We trust the caller to provide a valid buffer pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bytebuf_read_f32(buf: *mut ByteBuf, out: *mut f32) -> u8 {
    if buf.is_null() {
        return ByteBufError::NullPointer as u8;
    }

    unsafe {
        match (*buf).read_f32() {
            Ok(value) => {
                *out = value;
                ByteBufError::Success as u8
            }
            Err(err) => err as u8,
        }
    }
}

/// # Safety
///
/// We trust the caller to provide a valid buffer pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bytebuf_read_f64(buf: *mut ByteBuf, out: *mut f64) -> u8 {
    if buf.is_null() {
        return ByteBufError::NullPointer as u8;
    }

    unsafe {
        match (*buf).read_f64() {
            Ok(value) => {
                *out = value;
                ByteBufError::Success as u8
            }
            Err(err) => err as u8,
        }
    }
}

/// # Safety
///
/// We trust the caller to provide a valid buffer pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bytebuf_write_u8(buf: *mut ByteBuf, val: u8) {
    if buf.is_null() {
        return;
    }

    unsafe { (*buf).write_u8(val) }
}

/// # Safety
///
/// We trust the caller to provide a valid buffer pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bytebuf_write_u16(buf: *mut ByteBuf, val: u16) {
    if buf.is_null() {
        return;
    }

    unsafe { (*buf).write_u16(val) }
}

/// # Safety
///
/// We trust the caller to provide a valid buffer pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bytebuf_write_u32(buf: *mut ByteBuf, val: u32) {
    if buf.is_null() {
        return;
    }

    unsafe { (*buf).write_u32(val) }
}

/// # Safety
///
/// We trust the caller to provide a valid buffer pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bytebuf_write_u64(buf: *mut ByteBuf, val: u64) {
    if buf.is_null() {
        return;
    }

    unsafe { (*buf).write_u64(val) }
}

/// # Safety
///
/// We trust the caller to provide a valid buffer pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bytebuf_write_i8(buf: *mut ByteBuf, val: i8) {
    if buf.is_null() {
        return;
    }

    unsafe { (*buf).write_i8(val) }
}

/// # Safety
///
/// We trust the caller to provide a valid buffer pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bytebuf_write_i16(buf: *mut ByteBuf, val: i16) {
    if buf.is_null() {
        return;
    }

    unsafe { (*buf).write_i16(val) }
}

/// # Safety
///
/// We trust the caller to provide a valid buffer pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bytebuf_write_i32(buf: *mut ByteBuf, val: i32) {
    if buf.is_null() {
        return;
    }

    unsafe { (*buf).write_i32(val) }
}

/// # Safety
///
/// We trust the caller to provide a valid buffer pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bytebuf_write_i64(buf: *mut ByteBuf, val: i64) {
    if buf.is_null() {
        return;
    }

    unsafe { (*buf).write_i64(val) }
}

/// # Safety
///
/// We trust the caller to provide a valid buffer pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bytebuf_write_f32(buf: *mut ByteBuf, val: f32) {
    if buf.is_null() {
        return;
    }

    unsafe { (*buf).write_f32(val) }
}

/// # Safety
///
/// We trust the caller to provide a valid buffer pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bytebuf_write_f64(buf: *mut ByteBuf, val: f64) {
    if buf.is_null() {
        return;
    }

    unsafe { (*buf).write_f64(val) }
}

/// # Safety
///
/// We trust the caller to provide a valid buffer pointer and data pointer + length
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bytebuf_write_slice(buf: *mut ByteBuf, data: *const u8, len: usize) {
    if buf.is_null() || data.is_null() {
        return;
    }

    let slice = unsafe { core::slice::from_raw_parts(data, len) };
    unsafe { (*buf).write_slice(slice) }
}

/// # Safety
///
/// Proper bounds checking
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bytebuf_read_exact(buf: *mut ByteBuf, out: *mut u8, len: usize) -> bool {
    if buf.is_null() || out.is_null() {
        return false;
    }

    let rp = unsafe { (*buf).rp };
    let available = unsafe { (*buf).inner.len() - rp };

    if available < len {
        return false;
    }

    unsafe {
        ptr::copy_nonoverlapping((*buf).inner.as_ptr().add((*buf).rp), out, len);
        (*buf).rp += len;
    }

    true
}

/// # Safety
///
/// We trust the caller to provide a valid buffer pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bytebuf_len(buf: *const ByteBuf) -> usize {
    if buf.is_null() {
        return 0;
    }

    unsafe { (*buf).inner.len() }
}

#[cfg(test)]
mod tests {
    use crate::ByteBuf;

    macro_rules! define_rw_test {
        ($name:ident, $ty:ident, $write:ident, $read:ident) => {
            #[test]
            fn $name() {
                let mut buffer = ByteBuf::with_capacity(5); // Arbitrary number
                buffer.$write($ty::default());

                assert_eq!(buffer.$read().unwrap(), $ty::default());

                buffer.$write($ty::MAX);
                buffer.$write($ty::MIN);

                assert_eq!(buffer.$read().unwrap(), $ty::MAX);
                assert_eq!(buffer.$read().unwrap(), $ty::MIN);
            }
        };
    }

    define_rw_test!(rw_u8, u8, write_u8, read_u8);
    define_rw_test!(rw_u16, u16, write_u16, read_u16);
    define_rw_test!(rw_u32, u32, write_u32, read_u32);
    define_rw_test!(rw_u64, u64, write_u64, read_u64);

    define_rw_test!(rw_i8, i8, write_i8, read_i8);
    define_rw_test!(rw_i16, i16, write_i16, read_i16);
    define_rw_test!(rw_i32, i32, write_i32, read_i32);
    define_rw_test!(rw_i64, i64, write_i64, read_i64);

    define_rw_test!(rw_f32, f32, write_f32, read_f32);
    define_rw_test!(rw_f64, f64, write_f64, read_f64);
}
