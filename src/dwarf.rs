#![warn(unused_parens)]

use std::os::unix::prelude::*;
use std::fs::File;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::os::raw::c_uint;
use std::slice::from_raw_parts;
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

fn my_dwarf_get_FORM_name(tag: c_uint) -> *const c_char {
    let mut tagname = ptr::null::<c_char>() as *const c_char;
    unsafe {
        let res = dwarf_get_FORM_name(tag as u32, &mut tagname as *mut *const c_char);
        if (res != DW_DLV_OK) {
            panic!("Error in dwarf_get_FORM_name\n");
        }
    }
    tagname
}
fn my_dwarf_get_AT_name(tag: c_uint) -> *const c_char {
    let mut tagname = ptr::null::<c_char>() as *const c_char;
    unsafe {
        let res = dwarf_get_AT_name(tag as u32, &mut tagname as *mut *const c_char);
        if (res != DW_DLV_OK) {
            panic!("Error in dwarf_get_AT_name\n");
        }
    }
    tagname
}
fn my_dwarf_get_TAG_name(tag: c_uint) -> *const c_char {
    let mut tagname = ptr::null::<c_char>() as *const c_char;
    unsafe {
        let res = dwarf_get_TAG_name(tag as u32, &mut tagname as *mut *const c_char);
        if (res != DW_DLV_OK) {
            panic!("Error in dwarf_get_TAG_name\n");
        }
    }
    tagname
}


fn my_dwarf_attrlist(die: Dwarf_Die) -> Vec<Dwarf_Attribute> {
    let mut vec: Vec<Dwarf_Attribute> = Vec::new();
    let mut attrlist = ptr::null::<Dwarf_Attribute>() as *mut Dwarf_Attribute;
    let mut length: Dwarf_Signed = 0;
    unsafe {
        let res = dwarf_attrlist(die, &mut attrlist as *mut *mut Dwarf_Attribute,
            &mut length as *mut Dwarf_Signed, dwarf_error());
        let slice = from_raw_parts(attrlist, length as usize);
        vec.extend_from_slice(slice);
        if (res != DW_DLV_OK) {
            panic!("Error in dwarf_attrlist");
        }
    }
    vec
}

fn my_dwarf_tag(die: Dwarf_Die) -> c_uint {
    let mut tag: Dwarf_Half = 0;
    unsafe {
        let res = dwarf_tag(die, &mut tag as *mut u16, dwarf_error());
        if (res != DW_DLV_OK) {
            panic!("Error in dwarf_tag");
        }
    }
    tag as c_uint
}

fn my_dwarf_diename(die: Dwarf_Die) -> Option<*mut c_char> {
    let mut name = ptr::null::<c_char>() as *mut c_char;
    unsafe {
        let res = dwarf_diename(die, &mut name as *mut *mut c_char, dwarf_error());
        if (res == DW_DLV_ERROR) {
            panic!("Error in dwarf_diename");
        }
        if (res == DW_DLV_NO_ENTRY) {
            return None;
        }
        return Some(name)
    }
    
}

fn my_dwarf_sibling_of(dbg: Dwarf_Debug, cur_die: Dwarf_Die) -> Option<Dwarf_Die> {
    let mut sib_die = ptr::null::<Struct_Dwarf_Die_s>() as Dwarf_Die;
    let error = dwarf_error();
    unsafe {
        let res = dwarf_siblingof(dbg,
            cur_die,
            &mut sib_die as *mut Dwarf_Die,
            error as *mut Dwarf_Error);
        if (res == DW_DLV_ERROR) {
            panic!("Error in dwarf_siblingof");
        }
        if (res == DW_DLV_NO_ENTRY) {
            return None;
        }
    }
    Some(sib_die)
}

fn my_dwarf_child(die: Dwarf_Die) -> Option<Dwarf_Die> {
    let mut child = ptr::null::<Struct_Dwarf_Die_s>() as Dwarf_Die;
    let error = dwarf_error();
    unsafe {
        let res = dwarf_child(die, &mut child as *mut Dwarf_Die, error);
        if (res == DW_DLV_ERROR) {
            panic!("Error in dwarf_child");
        }
        if (res == DW_DLV_NO_ENTRY) {
            return None;
        }
    }
    Some(child)
}

