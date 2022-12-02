#[cfg(test)]
mod tests {
    use {crate::models::Pausable, crate::MidiPlay};
    
    fn setup() -> MidiPlay {
        return MidiPlay::new(192, 192);
    }
    #[test]
    fn force_resume() {
        let mut midi = setup();
        assert!(!midi.resume());
        assert!(!midi.resume());
    }
    #[test]
    fn attempt_resume() {
        let mut midi = setup();
        midi.pause();
        assert!(midi.resume());
    }

    #[test]
    fn force_pause() {
        let mut midi = setup();
        assert!(midi.pause());
        assert!(!midi.pause());
        assert!(!midi.pause());
    }

    #[test]
    fn attempt_pause() {
        let mut midi = setup();
        assert!(midi.pause());
        assert!(!midi.pause());
        assert!(!midi.pause());
    }

    #[tokio::test]
    async fn quit() {
        let mut midi = setup();
        assert!(midi.quit().await);
    }
}
