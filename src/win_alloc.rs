#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::upper_case_acronyms)]

use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;

type HANDLE = *mut c_void;
type DWORD = u32;
type SIZE_T = usize;
type LPVOID = *mut c_void;
type BOOL = i32;

const HEAP_ZERO_MEMORY: DWORD = 0x00000008;

unsafe extern "system" {
    fn GetProcessHeap() -> HANDLE;
    fn HeapAlloc(hHeap: HANDLE, dwFlags: DWORD, dwBytes: SIZE_T) -> LPVOID;
    fn HeapFree(hHeap: HANDLE, dwFlags: DWORD, lpMem: LPVOID) -> BOOL;
    fn HeapReAlloc(hHeap: HANDLE, dwFlags: DWORD, lpMem: LPVOID, dwBytes: SIZE_T) -> LPVOID;
}

pub struct WinAlloc;

unsafe impl GlobalAlloc for WinAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { HeapAlloc(GetProcessHeap(), 0, layout.size()) as *mut u8 }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { HeapFree(GetProcessHeap(), 0, ptr as LPVOID) };
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        unsafe { HeapAlloc(GetProcessHeap(), HEAP_ZERO_MEMORY, layout.size()) as *mut u8 }
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        unsafe { HeapReAlloc(GetProcessHeap(), 0, ptr as LPVOID, new_size) as *mut u8 }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use core::alloc::Layout;
    use core::ptr;

    #[test]
    fn simple() {
        unsafe {
            let layout = Layout::from_size_align(2048, 8).unwrap();
            let ptr = WinAlloc.alloc(layout);
            assert!(!ptr.is_null());

            ptr::write_bytes(ptr, 0x69, 2048);

            for i in 0..1024 {
                assert_eq!(*ptr.add(i), 0x69);
            }

            WinAlloc.dealloc(ptr, layout);
        }
    }

    #[test]
    fn zeroed() {
        unsafe {
            let layout = Layout::from_size_align(2048, 8).unwrap();
            let ptr = WinAlloc.alloc_zeroed(layout);
            assert!(!ptr.is_null());

            for i in 0..2048 {
                assert_eq!(*ptr.add(i), 0);
            }

            WinAlloc.dealloc(ptr, layout);
        }
    }

    #[test]
    fn realloc() {
        unsafe {
            let initial_layout = Layout::from_size_align(256, 8).unwrap();
            let ptr = WinAlloc.alloc(initial_layout);

            ptr::write(ptr as *mut u32, 0x12345678);

            let new_ptr = WinAlloc.realloc(ptr, initial_layout, 512);
            assert!(!new_ptr.is_null());

            assert_eq!(ptr::read(new_ptr as *mut u32), 0x12345678);

            ptr::write_bytes(new_ptr.add(256), 0xBB, 256);

            WinAlloc.dealloc(new_ptr, Layout::from_size_align(512, 8).unwrap());
        }
    }
}
