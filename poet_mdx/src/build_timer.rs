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
        let elapsed_milliseconds = self.started_at.elapsed().as_millis();

        info!("Finished in {elapsed_milliseconds} milliseconds");
    }
}
