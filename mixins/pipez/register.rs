// Called when block entity types are registered.
fn pipez_register_block_entities(types: &mut Vec<crate::game::native_hooks::BlockEntityType>) {
    for entity_type in crate::game::pipez::register_block_entities() {
        types.push(entity_type);
    }
}