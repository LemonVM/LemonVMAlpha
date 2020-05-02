// TODO: JIT EXECUTE
fn template_inter_jit(ins:u8,state:usize){
    // vm
    if ins == 0{}
    // load
    else if ins < 0 && ins > 0{}
    // cf
    else if ins < 0 && ins > 0{}
    // comp
    else if ins < 0 && ins > 0{}
    // num
    else if ins < 0 && ins > 0{}
    // stack
    else if ins < 0 && ins > 0{}
    // user def
    else if ins < 0 && ins > 0{}
    // debug
    else if ins < 0 && ins > 0{}
    // failed fall back to execute

}
fn jit_some_lines(func:&mut usize,ins_line_start:u32,ins_line_end:u32){
    // let bytecodes =  func[ins_line_start..inst_line_end];
    // let gen_native_code_buffer = gen(bytecodes)?;
    // Ok
    // let vm_call_for_jit = native_code_to_vm_call(gen...);
    // let replaced = bytecodes.splice(ins_line_start..inst_line_end,vm_call_for_jit and nops);
    // if debug then print replaced or something...
    // Err
    // donothing
}
fn jit_func(){
    // let sub_funcs = func.funcs;
    // while one fails with calling jit func -> fallback to jit_some_line with line(FOR EXAMPLE FORLOOPS,TAILCALLS) of that function call
    // let gen_native_code_buffer = gen(bytecodes)?;
    // Ok
    // let vm_call_for_jit = native_code_to_vm_call(gen...);
    // func.instructions replace to call native;
        // if debug then print replaced or something...
    // Err
        // donothing
}