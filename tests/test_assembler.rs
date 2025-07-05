use corewar::assembler::Assembler;

const COR_MAGIC: u32 = 0xea83f3;

#[test]
fn test_assemble_simple_champion() {
    let source = r#"
        .name "simple"
        .comment "A simple champion"

    start:
        live %1
        zjmp %:start
    "#;

    let assembler = Assembler::new(false);
    let bytecode = assembler.assemble_source(source).unwrap();

    // Header size is 4 (magic) + 128 (name) + 4 (padding) + 4 (size) + 128 (comment) + 4 (padding) = 272
    let header_size = 272;
    let instruction_size = 1 + 1 + 2 + 1 + 1 + 2; // live %1 (opcode + param_types + 2 bytes) + zjmp %:start (opcode + param_types + 2 bytes)
    assert_eq!(bytecode.len(), header_size + instruction_size);

    // Check magic number (stored in little-endian)
    let magic = u32::from_le_bytes([bytecode[0], bytecode[1], bytecode[2], bytecode[3]]);
    assert_eq!(magic, COR_MAGIC);

    // Check champion name
    let name = &bytecode[4..4 + 128];
    let name_str = std::str::from_utf8(&name[..6]).unwrap();
    assert_eq!(name_str, "simple");

    // Check bytecode for live %1
    assert_eq!(bytecode[header_size], 0x01); // live opcode
    assert_eq!(bytecode[header_size + 1], 0b10000000); // Parameter type (direct)
    let live_param = u16::from_le_bytes([bytecode[header_size + 2], bytecode[header_size + 3]]);
    assert_eq!(live_param, 1);

    // Check bytecode for zjmp %:start
    assert_eq!(bytecode[header_size + 4], 0x09); // zjmp opcode
    assert_eq!(bytecode[header_size + 5], 0b10000000); // Parameter type (direct)
    let zjmp_param = u16::from_le_bytes([bytecode[header_size + 6], bytecode[header_size + 7]]);
    assert_eq!(zjmp_param, 0); // Jumps to the start
}
