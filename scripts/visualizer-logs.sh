#!/bin/bash
# OWL TTL Web Visualizer - View Logs

VISUALIZER_DIR="/mnt/mldata/githubs/OntologyDesign/visualizer"

cd "$VISUALIZER_DIR"

if [ "$1" == "kg" ]; then
    echo "📋 Viewing KG application logs..."
    docker-compose logs -f kg
elif [ "$1" == "neo4j" ]; then
    echo "📋 Viewing Neo4j logs..."
    docker-compose logs -f neo4j
else
    echo "📋 Viewing all logs..."
    docker-compose logs -f
fi
