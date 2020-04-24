#[test]
fn TestTriColorGC() {
    use std::alloc::*;
    use super::tri_color::*;
    let mut _1 =     Vec::with_capacity(4).as_mut_ptr();
    _1 = unsafe{std::mem::transmute_copy::<u32,*mut u8>(&(1 as u32))};
    let mut _2 =     Vec::with_capacity(4).as_mut_ptr();
    _2 = unsafe{std::mem::transmute_copy::<u32,*mut u8>(&(2 as u32))};
    let mut _3 =     Vec::with_capacity(4).as_mut_ptr();
    _3 = unsafe{std::mem::transmute_copy::<u32,*mut u8>(&(3 as u32))};
    let mut _4 =     Vec::with_capacity(4).as_mut_ptr();
    _4 = unsafe{std::mem::transmute_copy::<u32,*mut u8>(&(4 as u32))};

    let mut heap = GCHeap::new();
    
    let mut g_1 = GCBlock::from_ptr(4, _1);
    let mut g_2 = GCBlock::from_ptr(4, _2);
    
    let mut g_3 = GCBlock::from_ptr(4, _3);
    let mut g_4 = GCBlock::from_ptr(4, _4);
    let mut root = vec!();
    let i_2 = heap.test_push_white(g_2);
    g_1.add_ref(i_2);
    let i_4 = heap.test_push_white(g_4);
    g_3.add_ref(i_4);
    let i_3 = heap.test_push_white(g_3);
    g_1.add_ref(i_3);

    // this will be a none after gc
    // and when is ordered will remove completly
    let mut _5 =     Vec::with_capacity(4).as_mut_ptr();
    _5 = unsafe{std::mem::transmute_copy::<u32,*mut u8>(&(5 as u32))};
    let g_5 = GCBlock::from_ptr(4, _5);
    let i_5 = heap.push(g_5,&mut root);

    let i_1 = heap.test_push_white(g_1);
    root.push(i_1.clone());

    let mut _6 =     Vec::with_capacity(4).as_mut_ptr();
    _6 = unsafe{std::mem::transmute_copy::<u32,*mut u8>(&(6 as u32))};
    let g_6 = GCBlock::from_ptr(4, _6);
    let i_6 = heap.push(g_6,&mut root);
    
    if let Some(h1) = &mut heap.blocks[i_1]{
            if let Some(vec) = &mut h1.ref_to{
                    vec.push(i_6);
            }
    }

    heap.gc(&mut root,true);
    assert_eq!(heap.total_white_size,20);
}