let attack_pattern = EntityAttackPattern::new(vec!["test_attack".to_string()], vec![0.1]);
world.add_entity_archetype(
    String::from("Test"),
    vec![
        EntityTags::MovementSpeed(5.0),
        EntityTags::Range(47),
        EntityTags::AggroRange(1000),
        EntityTags::Aggressive,
        EntityTags::FollowsPlayer,
        EntityTags::RespectsCollision,
        EntityTags::HasCollision(
            entity_components::CollisionBox{
                w: 32.0,
                h: 32.0,
                x_offset: 0.0,
                y_offset: 0.0
            }
        ),
        EntityTags::Attacks(attack_pattern),
    ]
);
world.set_entity_archetype(entity, String::from("Test"));
world.add_attack_component(entity, entity_components::EntityAttackComponent::default());
world.add_health_component(entity, entity_components::HealthComponent{health: 100.0, max_health: 100});
world.add_pathfinding_component(entity, entity_components::PathfindingComponent::default());
world.add_aggro_component(entity, entity_components::AggroComponent::default());




