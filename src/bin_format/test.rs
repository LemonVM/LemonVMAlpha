use super::*;
#[test]
fn LocVarFromBytes() {
    let bytes = [
        0x02, 0x00, 0x00, 0x00, 0x4c, 0x00, 0x65, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00,
        0x00,
    ];
    let mut reader = reader::Reader::new(bytes.as_ptr());
    let locvar = reader.read_loc_var();
    assert_eq!(locvar.name[0], 'L' as u16);
    assert_eq!(locvar.name[1], 'e' as u16);
    assert_eq!(locvar.start_pc, 1);
    assert_eq!(locvar.end_pc, 2);
}

#[test]
fn ValidateHeader() {
    let mut reader =
        reader::Reader::new([0x4c, 0x65, 0x4d, 0x30, 0x26, 0x01, 0x04, 0x02, 0x04, 0x08].as_ptr());
    let header = reader.read_header();
    assert_eq!(header.validate(), true);
}
#[test]
fn closure_capFromBytes() {
    let bytes = [0x01, 0x02];
    let mut reader = reader::Reader::new(bytes.as_ptr());
    let closure_cap = reader.read_closure_cap();
    assert_eq!(closure_cap.instack, 0x01);
    assert_eq!(closure_cap.idx, 0x02);
}

#[test]
fn ConstantFromBytes() {
    let bytes = [0x00];
    let mut reader = reader::Reader::new(bytes.as_ptr());
    let constant = reader.read_constant();
    assert_eq!(constant, Constant::Null);
    let bytes = [0x01, 0x01];
    let mut reader = reader::Reader::new(bytes.as_ptr());
    let constant = reader.read_constant();
    assert_eq!(constant, Constant::Bool(true));
    let bytes = [0x02, 0x4c, 0x00];
    let mut reader = reader::Reader::new(bytes.as_ptr());
    let constant = reader.read_constant();
    assert_eq!(constant, Constant::Char('L' as VMChar));
    let bytes = [0x03, 0x03, 0x00, 0x00, 0x00];
    let mut reader = reader::Reader::new(bytes.as_ptr());
    let constant = reader.read_constant();
    assert_eq!(constant, Constant::Int(3));
    let bytes = [0x04, 0x4a, 0xd8, 0x12, 0x4d, 0xfb, 0x21, 0x09, 0x40];
    let mut reader = reader::Reader::new(bytes.as_ptr());
    let constant = reader.read_constant();
    assert_eq!(constant, Constant::Num(3.1415926));
    let bytes = [0x05, 0x02, 0x00, 0x00, 0x00, 0x4c, 0x00, 0x65, 0x00];
    let mut reader = reader::Reader::new(bytes.as_ptr());
    let constant = reader.read_constant();
    assert_eq!(constant, Constant::Sym(vec!['L' as u16, 'e' as u16]));
}

#[test]
fn ProtoFromByteCode() {
    let constant_pool = [
        0x02, 0x00, 0x00, 0x00, 0x4c, 0x00, 0x65, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x01, 0x01
    ];
    let bytes = [
        0x02, 0x00, 0x00, 0x00, 0x4c, 0x00, 0x65, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x03, 0x00, 0x00, 0x00,
        0x00,
        0x00,
        0x00,
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
    ];
    reader::Reader::read_constant_pool(constant_pool.as_ptr(), constant_pool.len());
    let mut reader = reader::Reader::new(bytes.as_ptr());
    let proto = reader.read_proto();
    assert_eq!(proto.line_start, 0);
    assert_eq!(proto.line_end, 3);
    assert_eq!(proto.get_constants()[0], Constant::Bool(true));
    assert_eq!(proto.instruction_table.len(), 0);
    assert_eq!(proto.protos.len(), 0);
}

#[test]
fn BinaryChunkFromByteCode() {
    let bytes = [
        0x4c, 0x65, 0x4d, 0x30, 0x26, 0x01, 0x04, 0x02, 0x04, 0x08, 0x01, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xFF, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x05, 0x02, 0x00, 0x00, 0x00, 0x68, 0x00, 0x69, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00,
    ];
    let mut reader = reader::Reader::new(bytes.as_ptr());
    let bin = reader.read_binary_chunk();
    assert_eq!(bin.header.validate(), true);
    assert_eq!(bin.up_value_size, 1);
}
