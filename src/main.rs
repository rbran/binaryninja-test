use tikv_jemallocator::Jemalloc;
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

//use mimalloc::MiMalloc;
//#[global_allocator]
//static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    println!("main");
    binaryninja::headless::init();
    let _bv = binaryninja::load("/bin/true").unwrap();

    binaryninja::headless::shutdown();
    println!("shutdown complete");
}
