#!/bin/bash

# Boot script for Rust Core War (Terminal Game)
set -e

echo "🎮 Booting Core War..."

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo is not installed. Please install it first:"
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Build the project
echo "🔨 Building Core War engine..."
cargo build --release

echo ""
echo "🎮 Core War Game Ready!"
echo ""
echo "Usage Examples:"
echo "  ./target/release/corewar --help                              # Show all options"
echo "  ./target/release/corewar run examples/simple.cor examples/imp.cor    # Basic battle"
echo "  ./target/release/corewar run --visual examples/dwarf.cor examples/bomber.cor  # Visual mode"
echo "  ./target/release/corewar run examples/dwarf.cor examples/stone.cor examples/paper.cor  # 3-way battle"
echo "  ./target/release/corewar asm examples/simple.s              # Assemble source file"
echo "  ./target/release/corewar info examples/dwarf.cor            # Show warrior info"
echo ""

# Check for examples
if [ -d "examples" ]; then
    echo "📁 Available example warriors (.cor files):"
    ls examples/*.cor 2>/dev/null | head -8 | while read file; do
        name=$(./target/release/corewar info "$file" 2>/dev/null | grep "Name:" | cut -d: -f2 | xargs || basename "$file" .cor)
        echo "   $(basename "$file") - $name"
    done
    echo ""
    
    total_cor=$(ls examples/*.cor 2>/dev/null | wc -l | xargs)
    total_s=$(ls examples/*.s 2>/dev/null | wc -l | xargs)
    echo "   Total: $total_cor compiled warriors, $total_s source files"
    echo ""
fi

echo "🚀 Starting demo battle: Dwarf vs Bomber..."
./target/release/corewar run examples/dwarf.cor examples/bomber.cor