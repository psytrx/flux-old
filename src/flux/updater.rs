use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use log::debug;

use super::Film;

pub trait RenderUpdater: Sync {
    fn should_update(&self, current_pass: usize, total_passes: usize) -> bool;
    fn update(&self, event: RenderUpdateEvent);
}

pub struct RenderUpdateEvent {
    pub current_pass: usize,
    pub progress_percent: f32,
    pub film: Film,
    pub total_passes: usize,
}

pub struct DefaultRenderUpdater {
    interval: Duration,
    filepath: PathBuf,
    num_cpus: usize,
    last_update: Arc<Mutex<Instant>>,
}

impl DefaultRenderUpdater {
    pub fn new(interval: Duration, filepath: PathBuf) -> Self {
        let last_update = Arc::new(Mutex::new(Instant::now()));
        let num_cpus = num_cpus::get();
        Self {
            interval,
            filepath,
            num_cpus,
            last_update,
        }
    }
}

impl RenderUpdater for DefaultRenderUpdater {
    fn should_update(&self, current_pass: usize, total_passes: usize) -> bool {
        let last_update = self.last_update.lock().unwrap();
        current_pass % self.num_cpus == 0
            && current_pass < total_passes
            && last_update.elapsed() > self.interval
    }

    fn update(&self, evt: RenderUpdateEvent) {
        debug!(
            "pass {} / {}\t({:>6.3}%)",
            evt.current_pass, evt.total_passes, evt.progress_percent
        );
        evt.film.to_srgb_image().save(&self.filepath).unwrap();

        let mut last_update = self.last_update.lock().unwrap();
        *last_update = Instant::now();
    }
}
