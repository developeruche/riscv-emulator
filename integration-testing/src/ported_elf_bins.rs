use emulator_sdk::vm::Vm;

#[test]
fn test_load_elf_program() {
    for entry in std::fs::read_dir("ported-bins").unwrap() {
        let mut vm =
            Vm::from_bin_elf(String::from(entry.unwrap().path().to_str().unwrap())).unwrap();
        vm.run();
        assert!(!vm.running);
        assert_eq!(vm.exit_code, 0);
    }
}
