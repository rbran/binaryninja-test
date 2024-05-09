use binaryninja::binaryview::BinaryViewExt;

use tikv_jemallocator::Jemalloc;
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    binaryninja::headless::init();
    let file = format!("{}/../assets/parse_args", env!("CARGO_MANIFEST_DIR"));
    let bv = binaryninja::load(file).unwrap();

    // we need to find at lest the functions:
    let mut functions = [
        ("_init", 0x001000, false),
        ("main", 0x001050, false),
        ("_start", 0x001100, false),
        ("_fini", 0x0012fc, false),
    ];
    let funs = bv.functions();
    for fun in funs.iter() {
        // check if we are looking for this function
        for (name, addr, found) in &mut functions {
            let found_name = fun.symbol().full_name();
            let found_name = found_name.as_str();
            let found_addr = fun.start();
            let same_name = *name == found_name;
            let same_addr = *addr == found_addr;
            match (same_name, same_addr, *found) {
                // found the function, mark it as found
                (true, true, false) => *found = true,
                // found the same function multiple times
                (true, true, true) => panic!("Duplicated function {found_name} at {found_addr:#x}"),
                // same name but wrong address, or vice-versa
                (false, true, _) | (true, false, _) => {
                    panic!("Wrong function {found_name} at {found_addr:#x}")
                }
                // not the function that we are looking for
                (false, false, _) => {}
            }
        }
    }

    // check if we could not find any function
    let all_found = functions.iter().all(|(name, addr, found)| {
        if !*found {
            println!("Function {name} at {addr:#x} not found")
        }
        *found
    });
    if !all_found {
        panic!("Missing functions");
    }

    binaryninja::headless::shutdown();
}
