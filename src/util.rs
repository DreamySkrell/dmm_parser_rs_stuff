use std::time::SystemTime;

pub struct ModuleHelper {
    start_time: instant::Instant,
    name: String,
}

impl ModuleHelper {
    pub fn new(name: &str) -> Self {
        log::info!("{}; start...;", name);
        let start_time = instant::Instant::now();
        ModuleHelper {
            start_time,
            name: name.to_string(),
        }
    }
}

impl Drop for ModuleHelper {
    fn drop(&mut self) {
        let end_time = instant::Instant::now();
        let duration = end_time - self.start_time;

        log::info!(
            "{}; end; took: {}ms; or: {}s;",
            self.name,
            duration.as_millis(),
            duration.as_secs_f32(),
        );
    }
}
