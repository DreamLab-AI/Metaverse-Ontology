# OWL TTL Web Visualizer Guide

## Overview

The OWL TTL Web Visualizer is a 3D web-based tool for exploring and visualizing OWL ontologies in Turtle (.ttl) format. It uses Neo4j as a graph database and provides an interactive 3D force-directed graph visualization.

## Architecture

- **Web Application**: Flask-based Python app serving the visualization interface
- **Database**: Neo4j graph database for storing ontology structure
- **Visualization**: 3D Force-Directed Graph library for interactive exploration

## Quick Start

### Starting the Visualizer

```bash
cd /mnt/mldata/githubs/OntologyDesign
./scripts/visualizer-start.sh
```

This will:
1. Create necessary data directories
2. Start Neo4j database container
3. Start the web application container
4. Wait for services to be ready

### Accessing the Interface

- **Web Interface**: http://localhost:5005
- **Neo4j Browser**: http://localhost:7474
  - Username: `neo4j`
  - Password: `ioana123`

### Stopping the Visualizer

```bash
./scripts/visualizer-stop.sh
```

### Viewing Logs

```bash
# All logs
./scripts/visualizer-logs.sh

# Application logs only
./scripts/visualizer-logs.sh kg

# Database logs only
./scripts/visualizer-logs.sh neo4j
```

## Using the Visualizer

### 1. Upload TTL File

1. Open http://localhost:5005
2. Click the "Load" button
3. Select your `.ttl` ontology file
4. The system will parse the file and create a Neo4j database

### 2. Explore the Ontology

**Initial View**: The visualizer displays the first 10 nodes with their connections.

**Navigation**:
- **Rotate**: Click and drag
- **Zoom**: Scroll wheel
- **Pan**: Right-click and drag

**Interactions**:
- **Expand Node**: Click on a node to reveal more connections
- **Remove Node**: Delete nodes from the visualization (doesn't affect database)
- **Search**: Use the search box to find specific nodes (case-insensitive)

### 3. Available TTL Files

The OntologyDesign project includes several TTL files you can visualize:

- `ontology.ttl` - Main ontology (281 concepts)
- `examples/*.ttl` - Example ontology files
- Any custom TTL files you create

## Features

### Interactive 3D Visualization
- Real-time force-directed graph layout
- Smooth animations and transitions
- Hardware-accelerated rendering

### Node Operations
- **Click to Expand**: Reveal additional connections
- **Dynamic Loading**: Load nodes on-demand for performance
- **Visual Deletion**: Remove nodes from view without affecting data

### Search Functionality
- Case-insensitive text search
- Highlights matching nodes
- Searches node labels and properties

### Database Integration
- Automatic Neo4j database creation from TTL files
- Persistent storage in `visualizer/persistent_data/`
- Direct Cypher query access via Neo4j Browser

## File Structure

```
visualizer/
├── docker-compose.yml           # Container orchestration
├── kg/                          # Application code
│   ├── API/                     # Flask application
│   ├── ExploreOWL/             # TTL parsing
│   ├── Neo4JConnector/         # Database interface
│   └── DataObject/             # Data models
├── persistent_data/            # Persistent storage
│   ├── data/                   # Neo4j database
│   ├── logs/                   # Neo4j logs
│   └── code_logs/kg/          # Application logs
└── ress/                       # Resources
```

## Troubleshooting

### Port Conflicts

If ports 5005, 7474, or 7687 are in use:

1. Stop conflicting containers:
   ```bash
   docker ps | grep -E "neo4j|5005"
   docker stop <container-id>
   ```

2. Or modify `visualizer/docker-compose.yml` to use different ports

### Container Issues

```bash
# Check container status
docker-compose ps

# View detailed logs
docker-compose logs -f

# Restart containers
docker-compose restart

# Rebuild from scratch
docker-compose down
docker-compose up --build
```

### Database Issues

1. Access Neo4j Browser at http://localhost:7474
2. Login with `neo4j` / `ioana123`
3. Run diagnostic queries:
   ```cypher
   // Count all nodes
   MATCH (n) RETURN count(n)

   // List node types
   MATCH (n) RETURN DISTINCT labels(n), count(n)

   // Show sample data
   MATCH (n) RETURN n LIMIT 10
   ```

### Clear All Data

```bash
./scripts/visualizer-stop.sh
rm -rf visualizer/persistent_data
./scripts/visualizer-start.sh
```

## Performance Tips

1. **Large Ontologies**: Start with search/filter rather than loading all nodes
2. **Memory**: Neo4j uses significant RAM; ensure adequate system resources
3. **Browser**: Use Chrome or Firefox for best WebGL performance

## Advanced Usage

### Direct Neo4j Access

You can run custom Cypher queries via the Neo4j Browser:

```cypher
// Find specific class
MATCH (n:Class {name: "Dataset"})
RETURN n

// Find all relationships
MATCH (a)-[r]->(b)
RETURN type(r), count(r)

// Export subgraph
MATCH path = (n)-[*1..2]-(m)
WHERE n.name = "Concept"
RETURN path
```

### Backup Database

```bash
# Create backup
docker exec visualizer-neo4j-1 neo4j-admin database dump neo4j --to-path=/backups

# Restore backup
docker exec visualizer-neo4j-1 neo4j-admin database load neo4j --from-path=/backups
```

## Credits

Based on the [OWL TTL Web Visualizer](https://github.com/cheresioana/owl_ttl_web_visualizer) by Ioana Cheres, developed as part of the MindBugs Discovery Project.

Funded by the European Union under the NGI Search project (grant agreement No 101069364).
