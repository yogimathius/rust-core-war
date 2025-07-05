# Core War Rust Rewrite Specification

## Overview

This document outlines the complete specification for rewriting the existing C-based Core War implementation in Rust, including enhanced terminal visualization tools to provide a full immersive Core War experience.

## What is Core War?

Core War is a programming game where players write programs in the Redcode assembly language. These programs compete in a virtual machine's memory space, with the objective of stopping opposing programs from executing while keeping their own programs alive.

## Current Implementation Analysis

The existing C implementation provides:

- Virtual Machine with 6KB memory space
- Assembler for Redcode compilation
- 16 distinct instruction types
- Process scheduling and management
- Champion loading and execution
- Basic memory dumping capabilities

## Core Features Required for Rust Rewrite

### 1. Virtual Machine Core

#### Memory Management

- **Circular Memory**: 6KB (6144 bytes) circular memory space
- **Memory Protection**: Read/write operations with bounds checking
- **Address Modulo**: All memory operations use modulo addressing (IDX_MOD = 512)
- **Memory Initialization**: Zero-initialized memory at startup

#### Process Management

- **Process Structure**: Each process maintains:
  - Program counter (PC)
  - 16 registers (r1-r16)
  - Carry flag
  - Live counter
  - Death status
  - Player ID and color
  - Cycle countdown
- **Process Scheduling**: Round-robin execution with cycle-based timing
- **Process Lifecycle**: Creation, execution, forking, and termination

#### Champion Management

- **Champion Loading**: Load compiled `.cor` files with proper header parsing
- **Champion Placement**: Automatic optimal placement in memory
- **Champion Identification**: Unique ID assignment and tracking
- **Champion State**: Track lives, execution status, and program counter

### 2. Instruction Set Architecture

#### Core Instructions (16 total)

1. **live** (0x01) - Declare process alive
2. **ld** (0x02) - Load value into register
3. **st** (0x03) - Store register value to memory
4. **add** (0x04) - Add two registers
5. **sub** (0x05) - Subtract registers
6. **and** (0x06) - Bitwise AND operation
7. **or** (0x07) - Bitwise OR operation
8. **xor** (0x08) - Bitwise XOR operation
9. **zjmp** (0x09) - Jump if carry flag is set
10. **ldi** (0x0A) - Load indirect with index
11. **sti** (0x0B) - Store indirect with index
12. **fork** (0x0C) - Create new process
13. **lld** (0x0D) - Long load (no modulo)
14. **lldi** (0x0E) - Long load indirect
15. **lfork** (0x0F) - Long fork (no modulo)
16. **aff** (0x10) - Display character to stdout

#### Parameter Types

- **Register**: r1-r16 (1 byte encoding)
- **Direct**: Immediate value (2 bytes, prefixed with %)
- **Indirect**: Memory address (4 bytes)
- **Label**: Symbolic references resolved at compile time

### 3. Assembler Implementation

#### Lexical Analysis

- **Tokenizer**: Parse Redcode source files
- **Label Recognition**: Identify and resolve labels
- **Instruction Parsing**: Parse opcodes and parameters
- **Comment Handling**: Support for `#` comments

#### Syntax Support

- **Header Directives**: `.name` and `.comment` directives
- **Label Definitions**: `label:` syntax
- **Parameter Formats**: Register, direct, and indirect addressing
- **Instruction Validation**: Verify parameter types and counts

#### Code Generation

- **Binary Encoding**: Generate bytecode with proper parameter encoding
- **Header Creation**: Generate champion headers with magic numbers
- **Symbol Resolution**: Resolve label references to addresses
- **Output Generation**: Create `.cor` executable files

### 4. Game Engine

#### Execution Loop

- **Cycle Management**: Track execution cycles and timing
- **Process Scheduling**: Execute processes in order
- **Death Detection**: Monitor process liveness
- **Winner Determination**: Declare winner when only one champion remains

#### Game Rules

- **Cycle to Die**: 1536 cycles before death check
- **Live Requirement**: Processes must execute `live` instruction
- **Cycle Reduction**: CYCLE_TO_DIE decreases by CYCLE_DELTA (5) after NBR_LIVE (40) lives
- **Maximum Champions**: Support up to 4 champions simultaneously

### 5. Terminal Visualization System

#### Real-time Memory Visualization

- **Memory Grid Display**: Show 6KB memory as a grid (e.g., 96x64 or 128x48)
- **Color-coded Ownership**: Different colors for each champion's memory regions
- **Live Memory Updates**: Real-time memory changes during execution
- **Memory Regions**: Highlight different memory sections (code, data, stack)

#### Process Activity Visualization

- **Process Indicators**: Show active processes with unique symbols
- **Execution Trails**: Trail effects showing recent process movement
- **Process State**: Display register values and flags for selected processes
- **Fork Visualization**: Animate process creation and forking

#### Game State Dashboard

- **Champion Status**: Live count, process count, memory usage per champion
- **Cycle Counter**: Current cycle and cycles until death check
- **Execution Statistics**: Instructions executed, memory writes, forks created
- **Performance Metrics**: Cycles per second, memory efficiency

#### Interactive Features

- **Pause/Resume**: Ability to pause and resume execution
- **Step Mode**: Single-step execution for debugging
- **Speed Control**: Adjustable execution speed (1x to 1000x)
- **Memory Inspector**: Click on memory locations to inspect values
- **Process Tracker**: Follow specific processes through execution

#### Terminal UI Components

- **Split Screen Layout**: Memory view + dashboard + controls
- **Keyboard Shortcuts**: Full keyboard control for all features
- **Mouse Support**: Mouse interaction for memory inspection
- **Responsive Design**: Adaptive layout for different terminal sizes

