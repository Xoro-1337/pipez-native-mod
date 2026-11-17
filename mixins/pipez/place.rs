// Called when a block is placed.
fn pipez_block_place(game: &mut crate::game::Game<'_>, context: &crate::game::native_hooks::BlockPlaceContext) {
    if !game.native_on(crate::game::pipez::ID) {
        return;
    }
    crate::game::pipez::on_block_place(
        context.block_pos(),
        context.block_id(),
        |pos| context.get_block(pos),
    );
}