use super::rc::*;
#[test]
fn TestClean() {
    use super::rc::*;
    let mut _1 =     Vec::with_capacity(4).as_mut_ptr();
    _1 = unsafe{std::mem::transmute_copy::<u32,*mut u8>(&(1 as u32))};
    let mut _2 =     Vec::with_capacity(4).as_mut_ptr();
    _2 = unsafe{std::mem::transmute_copy::<u32,*mut u8>(&(2 as u32))};
    let mut _3 =     Vec::with_capacity(4).as_mut_ptr();
    _3 = unsafe{std::mem::transmute_copy::<u32,*mut u8>(&(3 as u32))};
    let mut _4 =     Vec::with_capacity(4).as_mut_ptr();
    _4 = unsafe{std::mem::transmute_copy::<u32,*mut u8>(&(4 as u32))};

    let mut heap = Heap::new();
    let mut g_1 = Block::from_ptr(4, _1);
    let mut g_2 = Block::from_ptr(4, _2);
    
    let mut g_3 = Block::from_ptr(4, _3);
    let mut g_4 = Block::from_ptr(4, _4);

    let i_4 = heap.push(g_4);
    g_3.add_ref(heap.get(i_4));
    let i_3 = heap.push(g_3);
    let i_2 = heap.push(g_2);
    g_1.add_ref(heap.get(i_2));
    g_1.add_ref(heap.get(i_3));
    let i_1 = heap.push(g_1);
    // will be removed

    let mut _5 =     Vec::with_capacity(4).as_mut_ptr();
    _5 = unsafe{std::mem::transmute_copy::<u32,*mut u8>(&(5 as u32))};
    let mut g_5 = Block::from_ptr(4, _5);
    let i_5 = heap.push(g_5);
    assert_eq!(heap.blocks.len(),5);
    heap.clean(vec!(i_1));
    assert_eq!(heap.blocks.len(),4);
}

#[test]
fn TestNoClean() {
    use super::rc::*;
    let mut _1 =     Vec::with_capacity(4).as_mut_ptr();
    _1 = unsafe{std::mem::transmute_copy::<u32,*mut u8>(&(1 as u32))};
    let mut _2 =     Vec::with_capacity(4).as_mut_ptr();
    _2 = unsafe{std::mem::transmute_copy::<u32,*mut u8>(&(2 as u32))};
    let mut _3 =     Vec::with_capacity(4).as_mut_ptr();
    _3 = unsafe{std::mem::transmute_copy::<u32,*mut u8>(&(3 as u32))};
    let mut _4 =     Vec::with_capacity(4).as_mut_ptr();
    _4 = unsafe{std::mem::transmute_copy::<u32,*mut u8>(&(4 as u32))};

    let mut heap = Heap::new();
    let mut g_1 = Block::from_ptr(4, _1);
    let mut g_2 = Block::from_ptr(4, _2);
    
    let mut g_3 = Block::from_ptr(4, _3);
    let mut g_4 = Block::from_ptr(4, _4);

    let i_4 = heap.push(g_4);
    g_3.add_ref(heap.get(i_4));
    let i_3 = heap.push(g_3);
    let i_2 = heap.push(g_2);
    g_1.add_ref(heap.get(i_2));
    g_1.add_ref(heap.get(i_3));
    let i_1 = heap.push(g_1);
    // will be removed

    let mut _5 =     Vec::with_capacity(4).as_mut_ptr();
    _5 = unsafe{std::mem::transmute_copy::<u32,*mut u8>(&(5 as u32))};
    let mut g_5 = Block::from_ptr(4, _5);
    let i_5 = heap.push(g_5);
    assert_eq!(heap.blocks.len(),5);
    heap.clean(vec!(i_1,i_5));
    assert_eq!(heap.blocks.len(),5);
}
