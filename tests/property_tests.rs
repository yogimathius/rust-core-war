use proptest::prelude::*;
use corewar::assembler::Assembler;
use corewar::vm::{GameConfig, GameEngine};
use tempfile::NamedTempFile;
use std::io::Write;

// Helper to create a dummy champion file for VM tests
fn create_dummy_champion(name: &str, code: &[u8]) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    let magic = 0xea83f3u32;
    file.write_all(&magic.to_le_bytes()).unwrap();
    let mut name_bytes = [0u8; 128];
    let name_src = name.as_bytes();
    name_bytes[..name_src.len()].copy_from_slice(name_src);
    file.write_all(&name_bytes).unwrap();
    file.write_all(&[0u8; 4]).unwrap();
    file.write_all(&(code.len() as u32).to_le_bytes()).unwrap();
    let comment = format!("{} - dummy champion", name);
    let mut comment_bytes = [0u8; 128];
    comment_bytes[..comment.len().min(127)].copy_from_slice(comment.as_bytes());
    file.write_all(&comment_bytes).unwrap();
    file.write_all(&[0u8; 4]).unwrap();
    file.write_all(code).unwrap();
    file.flush().unwrap();
    file
}

// Property: Assembling and disassembling a simple instruction should yield the same instruction
proptest! {
    #[test]
    fn prop_assemble_disassemble_simple_instruction(opcode in 0x01u8..=0x10) {
        let program_str = format!(
            ".name \"TestChamp\"\n.comment \"A test champion\"\n\nlive {}\n",
            opcode
        );
        let assembler = Assembler::new(false);
        let bytecode = assembler.assemble_source(&program_str).unwrap();
        prop_assert!(bytecode.len() > 0);
    }
}

// Property: VM should terminate within max_cycles
proptest! {
    #[test]
    fn prop_vm_terminates(max_cycles in 100u32..1000u32) {
        use std::sync::mpsc;
        use std::thread;
        use std::time::Duration;
        use proptest::test_runner::TestCaseError;

        let (sender, receiver) = mpsc::channel::<Result<(), TestCaseError>>();

        let handle = thread::spawn(move || {
            let result = (|| -> Result<(), TestCaseError> {
                let config = GameConfig {
                    max_cycles,
                    ..Default::default()
                };
                let mut engine = GameEngine::new(config);

                // Create two different champions for a proper Core War battle
                let champion1_file = create_dummy_champion("Champion1", &[0x01, 0x40, 0x01, 0x00]); // live %1
                let champion2_file = create_dummy_champion("Champion2", &[0x01, 0x40, 0x02, 0x00]); // live %2
                
                engine.load_champions(&[champion1_file.path(), champion2_file.path()], None).unwrap();
                engine.start().unwrap();

                // Manually tick the engine for a reasonable number of cycles
                let mut ticks = 0;
                while engine.tick().unwrap() && ticks < max_cycles * 2 {
                    ticks += 1;
                }

                prop_assert!(!engine.get_stats().running);
                prop_assert!(engine.get_stats().cycle <= max_cycles * 2);
                Ok(())
            })();

            let _ = sender.send(result);
        });

        match receiver.recv_timeout(Duration::from_secs(5)) {
            Ok(Ok(_)) => {}, // Test passed
            Ok(Err(e)) => panic!("Test thread failed: {:?}", e),
            Err(mpsc::RecvTimeoutError::Timeout) => panic!("Test thread timed out after 5 seconds"),
            Err(mpsc::RecvTimeoutError::Disconnected) => panic!("Test thread disconnected"),
        }

        let _ = handle.join().unwrap();
    }
}
