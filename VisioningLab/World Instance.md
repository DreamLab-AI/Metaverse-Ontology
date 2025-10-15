# World Instance

## Metadata
- **Term ID**: 20318
- **Type**: VirtualObject
- **Classification**: Instance Management
- **Domain**: InfrastructureDomain
- **Layer**: MiddlewareLayer
- **Status**: Active
- **Version**: 1.0
- **Last Updated**: 2025-10-15

## Definition

### Primary Definition
A **World Instance** is a specific runtime instantiation or copy of a virtual world or environment, created to support concurrent users, sessions, or gameplay scenarios with isolated state and resources. Each instance represents an independent execution context that maintains its own entity states, physics simulation, and user interactions while sharing the same world template or blueprint.

### Operational Characteristics
- **Instancing Model**: Dynamic spawning of isolated world copies
- **Session Isolation**: Independent state management per instance
- **Resource Allocation**: Dedicated computational resources per instance
- **Load Distribution**: Player sharding across multiple instances
- **Lifecycle Management**: Creation, maintenance, and destruction protocols

## Relationships

### Parent Classes
- **VirtualWorld**: A World Instance is a specific runtime manifestation of a Virtual World
- **ComputeResource**: Requires dedicated computational infrastructure
- **SessionContext**: Operates within session boundaries
- **RuntimeEnvironment**: Functions as an isolated runtime context

### Related Concepts
- **Instance Spawning**: Process of creating new world instances
- **Player Sharding**: Distribution of users across instances
- **State Synchronization**: Maintaining instance state consistency
- **Resource Pooling**: Management of instance resources
- **Dynamic Scaling**: Automatic instance creation/destruction based on demand

## Formal Ontology

<details>
<summary>Click to expand OntologyBlock</summary>

```clojure
;; World Instance Ontology (OWL Functional Syntax)
;; Term ID: 20318
;; Domain: InfrastructureDomain | Layer: MiddlewareLayer

(Declaration (Class :WorldInstance))

;; Core Classification
(SubClassOf :WorldInstance :VirtualWorld)
(SubClassOf :WorldInstance :ComputeResource)
(SubClassOf :WorldInstance :SessionContext)
(SubClassOf :WorldInstance :RuntimeEnvironment)

;; Instance Management
(SubClassOf :WorldInstance
  (ObjectSomeValuesFrom :hasInstanceTemplate :WorldBlueprint))
(SubClassOf :WorldInstance
  (ObjectSomeValuesFrom :managesUserSession :UserSession))
(SubClassOf :WorldInstance
  (ObjectSomeValuesFrom :allocatesResourcePool :ComputeResourcePool))

;; Isolation Properties
(SubClassOf :WorldInstance
  (ObjectSomeValuesFrom :maintainsStateIsolation :IsolatedState))
(SubClassOf :WorldInstance
  (ObjectSomeValuesFrom :providesSessionBoundary :SessionBoundary))

;; Scaling Characteristics
(SubClassOf :WorldInstance
  (ObjectSomeValuesFrom :supportsPlayerSharding :PlayerShardingStrategy))
(SubClassOf :WorldInstance
  (ObjectSomeValuesFrom :enablesDynamicScaling :ScalingPolicy))

;; Lifecycle Management
(SubClassOf :WorldInstance
  (ObjectSomeValuesFrom :followsLifecycleProtocol :InstanceLifecycle))
(SubClassOf :WorldInstance
  (ObjectSomeValuesFrom :implementsLoadBalancing :LoadBalancingStrategy))

;; Disjoint Classes
(DisjointClasses :WorldInstance :WorldTemplate :PersistentWorld)

;; Object Properties
(Declaration (ObjectProperty :hasInstanceTemplate))
(Declaration (ObjectProperty :managesUserSession))
(Declaration (ObjectProperty :allocatesResourcePool))
(Declaration (ObjectProperty :maintainsStateIsolation))
(Declaration (ObjectProperty :providesSessionBoundary))
(Declaration (ObjectProperty :supportsPlayerSharding))
(Declaration (ObjectProperty :enablesDynamicScaling))
(Declaration (ObjectProperty :followsLifecycleProtocol))
(Declaration (ObjectProperty :implementsLoadBalancing))

;; Data Properties
(DataPropertyAssertion :hasInstanceID :WorldInstance "instance-uuid"^^xsd:string)
(DataPropertyAssertion :hasPlayerCapacity :WorldInstance 100^^xsd:integer)
(DataPropertyAssertion :hasLifetimeSeconds :WorldInstance 3600^^xsd:integer)
(DataPropertyAssertion :hasMemoryAllocationMB :WorldInstance 2048^^xsd:integer)

;; Annotations
(AnnotationAssertion rdfs:label :WorldInstance "World Instance"@en)
(AnnotationAssertion rdfs:comment :WorldInstance
  "Specific runtime instantiation of a virtual world for concurrent users or sessions"@en)
```
</details>

