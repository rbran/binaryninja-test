use binaryninja::binaryview::BinaryViewExt;

fn main() {
    binaryninja::headless::init();
    let file = format!("{}/../assets/parse_args", env!("CARGO_MANIFEST_DIR"));
    let bv = binaryninja::load(file).unwrap();
    let functions = bv.functions_at(0x001050);
    assert_eq!(functions.len(), 1);
    let main = functions.get(0);
    let important = bv.create_tag_type("Important", "⚠️");
    main.add_tag(&important, "I think this is the main function", None, false, None);
    let crash = bv.create_tag_type("Results", "🎯");
    main.add_tag(&crash, "Result get Printed", Some(0x1082), false, None);

    binaryninja::headless::shutdown();
}
