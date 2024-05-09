use binaryninja::{
    binaryview::BinaryViewExt, mlil::MediumLevelILLiftedInstructionKind, types::PossibleValueSet,
};

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    binaryninja::headless::init();
    let file = format!("{}/../assets/parse_args", env!("CARGO_MANIFEST_DIR"));
    let bv = binaryninja::load(file).unwrap();
    let functions = bv.functions_at(0x001050);
    assert_eq!(functions.len(), 1);
    let main = functions.get(0);
    let name = "MyReg".into();

    // add/remove a fake type_ref
    main.add_user_type_ref(0x0010ba, &name, None);
    main.remove_user_type_ref(0x0010ba, &name, None);

    // add/remove a fake type_field_ref
    main.add_user_type_field_ref(0x0010ba, &name, 0, None, None);
    main.remove_user_type_field_ref(0x0010ba, &name, 0, None, None);

    let mlil = main.medium_level_il().unwrap();
    const INST_ADDR: u64 = 0x10C8;
    // get the variable being set at address 0x10C8
    let var = mlil
        .basic_blocks()
        .iter()
        .find_map(|block| {
            block
                .iter()
                .find(|inst| inst.address == INST_ADDR)
                .map(|inst| {
                    let MediumLevelILLiftedInstructionKind::SetVar(set_var) = inst.lift().kind
                    else {
                        panic!()
                    };
                    set_var.dest
                })
        })
        .unwrap();

    // set it's value to 1
    mlil.set_user_var_value(
        &var,
        INST_ADDR,
        PossibleValueSet::ConstantValue { value: 1 },
    )
    .unwrap();

    // ensure the value was set correctly
    for (_variable, addr_and_arch, value) in mlil.user_var_values().all() {
        if addr_and_arch.address != INST_ADDR {
            continue;
        }
        assert!(matches!(value, PossibleValueSet::ConstantValue { value: 1 }));
    }

    binaryninja::headless::shutdown();
}
