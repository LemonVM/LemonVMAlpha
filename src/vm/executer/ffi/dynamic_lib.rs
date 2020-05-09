extern crate libloading as lib;
use super::super::super::super::bin_format::*;

pub fn pass_args_to_NFunc_and_call(
    path: VMSym,
    symbol: VMSym,
    ret_type: Type,
    current_stack: &mut super::super::stack::Stack,
    args: u8,
) -> super::super::Value {
    let path = String::from_utf16(path.0.as_ref());
    let symbol = String::from_utf16(symbol.0.as_ref());
    let lib = lib::Library::new(path.unwrap()).unwrap();
    let func = unsafe {
        let func: lib::Symbol<*const u8> = lib.get(symbol.unwrap().as_ref()).unwrap();
        *func.into_raw()
    };
    use libffi::low::*;

    let cptr = CodePtr::from_ptr(func as *const libc::c_void);
    let mut fargs = vec![];
    for i in 0..args as usize {
        fargs.push(current_stack.stack.pop().unwrap());
    }
    let mut cif: ffi_cif = Default::default();
    unsafe {
        prep_cif(
            &mut cif,
            ffi_abi_FFI_DEFAULT_ABI,
            args as usize,
            &mut types::pointer,
            vec![].as_mut_ptr(),
        )
        .unwrap()
    };

    use super::super::PrimeValue::*;
    let mut fargs: Vec<*mut libc::c_void> = fargs
        .iter_mut()
        .map(|a| {
            let super::super::Value(v, t) = a;
            match v {
                Bool(a) => a as *mut bool as *mut libc::c_void,
                Byte(a) => a as *mut u8 as *mut libc::c_void,
                Char(a) => a as *mut u16 as *mut libc::c_void,
                Int(a) => a as *mut u32 as *mut libc::c_void,
                Num(a) => a as *mut f64 as *mut libc::c_void,
                UserData(a) => unsafe{*a.0 as *mut libc::c_void},
                _ => unimplemented!(),
            }
        })
        .collect();
    unsafe {
        match ret_type.clone() {
            Type::Mono(TAG_BOOL) => {
                super::super::Value::from(Bool(call(&mut cif, cptr, fargs.as_mut_ptr())))
            }
            Type::Mono(TAG_BYTE) => {
                super::super::Value::from(Byte(call(&mut cif, cptr, fargs.as_mut_ptr())))
            }
            Type::Mono(TAG_CHAR) => {
                super::super::Value::from(Char(call(&mut cif, cptr, fargs.as_mut_ptr())))
            }
            Type::Mono(TAG_INT) => {
                super::super::Value::from(Int(call(&mut cif, cptr, fargs.as_mut_ptr())))
            }
            Type::Mono(TAG_NUM) => {
                super::super::Value::from(Num(call(&mut cif, cptr, fargs.as_mut_ptr())))
            }
            Type::Mono(TAG_USERDATA) => {
                super::super::Value::from(UserData(call(&mut cif, cptr, fargs.as_mut_ptr())))
            }
            super::super::super::super::bin_format::Type::Kind => super::super::Value::from({
                let _: *mut libc::c_void = call(&mut cif, cptr, fargs.as_mut_ptr());
                Null
            }),
            _ => unimplemented!(),
        }
    }
}
