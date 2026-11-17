// Called when a player interacts with a block.
fn pipez_player_interact(game: &mut crate::game::Game<'_>, context: &crate::game::native_hooks::PlayerInteractContext) {
    if !game.native_on(crate::game::pipez::ID) {
        return;
    }
    crate::game::pipez::on_player_interact(
        context.held_item(),
        context.block_pos(),
        context.is_sneaking(),
        context.is_right_click(),
    );
}