fn my_dwarf_bytesize(die: Dwarf_Die) -> Dwarf_Unsigned {
    let mut size: Dwarf_Unsigned = 0;
    unsafe {
        let res = dwarf_bytesize(die, &mut size as *mut Dwarf_Unsigned, dwarf_error());
        if (res == DW_DLV_ERROR) {
            panic!("Error in dwarf_bytesize");
        }
    }
    size
}

fn my_dwarf_isbitfield(die: Dwarf_Die) -> Dwarf_Bool {
    let mut size: Dwarf_Bool = 0;
    unsafe {
        let res = dwarf_isbitfield(die, &mut size as *mut Dwarf_Bool, dwarf_error());
        if (res == DW_DLV_ERROR) {
            panic!("Error in dwarf_isbitfield");
        }
    }
    size
}

fn my_dwarf_bitsize(die: Dwarf_Die) -> Dwarf_Unsigned {
    let mut size: Dwarf_Unsigned = 0;
    unsafe {
        let res = dwarf_bitsize(die, &mut size as *mut Dwarf_Unsigned, dwarf_error());
        if (res == DW_DLV_ERROR) {
            panic!("Error in my_dwarf_bitsize");
        }
    }
    size
}
fn my_dwarf_bitoffset(die: Dwarf_Die) -> Dwarf_Unsigned {
    let mut size: Dwarf_Unsigned = 0;
    unsafe {
        let res = dwarf_bitoffset(die, &mut size as *mut Dwarf_Unsigned, dwarf_error());
        if (res == DW_DLV_ERROR) {
            panic!("Error in dwarf_bitoffset");
        }
    }
    size
}
fn my_dwarf_srclang(die: Dwarf_Die) -> Dwarf_Unsigned {
    let mut size: Dwarf_Unsigned = 0;
    unsafe {
        let res = dwarf_bitsize(die, &mut size as *mut Dwarf_Unsigned, dwarf_error());
        if (res == DW_DLV_ERROR) {
            panic!("Error in dwarf_srclang");
        }
    }
    size
}

fn my_dwarf_formstring(attr: Dwarf_Attribute) -> Option<*mut c_char> {
    let mut name = ptr::null::<c_char>() as *mut c_char;
    unsafe {
        let res = dwarf_formstring(attr, &mut name as *mut *mut c_char, dwarf_error());
        if (res == DW_DLV_ERROR) {
            return None;
            panic!("Error in formstring: {}", res);
        }
        if res == DW_DLV_NO_ENTRY {
            return None;
        }
    }
    Some(name)
}


fn my_dwarf_whatform(arg: Dwarf_Attribute) -> Dwarf_Half {
    let mut ret : Dwarf_Half = 0;
    unsafe {
        let res = dwarf_whatform(arg, &mut ret as *mut Dwarf_Half, dwarf_error());
        if (res != DW_DLV_OK) {
            panic!("Error in dwarf_whatform");
        }
    }
    ret
}

fn my_dwarf_whatform_direct(arg: Dwarf_Attribute) -> Dwarf_Half {
    let mut ret : Dwarf_Half = 0;
    unsafe {
        let res = dwarf_whatform_direct(arg, &mut ret as *mut Dwarf_Half, dwarf_error());
        if (res != DW_DLV_OK) {
            panic!("Error in dwarf_whatform_direct");
        }
    }
    ret
}

fn my_dwarf_whatattr(arg: Dwarf_Attribute) -> Dwarf_Half {
    let mut ret : Dwarf_Half = 0;
    unsafe {
        let res = dwarf_whatattr(arg, &mut ret as *mut Dwarf_Half, dwarf_error());
        if (res != DW_DLV_OK) {
            panic!("Error in dwarf_whatattr");
        }
    }
    ret
}

fn my_dwarf_formref(arg: Dwarf_Attribute) -> Dwarf_Off {
    let mut ret : Dwarf_Off = 0;
    unsafe {
        let res = dwarf_formref(arg, &mut ret as *mut Dwarf_Off, dwarf_error());
        if (res != DW_DLV_OK) {
            panic!("Error in dwarf_formref");
        }
    }
    ret
}

fn my_dwarf_global_formref(arg: Dwarf_Attribute) -> Dwarf_Off {
    let mut ret : Dwarf_Off = 0;
    unsafe {
        let res = dwarf_global_formref(arg, &mut ret as *mut Dwarf_Off, dwarf_error());
        if (res != DW_DLV_OK) {
            panic!("Error in dwarf_global_formref");
        }
    }
    ret
}

