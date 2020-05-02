pub fn write_constant_pool(path:String,filename:String,constant_pool:&[u8]){
    use std::fs::File;
    use std::io::prelude::*;
    let mut cp = std::fs::File::create(format!("{}{}.lmvmcp",path,filename)).unwrap();
    cp.write_all(&constant_pool).unwrap();
}
pub fn write_bin(path:String,filename:String,bin:&[u8]){
    use std::fs::File;
    use std::io::prelude::*;
    let mut cp = std::fs::File::create(format!("{}{}.lmvmb",path,filename)).unwrap();
    cp.write_all(&bin).unwrap();
}