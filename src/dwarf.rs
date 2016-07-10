use std::os::unix::prelude::*;
use std::fs::File;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::os::raw::c_uint;
use dwarf_bindings::*;
use std::ptr;
use std::ffi::CString;
use std::ffi::CStr;

fn dwarf_error() -> *mut *mut Struct_Dwarf_Error_s {
    let mut x: Dwarf_Error = ptr::null::<Struct_Dwarf_Error_s>() as Dwarf_Error;
    &mut x as *mut *mut Struct_Dwarf_Error_s
}

fn indent(level: u32) {
    let mut i = 0;
    while (i < level) {
        i = i + 1;
        print!("  ");

    }
}

fn print_die_data(dbg: Dwarf_Debug, print_me: Dwarf_Die, level: u32) {
    let error_ptr = dwarf_error();
    let mut tag: c_uint = 0;
    let mut tagname = ptr::null::<c_char>() as *const c_char;
    let mut name = ptr::null::<c_char>() as *mut c_char;
    let mut old_name = name;
    unsafe {
        let mut res = dwarf_diename(print_me, &mut name as *mut *mut c_char, error_ptr);
        if (res == DW_DLV_NO_ENTRY) {
            return;
        }
        if (res == DW_DLV_ERROR) {
            panic!("Error in dwarf_diename , level {} \n", level);
        }
        let name2 = CStr::from_ptr(name);
        // this line below is segfaulting
        res = dwarf_tag(print_me, &mut tag as *mut u32 as *mut u16, error_ptr);
        if (res != DW_DLV_OK) {
            panic!("Error in dwarf_tag , level {} \n", level);
        }
        res = dwarf_get_TAG_name(tag, &mut tagname as *mut *const c_char);
        if (res != DW_DLV_OK) {
            panic!("Error in dwarf_get_TAG_name , level {} \n", level);
        }
        indent(level);
        println!("{:?} {:?} tag: {:?} {:?}  name: {:?}",
                 level,
                 name2,
                 tag,
                 tagname,
                 name);
        // dwarf_dealloc(dbg,name as *mut c_void,DW_DLA_STRING);
        name = old_name;
    }
}


fn get_die_and_siblings(dbg: Dwarf_Debug, in_die: Dwarf_Die, in_level: u32) {
    let mut child = ptr::null::<Struct_Dwarf_Die_s>() as Dwarf_Die;
    let mut cur_die = in_die;
    let error = dwarf_error();
    let mut res = DW_DLV_ERROR;
    print_die_data(dbg, in_die, in_level);

    while true {
        let mut sib_die = ptr::null::<Struct_Dwarf_Die_s>() as Dwarf_Die;
        unsafe {
            res = dwarf_child(cur_die, &mut child as *mut Dwarf_Die, error);
            if (res == DW_DLV_ERROR) {
                panic!("oh no {}", in_level);
            }
            if (res == DW_DLV_OK) {
                get_die_and_siblings(dbg, child, in_level + 1);
            }
            res = dwarf_siblingof(dbg,
                                  cur_die,
                                  &mut sib_die as *mut Dwarf_Die,
                                  error as *mut Dwarf_Error);
            print_die_data(dbg, cur_die, in_level);
            if (res == DW_DLV_ERROR) {
                panic!("Error in dwarf_siblingof , level {} \n", in_level);
            }
            if (res == DW_DLV_NO_ENTRY) {
                // Done at this level.
                break;
            }
            // res == DW_DLV_OK
            if (cur_die != in_die) {
                // dwarf_dealloc(dbg,cur_die as *mut c_void,DW_DLA_DIE);
            }
            cur_die = sib_die;
        }
    }
}

fn read_cu_list(dbg: Dwarf_Debug) {
    let mut cu_header_length: Dwarf_Unsigned = 0;
    let mut version_stamp: Dwarf_Half = 0;
    let mut abbrev_offset: Dwarf_Unsigned = 0;
    let mut address_size: Dwarf_Half = 0;
    let mut next_cu_header: Dwarf_Unsigned = 0;
    let mut error: Dwarf_Error = ptr::null::<Struct_Dwarf_Error_s>() as Dwarf_Error;

    let i = 0;
    while true {
        let no_die: Dwarf_Die = ptr::null::<Struct_Dwarf_Die_s>() as Dwarf_Die;
        let mut cu_die: Dwarf_Die = ptr::null::<Struct_Dwarf_Die_s>() as Dwarf_Die;
        unsafe {
            let mut res = DW_DLV_ERROR;
            res = dwarf_next_cu_header(dbg,
                                       &mut cu_header_length,
                                       &mut version_stamp as *mut Dwarf_Half,
                                       &mut abbrev_offset as *mut Dwarf_Unsigned,
                                       &mut address_size as *mut Dwarf_Half,
                                       &mut next_cu_header as *mut Dwarf_Unsigned,
                                       &mut error as *mut *mut Struct_Dwarf_Error_s);
            if res == DW_DLV_ERROR {
                panic!("Error in dwarf_next_cu_header\n");
            }
            if res == DW_DLV_NO_ENTRY {
                println!("done");
                return;
            }
            println!("{}, {}, {}", cu_header_length, address_size, next_cu_header);
            res = dwarf_siblingof(dbg,
                                  no_die,
                                  &mut cu_die as *mut Dwarf_Die,
                                  &mut error as *mut *mut Struct_Dwarf_Error_s);
            if (res == DW_DLV_ERROR) {
                panic!("Error in dwarf_siblingof on CU die \n");
            }
            if (res == DW_DLV_NO_ENTRY) {
                // Impossible case.
                panic!("no entry! in dwarf_siblingof on CU die \n");
            }
            get_die_and_siblings(dbg, cu_die, 0);
        }
    }
}



pub fn do_everything() {
    let mut dbg: Dwarf_Debug = ptr::null::<Struct_Dwarf_Debug_s>() as Dwarf_Debug;
    let errhand: Dwarf_Handler = None;
    let error_ptr = dwarf_error();
    let errarg: Dwarf_Ptr = ptr::null::<c_void> as *mut c_void;
    let file = match File::open("/home/bork/.rbenv/versions/2.1.6/bin/ruby") {
        Err(why) => panic!("couldn't open file sryyyy"),
        Ok(file) => file,
    };
    let fd = file.as_raw_fd() as ::std::os::raw::c_int;
    unsafe {
        let res = dwarf_init(fd,
                             0, // 0 means read
                             errhand,
                             errarg,
                             &mut dbg as *mut *mut Struct_Dwarf_Debug_s,
                             error_ptr);
        if res != DW_DLV_OK {
            panic!("Giving up, cannot do DWARF processing\n");
        }
    };
    read_cu_list(dbg);
    // res = dwarf_finish(dbg,&error);
    // if(res != DW_DLV_OK) {
    // printf("dwarf_finish failed!\n");
    // }
    //
}
