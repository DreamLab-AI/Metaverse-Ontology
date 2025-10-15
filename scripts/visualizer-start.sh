#!/bin/bash
# OWL TTL Web Visualizer - Start Script

set -e

VISUALIZER_DIR="/mnt/mldata/githubs/OntologyDesign/visualizer"

echo "🚀 Starting OWL TTL Web Visualizer..."

cd "$VISUALIZER_DIR"

# Create persistent data directories if they don't exist
mkdir -p persistent_data/{data,logs,code_logs/kg}

# Start containers
docker-compose up -d

echo ""
echo "✅ Visualizer is starting..."
echo ""
echo "📊 Services:"
echo "   - Web Interface: http://localhost:5005"
echo "   - Neo4j Browser: http://localhost:7474"
echo "   - Neo4j Credentials: neo4j/ioana123"
echo ""
echo "📝 To view logs:"
echo "   docker-compose logs -f"
echo ""
echo "🛑 To stop:"
echo "   ./scripts/visualizer-stop.sh"
echo ""
echo "⏳ Waiting for services to be ready (30s)..."
sleep 30

# Check if services are up
if docker-compose ps | grep -q "Up"; then
    echo "✅ Services are running!"
    echo ""
    echo "🌐 Open http://localhost:5005 to start visualizing your TTL files"
else
    echo "❌ Services failed to start. Check logs with: docker-compose logs"
    exit 1
fi
