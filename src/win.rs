use std::{ffi::CString, ptr::null_mut, str::from_utf8};
use winapi::{
    shared::minwindef::HKEY,
    um::{
        winnt::{KEY_ALL_ACCESS, REG_SZ},
        winreg::*,
    },
};

pub fn get_path_from_reg() -> Vec<u8> {
    unsafe {
        let mut hkey: HKEY = null_mut();
        let environment_str = CString::new("Environment").unwrap();

        let open_status = RegOpenKeyExA(
            HKEY_CURRENT_USER,
            environment_str.as_ptr(),
            0,
            KEY_ALL_ACCESS,
            &mut hkey,
        );

        // 0 = pog
        // 87 = access denied
        // 2 = no exist
        if open_status != 0 {
            RegCloseKey(hkey);
            panic!("open_status non-zero");
        }

        let mut size: u32 = 6969;
        let mut path_vec: Vec<u8> = vec![0; size as usize];
        let mut reg_value_type = REG_SZ;

        let path_str = CString::new("Path").unwrap();
        let get_status = RegGetValueA(
            HKEY_CURRENT_USER,
            environment_str.as_ptr(),
            path_str.as_ptr(),
            RRF_RT_REG_SZ,
            &mut reg_value_type,
            path_vec.as_ptr() as *const _ as *mut _,
            &mut size,
        );

        if get_status != 0 {
            RegCloseKey(hkey);
            panic!("get_status non-zero");
        }

        RegCloseKey(hkey);

        path_vec.resize(size as usize, 0);
        return path_vec;
    }
}

pub fn set_path_in_reg(path: &mut String) {
    unsafe {
        let mut hkey: HKEY = null_mut();
        let environment_str = CString::new("Environment").unwrap();

        let open_status = RegOpenKeyExA(
            HKEY_CURRENT_USER,
            environment_str.as_ptr(),
            0,
            KEY_ALL_ACCESS,
            &mut hkey,
        );

        // 0 = pog
        // 87 = access denied
        // 2 = no exist
        if open_status != 0 {
            RegCloseKey(hkey);
            panic!("open_status non-zero");
        }

        let mut path_vec: Vec<u8> = path.as_bytes_mut().to_vec();
        path_vec.resize(path_vec.len() + 1, 0);
        println!("path_vec as str = {:?}", from_utf8(&path_vec));

        let size = path_vec.len();

        let path_str = CString::new("Path").unwrap();
        let set_status = RegSetValueExA(
            hkey,
            path_str.as_ptr(),
            0,
            REG_SZ,
            path_vec.as_ptr(),
            size as u32
        );

        if set_status != 0 {
            RegCloseKey(hkey);
            panic!("set_status non-zero");
        }

        RegCloseKey(hkey);
    }
}