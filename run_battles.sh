#!/bin/bash

# Core War Battle Script
# Assembles all champions and runs battles between them

set -e  # Exit on any error

echo "🦀 Core War Battle Script"
echo "========================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Champions to assemble and battle
CHAMPIONS=("simple" "imp" "dwarf" "paper" "stone")

echo -e "${BLUE}📦 Assembling champions...${NC}"

# Assemble all champions
for champion in "${CHAMPIONS[@]}"; do
    echo -e "  ${YELLOW}Assembling ${champion}.s...${NC}"
    cargo run -- asm "examples/${champion}.s" -o "examples/${champion}.cor"
    if [ $? -eq 0 ]; then
        echo -e "  ${GREEN}✓ ${champion}.cor created${NC}"
    else
        echo -e "  ${RED}✗ Failed to assemble ${champion}.s${NC}"
        exit 1
    fi
done

echo ""
echo -e "${BLUE}⚔️  Running battles...${NC}"
echo ""

# Function to run a battle
run_battle() {
    local champ1=$1
    local champ2=$2
    echo -e "${YELLOW}🥊 Battle: ${champ1} vs ${champ2}${NC}"
    echo "----------------------------------------"
    
    # Run the battle and capture output
    cargo run -- run "examples/${champ1}.cor" "examples/${champ2}.cor" --cycles 10000 --speed 1000 2>&1
    
    echo ""
    echo "----------------------------------------"
    echo ""
}

# Run all possible battles (each champion against every other)
for i in "${!CHAMPIONS[@]}"; do
    for j in "${!CHAMPIONS[@]}"; do
        if [ $i -lt $j ]; then  # Only run each pair once
            run_battle "${CHAMPIONS[$i]}" "${CHAMPIONS[$j]}"
        fi
    done
done

echo -e "${GREEN}🏆 All battles completed!${NC}"
echo ""
echo -e "${BLUE}💡 Tips:${NC}"
echo "  • Use --visual flag for terminal visualization"
echo "  • Use --cycles <num> to limit battle length"
echo "  • Use --pause to start battles paused"
echo ""
echo "Example:"
echo "  cargo run -- run examples/imp.cor examples/dwarf.cor --visual --cycles 10000" 