fn my_dwarf_formaddr(arg: Dwarf_Attribute) -> Dwarf_Addr {
    let mut ret : Dwarf_Addr = 0;
    unsafe {
        let res = dwarf_formaddr(arg, &mut ret as *mut Dwarf_Addr, dwarf_error());
        if (res != DW_DLV_OK) {
            panic!("Error in dwarf_formaddr");
        }
    }
    ret
}

fn my_dwarf_formflag(arg: Dwarf_Attribute) -> Dwarf_Bool {
    let mut ret : Dwarf_Bool = 0;
    unsafe {
        let res = dwarf_formflag(arg, &mut ret as *mut Dwarf_Bool, dwarf_error());
        if (res != DW_DLV_OK) {
            panic!("Error in dwarf_formflag");
        }
    }
    ret
}

fn my_dwarf_formudata(arg: Dwarf_Attribute) -> Dwarf_Unsigned {
    let mut ret : Dwarf_Unsigned = 0;
    unsafe {
        let res = dwarf_formudata(arg, &mut ret as *mut Dwarf_Unsigned, dwarf_error());
        if (res != DW_DLV_OK) {
            panic!("Error in dwarf_formudata");
        }
    }
    ret
}

fn my_dwarf_formsdata(arg: Dwarf_Attribute) -> Dwarf_Signed {
    let mut ret : Dwarf_Signed = 0;
    unsafe {
        let res = dwarf_formsdata(arg, &mut ret as *mut Dwarf_Signed, dwarf_error());
        if (res != DW_DLV_OK) {
            panic!("Error in dwarf_formsdata");
        }
    }
    ret
}

fn print_die_data(dbg: Dwarf_Debug, print_me: Dwarf_Die, level: u32) {
    unsafe {
        let name = match my_dwarf_diename(print_me) {
            Some(name) => CStr::from_ptr(name),
            None => return,
        };

        let size = my_dwarf_bytesize(print_me);
        let tag = my_dwarf_tag(print_me);
        let tagname = CStr::from_ptr(my_dwarf_get_TAG_name(tag));
        indent(level);
        println!("{:?} | size: {} | tag: {:?} {:?}",
                 name,
                 size,
                 tag,
                 tagname);

        let attributes = my_dwarf_attrlist(print_me);
        for attr in attributes {
            //println!("what form: {:?}", CStr::from_ptr(my_dwarf_get_FORM_name(my_dwarf_whatform(attr) as c_uint)));
            let whatattr = my_dwarf_whatattr(attr) as c_uint;
            let at_name = CStr::from_ptr(my_dwarf_get_AT_name(whatattr));
            if at_name.to_str().unwrap() == "DW_AT_type" {
                // this is the identifier for the type of the thing!!!!!!
                println!("    ref: {}", my_dwarf_formref(attr));
            }
            /*
            println!("attribute: {:?} {:?}", whatattr, at_name);
            match my_dwarf_formstring(attr) {
                Some(s) => println!("attr: {:?}", CStr::from_ptr(s)),
                None => {},
            };
            */
        }
        dwarf_dealloc(dbg,name.as_ptr() as *mut c_void,DW_DLA_STRING);
    }
}

fn get_die_and_siblings(dbg: Dwarf_Debug, in_die: Dwarf_Die, in_level: u32) {
    let mut cur_die = in_die;
    print_die_data(dbg, in_die, in_level);

    while true {
        let mut sib_die = ptr::null::<Struct_Dwarf_Die_s>() as Dwarf_Die;
        unsafe {
            match my_dwarf_child(cur_die) {
                Some(child) => get_die_and_siblings(dbg, child, in_level + 1),
                None => {},
            }
            print_die_data(dbg, cur_die, in_level);
            match my_dwarf_sibling_of(dbg, cur_die) {
                Some(v) => { sib_die = v }
                None => break
            }
            if (cur_die != in_die) {
                dwarf_dealloc(dbg,cur_die as *mut c_void,DW_DLA_DIE);
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
            let cu_die = match my_dwarf_sibling_of(dbg, no_die) {
                Some(v) => v,
                None => panic!("no entry! in dwarf_siblingof on CU die \n"),
            };
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