## Implementation Patterns

### Instance Spawning
```python
class WorldInstanceManager:
    """Manages world instance lifecycle and allocation"""

    def spawn_instance(self, template_id: str, config: InstanceConfig) -> WorldInstance:
        """Create new world instance from template"""
        instance = WorldInstance(
            template_id=template_id,
            instance_id=generate_uuid(),
            max_players=config.max_players,
            resource_pool=allocate_resources(config.resource_tier)
        )
        instance.initialize_state()
        return instance

    def assign_player(self, player_id: str, instance: WorldInstance):
        """Assign player to instance with load balancing"""
        if instance.current_players < instance.max_players:
            instance.add_player(player_id)
        else:
            # Find or create alternative instance
            alternative = self.find_available_instance()
            alternative.add_player(player_id)
```

### State Isolation
```javascript
class IsolatedInstanceState {
  constructor(instanceId) {
    this.instanceId = instanceId;
    this.entities = new Map();
    this.physics = new PhysicsSimulation();
    this.events = new EventQueue();
  }

  updateEntity(entityId, state) {
    // Update only affects this instance
    this.entities.set(entityId, state);
    this.physics.updateBody(entityId, state);
  }

  synchronize() {
    // Sync instance state to database (checkpoint)
    persistInstanceSnapshot(this.instanceId, this.serialize());
  }
}
```

### Dynamic Scaling
```typescript
interface ScalingPolicy {
  minInstances: number;
  maxInstances: number;
  targetPlayerRatio: number;
  scaleUpThreshold: number;
  scaleDownThreshold: number;
}

class AutoScaler {
  async evaluateScaling(policy: ScalingPolicy): Promise<void> {
    const instances = await this.getActiveInstances();
    const totalPlayers = this.getTotalPlayerCount();
    const avgLoad = totalPlayers / instances.length;

    if (avgLoad > policy.scaleUpThreshold && instances.length < policy.maxInstances) {
      await this.spawnInstance();
    } else if (avgLoad < policy.scaleDownThreshold && instances.length > policy.minInstances) {
      await this.terminateUnderutilizedInstance();
    }
  }
}
```

## Use Cases

### MMO Dungeon Instances
- **Scenario**: World of Warcraft dungeon system
- **Implementation**: Private instance per party (5 players)
- **Duration**: 1-3 hours, auto-cleanup on completion
- **Features**: Boss progression, loot distribution, difficulty scaling

### Battle Royale Island Instances
- **Scenario**: Fortnite match instances
- **Implementation**: 100-player instance per match
- **Duration**: 20-30 minutes, strict lifecycle
- **Features**: Shrinking play zone, real-time physics, matchmaking integration

### VRChat Room Instances
- **Scenario**: Social VR world instances
- **Implementation**: 20-80 player capacity per instance
- **Duration**: Persistent while users present, auto-shutdown on empty
- **Features**: User-created worlds, permission systems, portal linking

