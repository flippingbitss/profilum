use std::time::Duration;

pub use crate::cpu::{get_cpu_freq, read_tsc};
use crate::profiler;

#[derive(Clone, Copy)]
pub struct ProfilerSlot {
    name: &'static str,
    tsc_elapsed: u64,
    tsc_elapsed_children: u64,
    hits: usize,
}

impl ProfilerSlot {
    const fn empty() -> Self {
        Self {
            name: "",
            tsc_elapsed: 0,
            tsc_elapsed_children: 0,
            hits: 0,
        }
    }
}

const SLOT_COUNT: usize = 512;

pub struct Profiler<T: Into<usize> + 'static> {
    pub slots: [ProfilerSlot; SLOT_COUNT],
    tsc_start: u64,
    tsc_end: u64,
    running: bool,
    global_parent_slot: usize,
    __marker: std::marker::PhantomData<T>,
}

impl<T: Into<usize> + 'static> Profiler<T> {
    pub const fn new() -> Self {
        Self {
            slots: [ProfilerSlot::empty(); SLOT_COUNT],
            tsc_start: 0,
            tsc_end: 0,
            global_parent_slot: 0,
            running: false,
            __marker: std::marker::PhantomData,
        }
    }

    pub fn start(&mut self) {
        self.running = true;
        self.tsc_start = read_tsc();
    }

    pub fn end(&mut self) {
        self.running = false;
        self.tsc_end = read_tsc();
    }

    pub fn get_output(&self) -> String {
        let p = self;
        let total = p.tsc_end - p.tsc_start;
        let freq = get_cpu_freq(Duration::from_millis(100));
        let total_time_in_sec = total as f64 / freq as f64 / 10.0;
        let mut output = String::new();
        output.push_str("\n======Profiler Metrics======\n");
        output.push_str(&format!(
            "Total time: {:.2}, CPU Freq: {}\n",
            total_time_in_sec, freq
        ));
        output.push_str(&format!("Total elapsed: {}\n", total));
        output.push_str("Slots: \n");
        for slot in p.slots.iter().skip(1) {
            if slot.hits != 0 {
                let line = format!(
                    "    {:<24}: Exc: ({:.2}%), Inc: ({:.2}%), hits = {}, elapsed = {} \n",
                    slot.name,
                    (slot.tsc_elapsed as f64 / total as f64) * 100.0,
                    ((slot.tsc_elapsed - slot.tsc_elapsed_children) as f64 / total as f64) * 100.0,
                    slot.hits,
                    slot.tsc_elapsed,
                );
                output.push_str(&line);
            }
        }
        output.push_str("======================\n");
        output
    }
}

pub struct ProfileScope<'a, T: Into<usize> + 'static> {
    name: &'static str,
    slot_index: usize,
    parent_slot_index: usize,
    tsc_start: u64,
    tsc_end: u64,
    profiler: &'a mut Profiler<T>,
}

impl<'a, T: Into<usize> + 'static> ProfileScope<'a, T> {
    pub fn new(
        name: impl Into<&'static str>,
        slot: impl Into<usize>,
        profiler: &'a mut Profiler<T>,
    ) -> Self {
        let parent_slot = profiler.global_parent_slot;
        let current_slot = slot.into();
        profiler.global_parent_slot = current_slot;
        let ts = read_tsc();
        Self {
            name: name.into(),
            slot_index: current_slot,
            parent_slot_index: parent_slot,
            tsc_start: ts,
            tsc_end: ts,
            profiler,
        }
    }
}

impl<'a, T: Into<usize> + 'static> Drop for ProfileScope<'a, T> {
    fn drop(&mut self) {
        self.tsc_end = read_tsc();

        // if self.parent_slot_index != self.slot_index {}

        let current_slot = &mut self.profiler.slots[self.slot_index];

        let scope_elapsed = self.tsc_end - self.tsc_start;

        current_slot.tsc_elapsed += scope_elapsed;
        current_slot.hits += 1;
        current_slot.name = self.name;

        let parent_slot = &mut self.profiler.slots[self.parent_slot_index];
        parent_slot.tsc_elapsed_children += scope_elapsed;

        self.profiler.global_parent_slot = self.parent_slot_index;
    }
}
