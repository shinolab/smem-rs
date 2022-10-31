/*
 * File: ipc-server.rs
 * Project: examples
 * Created Date: 31/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use smem::*;
use std::io::Write;

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
            let len = (0..65536)
                .find(|&x| std::ptr::read_volatile(ptr.add(x)) == 0x00)
                .unwrap();
            let s = std::str::from_utf8_mut(std::slice::from_raw_parts_mut(ptr.add(1), len - 1))
                .unwrap();
            print!("Receive: {}", s);
            std::io::stdout().flush().unwrap();

            if s.strip_suffix("\r\n")
                .or_else(|| s.strip_suffix('\n'))
                .unwrap_or(s)
                == "q"
            {
                break;
            }
            std::ptr::write_volatile(ptr, 0x00);
        }

        smem.unmap();
        smem.close().unwrap();
    }
}