### Private Event Venues
- **Scenario**: Virtual conference or concert instances
- **Implementation**: Dedicated instance for event with access control
- **Duration**: Event duration + setup/teardown
- **Features**: Access restrictions, recording, capacity management

## Technical Considerations

### Performance Optimization
- **Resource Pooling**: Pre-allocate instance resources for fast spawning
- **State Checkpointing**: Periodic state saves for crash recovery
- **Load Balancing**: Distribute players based on latency and capacity
- **Instance Migration**: Move players between instances for optimization

### Failure Handling
- **Graceful Shutdown**: Save state and notify players before termination
- **Crash Recovery**: Restore instance from latest checkpoint
- **Player Migration**: Transfer players to backup instance on failure
- **Data Consistency**: Ensure state consistency across failures

### Monitoring Metrics
- **Instance Health**: CPU, memory, network utilization per instance
- **Player Distribution**: Player count and balance across instances
- **Lifecycle Events**: Creation, destruction, migration events
- **Performance KPIs**: Frame rate, latency, simulation accuracy

## Challenges and Solutions

### Challenge: Instance Proliferation
- **Problem**: Too many underutilized instances waste resources
- **Solution**: Aggressive consolidation policies with player migration

### Challenge: State Synchronization
- **Problem**: Keeping persistent player data synchronized across instances
- **Solution**: Centralized player state service with instance checkpoints

### Challenge: Cross-Instance Communication
- **Problem**: Players in different instances need to coordinate
- **Solution**: Global messaging system with instance-aware routing

### Challenge: Fair Distribution
- **Problem**: Ensuring balanced player experience across instances
- **Solution**: Skill-based matchmaking with instance difficulty adjustment

## Best Practices

1. **Instance Template Design**: Create reusable, parameterizable world templates
2. **Capacity Planning**: Monitor player patterns to optimize instance allocation
3. **State Management**: Implement clear boundaries between instance and persistent state
4. **Testing**: Stress-test instance spawning and destruction under load
5. **Monitoring**: Real-time dashboards for instance health and distribution
6. **Documentation**: Maintain instance configuration and scaling policies

## Related Terms
- **WorldTemplate** (20320): Blueprint for creating instances
- **SessionContext** (20115): Execution context for instance
- **PlayerSharding** (20321): Distribution strategy across instances
- **LoadBalancing** (20322): Instance selection and player assignment
- **ResourcePool** (20323): Computational resources for instances

## References
- "Scalable Game Server Architecture" - Cloud Gaming Platforms, 2023
- "Instance Management in MMO Systems" - Game Server Engineering, 2022
- "Dynamic World Scaling Patterns" - Virtual World Infrastructure, 2024
- "Session Isolation in Multi-Tenant Environments" - Distributed Systems, 2023

## Examples

### Example 1: Party Dungeon Instance (WoW-style)
```yaml
instance:
  type: dungeon
  template: "dire-maul"
  max_players: 5
  difficulty: heroic
  lifetime: 4h
  features:
    - boss_progression
    - loot_tables
    - resurrection_mechanics
    - lockout_timer
```

### Example 2: Battle Royale Match Instance
```yaml
instance:
  type: battle_royale
  template: "island-map-v3"
  max_players: 100
  lifetime: 25m
  features:
    - shrinking_zone
    - loot_spawning
    - storm_damage
    - matchmaking_integration
```

### Example 3: Social VR Room Instance
```yaml
instance:
  type: social_room
  template: "custom-world-12345"
  max_players: 40
  lifetime: until_empty
  features:
    - voice_chat
    - avatar_system
    - user_permissions
    - portal_linking
```

---

**Navigation**: [← Back to Index](../README.md) | [Domain: InfrastructureDomain](../domains/InfrastructureDomain.md) | [Layer: MiddlewareLayer](../layers/MiddlewareLayer.md)
