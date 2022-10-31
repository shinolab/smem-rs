/*
 * File: simple.rs
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

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    println!("enter q to finish...");

    let created = Arc::new(AtomicBool::new(false));

    let created_clone = Arc::clone(&created);
    let server_th = std::thread::spawn(move || unsafe {
        let mut smem = SMem::new();
        smem.create("test_smem", 65536).unwrap();
        let ptr = smem.map();
        std::ptr::write_volatile(ptr, 0);
        created_clone.store(true, Ordering::SeqCst);
        loop {
            if std::ptr::read_volatile(ptr) != 0x00 {
                let str =
                    std::str::from_utf8_mut(std::slice::from_raw_parts_mut(ptr, 65536)).unwrap();
                print!("Receive: {}", str);
                break;
            }
        }
        smem.unmap();
        smem.close().unwrap();
    });

    let client_th = std::thread::spawn(move || unsafe {
        while !created.load(Ordering::SeqCst) {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        let mut smem = SMem::new();
        smem.create("test_smem", 65536).unwrap();
        let ptr = smem.map();

        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();

        print!("Send: {}", &s);

        std::ptr::copy_nonoverlapping(s.as_ptr(), ptr, s.len());
        smem.unmap();
    });

    server_th.join().unwrap();
    client_th.join().unwrap();
}
