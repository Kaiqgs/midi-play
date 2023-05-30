#[derive(Clone, Default)]
pub struct TrackWindowContext {
    pub zoom: f64,
    pub range: Option<(f64, f64)>,
    pub note_count: usize,
}

impl TrackWindowContext {
    pub fn new(note_count: usize, zoom: Option<f64>, range: Option<(f64, f64)>) -> Self {
        TrackWindowContext {
            note_count,
            zoom: zoom.unwrap_or(1.0),
            range,
        }
    }
}
