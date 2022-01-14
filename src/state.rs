#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Loading,
    MainMenu,
    Playing,
    Paused,
}