// Called every game tick when in a world.
fn pipez_tick(game: &mut crate::game::Game<'_>) {
    // Turned off in Options > Mods: nothing until the next build leaves it out.
    if !game.native_on(crate::game::pipez::ID) {
        return;
    }
    crate::game::pipez::tick();
}