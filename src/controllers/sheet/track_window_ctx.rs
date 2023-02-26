use crate::models::sheet::track_window_ctx::TrackWindowContext;

impl TrackWindowContext {
    pub fn new(zoom: Option<f64>, range: Option<(f64, f64)>) -> Self {
        TrackWindowContext {
            zoom: zoom.unwrap_or(1.0),
            range,
        }
    }

    // get zoom
    // match with
    // p5.prototype.map = function(n, start1, stop1, start2, stop2) {
    //  return ((n-start1)/(stop1-start1))*(stop2-start2)+start2;
    // };
    pub fn get_zoom(&self) -> f64 {
        match self.range {
            Some(range) => {
                let (start1, stop1) = (0.0, 1.0);
                let (start2, stop2) = range;
                ((self.zoom.clamp(start1, stop1) - start1) / (stop1 - start1)) * (stop2 - start2)
                    + start2
            }
            None => self.zoom,
        }
    }
}

impl Clone for TrackWindowContext {
    fn clone(&self) -> Self {
        TrackWindowContext {
            zoom: self.zoom,
            range: self.range,
        }
    }
}

impl Default for TrackWindowContext {
    fn default() -> Self {
        TrackWindowContext::new(None, None)
    }
}
