use binaryninja::binaryview::BinaryViewExt;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    binaryninja::headless::init();
    let file = format!("{}/../assets/parse_args", env!("CARGO_MANIFEST_DIR"));
    let bv = binaryninja::load(&file).unwrap();
    let functions = bv.functions_at(0x001050);
    assert_eq!(functions.len(), 1);
    let main = functions.get(0);
    let mlil = main.medium_level_il().unwrap();
    for (variable, addr_and_arch, value) in mlil.user_var_values().all() {
        println!("{variable:?}:{} = {value:?}", addr_and_arch.address);
    }

    binaryninja::headless::shutdown();
}
