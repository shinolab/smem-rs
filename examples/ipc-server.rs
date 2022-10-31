/*
 * File: ipc-server.rs
 * Project: examples
 * Created Date: 31/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use smem::*;

fn main() {
    let mut smem = SMem::new();
    smem.create("test_smem", 65536).unwrap();
    let ptr = smem.map();
    unsafe {
        std::ptr::write_volatile(ptr, 0);
        loop {
            if std::ptr::read_volatile(ptr) == 0x00 {
                continue;
            }
            let str =
                std::str::from_utf8_mut(std::slice::from_raw_parts_mut(ptr.add(1), 65535)).unwrap();
            print!("Receive: {}", str);
            if str == "q" {
                break;
            }
            std::ptr::write_volatile(ptr, 0x00);
        }

        smem.unmap();
        smem.close().unwrap();
    }
}
