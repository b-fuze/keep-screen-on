use winapi::um::powersetting::PowerGetActiveScheme;
use winapi::um::powersetting::PowerSetActiveScheme;
use winapi::um::powersetting::PowerWriteACValueIndex;
use winapi::um::powrprof::GetActivePwrScheme;
use winapi::um::powrprof::PowerEnumerate;
use winapi::um::powrprof::PowerReadFriendlyName;
use winapi::um::powrprof::PowerReadACValueIndex;
use winapi::um::powrprof::PowerReadSettingAttributes;
use winapi::shared::guiddef::GUID;
use winapi::um::winnt::GUID_VIDEO_SUBGROUP;
use std::ffi::CStr;

unsafe fn utf8_from_utf16(raw: &Vec<u8>) -> &str {
    let cstr: Vec<i8> = std::mem::transmute::<&Vec<u8>, &Vec<u16>>(&raw).iter().map(|n| *n as i8).collect();
    let ptr = cstr.as_ptr();
    std::mem::forget(cstr);
    CStr::from_ptr(ptr).to_str().unwrap()
}

fn main() {
    unsafe {
        let mut ui_id = 0u32;
        GetActivePwrScheme(&mut ui_id);

        let mut guid_ptr: *mut GUID = std::ptr::null_mut();
        let success = PowerGetActiveScheme(std::ptr::null_mut(), &mut guid_ptr);

        if success != 0 {
            panic!("PowerGetActiveScheme: failed to get active scheme");
        }

        // Enumerate settings
        let mut index = 0u32;
        let mut sett_guid_buf: Vec<u8> = Vec::with_capacity(16);
        let mut buf_size = sett_guid_buf.capacity() as u32;
        let sett_guid_buf_ptr = sett_guid_buf.as_mut_ptr();
        sett_guid_buf.set_len(sett_guid_buf.capacity());

        let mut name_buf: Vec<u8> = Vec::with_capacity(256);
        let mut name_buf_size = name_buf.capacity() as u32;
        let name_buf_ptr = name_buf.as_mut_ptr();
        name_buf.set_len(name_buf.capacity());

        loop {
            let result = PowerEnumerate(
                std::ptr::null_mut(),
                guid_ptr,
                &GUID_VIDEO_SUBGROUP,
                18u32,
                index,
                sett_guid_buf_ptr,
                &mut buf_size,
            );

            if result == 0 {
                PowerReadFriendlyName(
                    std::ptr::null_mut(),
                    guid_ptr,
                    &GUID_VIDEO_SUBGROUP,
                    sett_guid_buf_ptr as *const GUID,
                    name_buf_ptr,
                    &mut name_buf_size,
                );

                let attr = PowerReadSettingAttributes(
                    &GUID_VIDEO_SUBGROUP,
                    sett_guid_buf_ptr as *const GUID,
                );

                let mut idx = 0u32;
                PowerReadACValueIndex(
                    std::ptr::null_mut(),
                    guid_ptr,
                    &GUID_VIDEO_SUBGROUP,
                    sett_guid_buf_ptr as *const GUID,
                    &mut idx,
                );

                let sett_name = utf8_from_utf16(&name_buf);
                if sett_name.starts_with("Dim display after") && attr == 2 {
                    println!("Old value: {}", idx);
                    let setting_success = PowerWriteACValueIndex(
                        std::ptr::null_mut(),
                        guid_ptr,
                        &GUID_VIDEO_SUBGROUP,
                        sett_guid_buf_ptr as *const GUID,
                        0u32,
                    );

                    let scheme_success = PowerSetActiveScheme(
                        std::ptr::null_mut(),
                        guid_ptr,
                    );

                    if setting_success == 0 && scheme_success == 0 {
                        println!("Success");
                    } else {
                        println!("Failed:\n  Setting: {}\n  Scheme: {}", setting_success, scheme_success);
                    }
                }
            } else {
                if index == 0 {
                    println!("Failed! {}", index);
                }

                break;
            }
            index += 1;
        }
    }
}

