use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};

const BAR_SIZE: usize = 20;

pub struct ProgressBar {
    pos: AtomicUsize,
    max: usize
}

impl ProgressBar {
    pub fn new(max: usize) -> Self {
        let pb = Self {
            pos: AtomicUsize::new(0),
            max
        };

        pb.update();
        pb
    }

    pub fn update(&self) {
        let curr = self.pos.fetch_add(1, SeqCst);
        let ratio = (curr as f32) / (self.max as f32);
        let bar_ratio = (ratio * (BAR_SIZE as f32)) as usize;

        print!("\r[{}{}] | {}% | {}/{}", "#".repeat(bar_ratio), " ".repeat(BAR_SIZE - bar_ratio), (ratio * 100.0) as usize, curr, self.max);
    }

    pub fn end(self) {
        print!("\r");
    }
}
