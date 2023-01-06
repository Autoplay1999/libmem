/* Disable warnings for libmem compatibility */
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

/* Note: the types and structures must be
 * the same size and aligned with their C variations */
const LM_FALSE : i32 = 0;
const LM_TRUE : i32 = 1;
const LM_PATH_MAX : usize = 512;

#[repr(C)]
#[derive(Clone)]
#[derive(Copy)]
pub struct lm_process_t {
    pid : u32,
    ppid : u32,
    bits : usize,
    // OBS: if lm_char_t is a wchar_t, these variables won't work. Use Multibyte
    path : [u8; LM_PATH_MAX],
    name : [u8; LM_PATH_MAX]
}

fn string_from_cstring(cstring : &[u8]) -> String {
    // This function finds the null terminator from
    // a vector and deletes everything after that
    let mut cstring = cstring.to_vec();
    let mut null_index = 0;

    for i in 0..cstring.len() {
        if cstring[i] == 0 {
            null_index = i;
            break;
        }
    }

    if null_index == 0 {
        cstring.clear();
    } else {
        cstring = cstring[0..null_index].to_vec();
    }

    String::from_utf8_lossy(&cstring).to_string()
}

impl lm_process_t {
    pub fn get_pid(&self) -> u32 {
        self.pid
    }

    pub fn get_ppid(&self) -> u32 {
        self.ppid
    }

    pub fn get_bits(&self) -> usize {
        self.bits
    }

    pub fn get_path(&self) -> String {
        string_from_cstring(&self.path)
    }

    pub fn get_name(&self) -> String {
        string_from_cstring(&self.name)
    }
}

// Raw libmem calls
mod libmem_c {
    use crate::*;

    // link against 'mem' (the lib prefix is appended automatically)
    #[link(name = "mem")]
    extern "C" {
        pub(super) fn LM_EnumProcesses(callback : extern "C" fn(*const lm_process_t, *mut ()) -> i32, arg : *mut ()) -> i32;
        pub(super) fn LM_GetProcess(pproc : *mut lm_process_t) -> i32;
    }
}

// Rustified libmem calls
extern "C" fn _LM_EnumProcessesCallback(pproc : *const lm_process_t, arg : *mut ()) -> i32 {
    let proc_list_ptr = arg as *mut Vec<lm_process_t>;
    unsafe {
        (*proc_list_ptr).push(*pproc);
    }
    LM_TRUE
}

pub fn LM_EnumProcesses() -> Vec<lm_process_t> {
    let mut proc_list : Vec<lm_process_t> = Vec::new();
    unsafe {
        let arg = &mut proc_list as *mut Vec<lm_process_t>;
        let arg = arg as *mut ();
        if libmem_c::LM_EnumProcesses(_LM_EnumProcessesCallback, arg) == LM_FALSE {
            proc_list.clear();
        }
    }

    proc_list
}

pub fn LM_GetProcess() -> Option<lm_process_t> {
    let mut proc = lm_process_t {
        pid: 0, ppid: 0, bits: 0,
        path: [0;LM_PATH_MAX], name: [0;LM_PATH_MAX]
    };

    unsafe {
        if libmem_c::LM_GetProcess(&mut proc as *mut lm_process_t) != LM_FALSE {
            Some(proc)
        } else {
            None
        }
    }
}
