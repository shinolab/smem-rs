/*
 * File: unix.rs
 * Project: src
 * Created Date: 31/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;

use crate::error::SMemError;

pub struct SMem {
    seg_id: i32,
    key_path: String,
    ptr: *mut u8,
}

impl SMem {
    pub fn new() -> Self {
        Self {
            seg_id: -1,
            key_path: String::new(),
            ptr: std::ptr::null_mut(),
        }
    }

    pub fn create(&mut self, name: &str, size: usize) -> Result<()> {
        let home_path = std::env::var("HOME")?;

        self.key_path = format!("{}/{}", home_path, name);

        {
            let mut _file = std::fs::File::create(&self.key_path)?;
        }

        const ID: i32 = 97;
        unsafe {
            let key = nix::libc::ftok(std::ffi::CString::new(self.key_path.as_str())?.as_ptr(), ID);
            if key == -1 {
                return Err(SMemError::GetKeyFailed {
                    errno: nix::errno::errno(),
                }
                .into());
            }

            self.seg_id = nix::libc::shmget(
                key,
                size,
                nix::libc::IPC_CREAT
                    | nix::libc::IPC_EXCL
                    | nix::libc::S_IRUSR as i32
                    | nix::libc::S_IWUSR as i32,
            );
            if self.seg_id == -1 {
                if nix::errno::errno() == 17 {
                    self.seg_id = nix::libc::shmget(key, 0, 0);
                    if self.seg_id == -1 {
                        return Err(SMemError::GetSharedMemoryFailed {
                            errno: nix::errno::errno(),
                        }
                        .into());
                    }
                } else {
                    return Err(SMemError::CreateSharedMemoryFailed {
                        errno: nix::errno::errno(),
                    }
                    .into());
                }
            }
        }
        Ok(())
    }

    pub fn map(&mut self) -> *mut u8 {
        unsafe {
            self.ptr = nix::libc::shmat(self.seg_id, std::ptr::null(), 0) as *mut u8;
        }
        self.ptr
    }

    pub fn unmap(&mut self) {
        if self.ptr == std::ptr::null_mut() {
            return;
        }
        unsafe {
            nix::libc::shmdt(self.ptr as *mut nix::libc::c_void as *const _);
            self.ptr = std::ptr::null_mut();
        }
    }

    pub fn close(&mut self) -> Result<()> {
        if self.seg_id == -1 {
            return Ok(());
        }
        unsafe {
            nix::libc::shmctl(self.seg_id, nix::libc::IPC_RMID, std::ptr::null_mut());
            self.seg_id = -1;
            if std::path::Path::new(&self.key_path).exists() {
                std::fs::remove_file(&self.key_path)?;
            }
        }
        Ok(())
    }
}

impl Drop for SMem {
    fn drop(&mut self) {
        let _ = self.unmap();
        let _ = self.close();
    }
}
