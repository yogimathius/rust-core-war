/// Core War - A Rust implementation of the Core War programming game
///
/// This is the main CLI interface for running Core War battles between
/// champion programs written in Redcode assembly language.
use clap::{Arg, ArgAction, Command};
use corewar::{Assembler, GameConfig, GameEngine};
use log::{error, info};
use std::path::PathBuf;
use std::process;

fn main() {
    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    // Parse command line arguments
    let matches = Command::new("corewar")
        .version("0.1.0")
        .author("Mathius Johnson")
        .about("Core War - Programming game virtual machine")
        .long_about("Core War is a programming game where players write programs in Redcode assembly \
                    language. These programs compete in a virtual machine's memory space, with the \
                    objective of stopping opposing programs while keeping their own programs alive.")
        .subcommand(
            Command::new("run")
                .about("Run a Core War battle")
                .arg(
                    Arg::new("champions")
                        .help("Champion .cor files to load")
                        .value_name("FILE")
                        .num_args(1..=4)
                        .required(true)
                )
                .arg(
                    Arg::new("visual")
                        .short('v')
                        .long("visual")
                        .help("Enable terminal visualization")
                        .action(ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("dump")
                        .short('d')
                        .long("dump")
                        .help("Dump memory after specified cycles")
                        .value_name("CYCLES")
                        .value_parser(clap::value_parser!(u32))
                )
                .arg(
                    Arg::new("speed")
                        .short('s')
                        .long("speed")
                        .help("Set execution speed (1-1000)")
                        .value_name("RATE")
                        .value_parser(clap::value_parser!(u32))
                        .default_value("1")
                )
                .arg(
                    Arg::new("pause")
                        .short('p')
                        .long("pause")
                        .help("Start in paused mode")
                        .action(ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("cycles")
                        .short('c')
                        .long("cycles")
                        .help("Set maximum cycles")
                        .value_name("MAX")
                        .value_parser(clap::value_parser!(u32))
                        .default_value("0")
                )
                .arg(
                    Arg::new("verbose")
                        .long("verbose")
                        .help("Enable verbose logging")
                        .action(ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("asm")
                .about("Assemble a Redcode source file")
                .arg(
                    Arg::new("input")
                        .help("Input .s file")
                        .value_name("INPUT")
                        .required(true)
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .help("Output .cor file")
                        .value_name("OUTPUT")
                )
                .arg(
                    Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .help("Verbose compilation output")
                        .action(ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("info")
                .about("Display information about a champion file")
                .arg(
                    Arg::new("file")
                        .help("Champion .cor file")
                        .value_name("FILE")
                        .required(true)
                )
        )
        .get_matches();

    // Handle subcommands
    match matches.subcommand() {
        Some(("run", sub_matches)) => {
            if let Err(e) = run_battle(sub_matches) {
                error!("Failed to run battle: {}", e);
                process::exit(1);
            }
        }
        Some(("asm", sub_matches)) => {
            if let Err(e) = assemble_file(sub_matches) {
                error!("Failed to assemble file: {}", e);
                process::exit(1);
            }
        }
        Some(("info", sub_matches)) => {
            if let Err(e) = show_champion_info(sub_matches) {
                error!("Failed to show champion info: {}", e);
                process::exit(1);
            }
        }
        _ => {
            // No subcommand provided, show help
            let mut cmd = Command::new("corewar");
            cmd.print_help().unwrap();
            println!();
        }
    }
}

/// Run a Core War battle
fn run_battle(matches: &clap::ArgMatches) -> anyhow::Result<()> {
    let champion_files: Vec<PathBuf> = matches
        .get_many::<String>("champions")
        .unwrap()
        .map(PathBuf::from)
        .collect();

    let visual = matches.get_flag("visual");
    let dump_cycles = matches.get_one::<u32>("dump").copied().unwrap_or(0);
    let speed = matches.get_one::<u32>("speed").copied().unwrap_or(1);
    let start_paused = matches.get_flag("pause");
    let max_cycles = matches.get_one::<u32>("cycles").copied().unwrap_or(0);
    let verbose = matches.get_flag("verbose");

    // Validate speed
    if speed == 0 || speed > 1000 {
        return Err(anyhow::anyhow!("Speed must be between 1 and 1000"));
    }

    // Create game configuration
    let config = GameConfig {
        max_cycles,
        dump_cycles,
        speed,
        verbose,
        start_paused,
    };

    // Create and configure game engine
    let mut engine = GameEngine::new(config);

    // Load champions
    info!("Loading {} champions...", champion_files.len());
    engine.load_champions(&champion_files, None)?;

    // Run the battle
    if visual {
        // TODO: Implement visual mode (Phase 4)
        info!("Visual mode requested but not yet implemented");
        info!("Running in text mode instead");
        run_text_mode(&mut engine)?;
    } else {
        run_text_mode(&mut engine)?;
    }

    Ok(())
}

/// Run battle in text mode
fn run_text_mode(engine: &mut GameEngine) -> anyhow::Result<()> {
    info!("Starting Core War battle...");

    // Show initial state
    engine.dump_memory()?;

    // Run to completion
    let winner = engine.run_to_completion()?;

    // Show final results
    let stats = engine.get_stats();
    println!("\n=== Battle Results ===");
    println!("Total cycles: {}", stats.cycle);
    println!("Elapsed time: {:.2}s", stats.elapsed_time.as_secs_f64());
    println!("Cycles per second: {:.1}", stats.cycles_per_second);

    match winner {
        Some(winner_id) => {
            let winner_name = engine
                .champions()
                .iter()
                .find(|c| c.id == winner_id)
                .map(|c| c.name.as_str())
                .unwrap_or("Unknown");
            println!("Winner: Champion {} ({})", winner_id, winner_name);
        }
        None => {
            println!("Result: Draw (no winner)");
        }
    }

    // Final memory dump
    engine.dump_memory()?;

    Ok(())
}

/// Assemble a Redcode source file
fn assemble_file(matches: &clap::ArgMatches) -> anyhow::Result<()> {
    let input_file = matches.get_one::<String>("input").unwrap();
    let output_file = matches.get_one::<String>("output");
    let verbose = matches.get_flag("verbose");

    let assembler = Assembler::new(verbose);

    info!("Assembling {}...", input_file);

    let bytecode = assembler.assemble_file(input_file, output_file)?;

    let output_name = match output_file {
        Some(output) => output.to_string(),
        None => {
            let mut path = PathBuf::from(input_file);
            path.set_extension("cor");
            path.to_string_lossy().to_string()
        }
    };

    info!("Generated {} ({} bytes)", output_name, bytecode.len());

    Ok(())
}

/// Show information about a champion file
fn show_champion_info(matches: &clap::ArgMatches) -> anyhow::Result<()> {
    let champion_file = matches.get_one::<String>("file").unwrap();

    let loader = corewar::ChampionLoader::new(true);
    let info = loader.get_champion_info(champion_file)?;

    println!("Champion Information");
    println!("===================");
    println!("File: {}", champion_file);
    println!("Name: {}", info.name);
    println!("Comment: {}", info.comment);
    println!("Code size: {} bytes", info.code_size);
    println!("Magic: 0x{:08x}", info.magic);

    Ok(())
}