### 6. Command Line Interface

#### Core War Runner

```bash
corewar [options] champion1.cor [champion2.cor] [champion3.cor] [champion4.cor]
```

**Options:**

- `-v, --visual`: Enable terminal visualization
- `-d, --dump <cycles>`: Dump memory after specified cycles
- `-s, --speed <rate>`: Set execution speed (1-1000)
- `-p, --pause`: Start in paused mode
- `-n, --number <n>`: Set champion number
- `-a, --address <addr>`: Set load address
- `-c, --cycles <max>`: Set maximum cycles
- `-h, --help`: Display help information

#### Assembler

```bash
asm [options] source.s
```

**Options:**

- `-o, --output <file>`: Specify output file
- `-v, --verbose`: Verbose compilation output
- `-l, --listing`: Generate assembly listing
- `-h, --help`: Display help information

### 7. Rust-Specific Implementation Details

#### Memory Safety

- **Bounds Checking**: All memory operations with proper bounds checking
- **Safe Indexing**: Use Rust's safe indexing or explicit bounds checking
- **Memory Layout**: Efficient memory representation with Vec<u8> or similar

#### Error Handling

- **Result Types**: Use `Result<T, E>` for all fallible operations
- **Custom Error Types**: Define specific error types for different failure modes
- **Error Propagation**: Proper error propagation through the call stack

#### Performance Optimizations

- **Zero-cost Abstractions**: Leverage Rust's zero-cost abstractions
- **SIMD Operations**: Use SIMD for bulk memory operations where applicable
- **Efficient Data Structures**: Choose optimal data structures for different components

#### Concurrency Support

- **Async Execution**: Optional async execution for better performance
- **Thread Safety**: Thread-safe operations for potential multi-threading
- **Channel Communication**: Use channels for UI updates and event handling

### 8. Testing Strategy

#### Unit Tests

- **Instruction Tests**: Test each instruction individually
- **Memory Tests**: Test memory operations and addressing
- **Assembler Tests**: Test compilation of various Redcode programs
- **Game Logic Tests**: Test game rules and execution flow

#### Integration Tests

- **Champion Loading**: Test loading and executing real champions
- **Multi-champion Games**: Test games with multiple champions
- **Visualization Tests**: Test terminal UI components
- **Performance Tests**: Benchmark execution speed and memory usage

#### Test Champions

- **Simple Programs**: Basic test programs for validation
- **Complex Programs**: Advanced programs testing edge cases
- **Historical Champions**: Include classic Core War champions
- **Benchmark Suite**: Standard benchmark programs for performance testing

### 9. Documentation Requirements

#### User Documentation

- **Getting Started Guide**: Quick start tutorial
- **Redcode Language Reference**: Complete language specification
- **Terminal UI Guide**: How to use the visualization features
- **Champion Writing Tutorial**: Step-by-step guide for writing champions

#### Developer Documentation

- **Architecture Overview**: High-level system design
- **API Reference**: Complete API documentation
- **Extension Guide**: How to extend the system
- **Performance Guide**: Optimization techniques and best practices

### 10. Build and Distribution

#### Build System

- **Cargo Configuration**: Proper Cargo.toml setup
- **Feature Flags**: Optional features for different use cases
- **Cross-compilation**: Support for major platforms
- **Release Optimization**: Optimized release builds

#### Distribution

- **Crates.io**: Publish core components as crates
- **Binary Releases**: Pre-compiled binaries for major platforms
- **Package Managers**: Integration with system package managers
- **Docker Images**: Containerized versions for easy deployment

### 11. Future Enhancements

#### Advanced Visualization

- **3D Memory Visualization**: 3D representation of memory space
- **Network Play**: Support for networked tournaments
- **Replay System**: Record and replay games
- **Statistics Dashboard**: Advanced game statistics and analytics

#### Educational Features

- **Interactive Tutorials**: Built-in learning system
- **Debugging Tools**: Advanced debugging capabilities
- **Code Analysis**: Static analysis of Redcode programs
- **Tournament Management**: Built-in tournament system

## Implementation Priority

### Phase 1: Core Engine (Weeks 1-4)

1. Virtual machine core with memory management
2. Basic instruction set implementation
3. Process management and scheduling
4. Champion loading and execution

### Phase 2: Assembler (Weeks 5-6)

1. Lexical analysis and parsing
2. Code generation and optimization
3. Symbol resolution and linking
4. Error reporting and validation

### Phase 3: Game Engine (Weeks 7-8)

1. Game loop and cycle management
2. Death detection and winner determination
3. Command-line interface
4. Basic text output

### Phase 4: Terminal Visualization (Weeks 9-12)

1. Memory grid visualization
2. Process activity display
3. Interactive controls
4. Real-time updates and animations

### Phase 5: Polish and Testing (Weeks 13-16)

1. Comprehensive testing suite
2. Performance optimization
3. Documentation completion
4. User experience refinement

## Success Metrics

- **Performance**: Match or exceed C implementation speed
- **Memory Safety**: Zero segfaults or memory leaks
- **Compatibility**: 100% compatibility with existing `.cor` files
- **Usability**: Intuitive terminal interface with smooth animations
- **Code Quality**: High-quality, well-documented Rust code
- **Community**: Active user community and contributions

## Conclusion

This Rust rewrite will provide a modern, safe, and visually engaging Core War implementation that maintains full compatibility with the existing ecosystem while adding powerful new visualization and interaction capabilities. The terminal-based visualization system will bring the Core War experience to life, making it accessible to both newcomers and experienced players.

The implementation will leverage Rust's strengths in memory safety, performance, and concurrency to create a robust and efficient Core War engine that serves as both a gaming platform and an educational tool for understanding low-level programming concepts.
