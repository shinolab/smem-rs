/*
 * File: windows.rs
 * Project: src
 * Created Date: 01/11/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;

use windows::{
    core::*,
    Win32::{Foundation::*, System::Memory::*},
};

pub struct SMem {
    handle: HANDLE,
    ptr: *mut u8,
    size: usize,
}

impl SMem {
    pub fn new() -> Self {
        Self {
            handle: INVALID_HANDLE_VALUE,
            ptr: std::ptr::null_mut(),
            size: 0,
        }
    }

    pub fn create(&mut self, name: &str, size: usize) -> Result<()> {
        unsafe {
            self.handle = CreateFileMappingA(
                INVALID_HANDLE_VALUE,
                None,
                PAGE_READWRITE,
                0,
                size as u32,
                PCSTR::from_raw(std::ffi::CString::new(name)?.as_ptr() as *const _),
            )?;
        }
        self.size = size;
        Ok(())
    }

    pub fn map(&mut self) -> *mut u8 {
        unsafe {
            self.ptr = MapViewOfFile(self.handle, FILE_MAP_WRITE | FILE_MAP_READ, 0, 0, self.size)
                as *mut _;
        }
        self.ptr
    }

    pub fn unmap(&mut self) {
        unsafe {
            if self.ptr.is_null() {
                return;
            }
            UnmapViewOfFile(self.ptr as *const _);
            self.ptr = std::ptr::null_mut();
        }
    }

    pub fn close(&mut self) -> Result<()> {
        if self.handle == INVALID_HANDLE_VALUE {
            return Ok(());
        }
        unsafe {
            CloseHandle(self.handle);
        }
        self.handle = INVALID_HANDLE_VALUE;
        self.size = 0;
        Ok(())
    }
}

impl Drop for SMem {
    fn drop(&mut self) {
        self.unmap();
        _ = self.close();
    }
}

impl Default for SMem {
    fn default() -> Self {
        Self::new()
    }
}
