// Called when a block is broken.
fn pipez_block_break(game: &mut crate::game::Game<'_>, context: &crate::game::native_hooks::BlockBreakContext) {
    if !game.native_on(crate::game::pipez::ID) {
        return;
    }
    crate::game::pipez::on_block_break(context.block_pos());
}