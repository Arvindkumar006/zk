#!/bin/bash
echo "🌌 Initializing Stellar Horizon Pool Workspace Dependencies..."

# Pull down external cryptographic verifier submodules automatically
if [ ! -d "lib/rs-soroban-ultrahonk" ]; then
    echo "📦 Fetching rs-soroban-ultrahonk framework..."
    mkdir -p lib
    git clone https://github.com/yugocabrio/rs-soroban-ultrahonk.git lib/rs-soroban-ultrahonk
fi

echo "🚀 Workspace setup complete! Run 'nargo test' and 'cargo test' to verify."
