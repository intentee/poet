use std::time::Instant;

use log::info;

pub struct BuildTimer {
    started_at: Instant,
}

impl Default for BuildTimer {
    fn default() -> Self {
        Self {
            started_at: Instant::now(),
        }
    }
}

impl Drop for BuildTimer {
    fn drop(&mut self) {
        info!(
            "Finished in {} milliseconds",
            self.started_at.elapsed().as_millis()
        )
    }
}
