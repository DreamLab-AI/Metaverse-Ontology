#!/bin/bash
# OWL TTL Web Visualizer - Stop Script

set -e

VISUALIZER_DIR="/mnt/mldata/githubs/OntologyDesign/visualizer"

echo "🛑 Stopping OWL TTL Web Visualizer..."

cd "$VISUALIZER_DIR"

docker-compose down

echo "✅ Visualizer stopped"
echo ""
echo "💡 To start again: ./scripts/visualizer-start.sh"
echo "🗑️  To remove all data: rm -rf visualizer/persistent_data"
