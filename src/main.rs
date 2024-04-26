use binaryninja::architecture::Architecture;
use binaryninja::binaryview::{BinaryViewBase, BinaryViewExt};

use tikv_jemallocator::Jemalloc;
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

//use mimalloc::MiMalloc;
//#[global_allocator]
//static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    println!("main");
    binaryninja::headless::init();
    let bv = binaryninja::load("/bin/true").unwrap();

    let funs = bv.functions();
    assert!(!funs.is_empty());
    for fun in funs.iter() {
        println!("function {}", fun.symbol().short_name());
        let arch = fun.arch();
        let start: usize = fun.start().try_into().unwrap();
        let end: usize = (fun.highest_address() + 1).try_into().unwrap();
        let len = end - start;
        let mut buf = vec![0u8; len];
        assert_eq!(bv.read(&mut buf, fun.start()), buf.len());

        let mut counter = 0;
        while counter < len {
            let (consumed, instr) = arch
                .instruction_text(&buf[counter..], (start + counter).try_into().unwrap())
                .unwrap();
            println!(
                "\tconsumed {consumed} {:#08x} instr: {}",
                start + counter,
                instr.iter().map(|i| i.text()).collect::<String>()
            );
            counter += consumed;
        }
        println!();
    }

    binaryninja::headless::shutdown();
    println!("shutdown complete");
}
