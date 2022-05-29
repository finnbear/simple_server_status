use crate::{delta, next, next_or_zero};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{io, mem};

#[derive(Debug, Default)]
pub struct CpuStatus {
    new: CpuCounters,
    old: CpuCounters,
}

impl CpuStatus {
    pub fn update(&mut self) -> io::Result<()> {
        self.old = mem::take(&mut self.new);
        self.new = CpuCounters::sample()?;
        Ok(())
    }

    pub fn usage(&self) -> Option<f32> {
        delta!(self.old, self.new, _use, total)
    }

    pub fn local_usage(&self) -> Option<f32> {
        delta!(self.old, self.new, local_use, total)
    }

    pub fn stolen_usage(&self) -> Option<f32> {
        delta!(self.old, self.new, stolen_use, total)
    }
}

#[derive(Debug, Default)]
struct CpuCounters {
    user: u64,
    nice: u64,
    system: u64,
    idle: u64,
    io_wait: u64,
    irq: u64,
    soft_irq: u64,
    steal: u64,
    guest: u64,
    guest_nice: u64,
}

impl CpuCounters {
    fn sample() -> io::Result<Self> {
        let proc_stat = File::open("/proc/stat")?;
        let reader = BufReader::new(proc_stat);
        let mut lines = reader.lines();
        let first_line = lines
            .next()
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "/proc/stat missing first line",
            ))?
            .map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "/proc/stat unreadable first line",
                )
            })?;
        let mut tokens = first_line.split_ascii_whitespace();
        if tokens.next() != Some("cpu") {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "/proc/stat unexpected string",
            ));
        }

        // https://man7.org/linux/man-pages/man5/proc.5.html
        Ok(Self {
            // These were there since the beginning, so do fail if they don't exist.
            user: next(&mut tokens)?,
            nice: next(&mut tokens)?,
            system: next(&mut tokens)?,
            idle: next(&mut tokens)?,
            // These were not there since the beginning, so don't fail if they don't exist.
            io_wait: next_or_zero(&mut tokens)?,
            irq: next_or_zero(&mut tokens)?,
            soft_irq: next_or_zero(&mut tokens)?,
            steal: next_or_zero(&mut tokens)?,
            guest: next_or_zero(&mut tokens)?,
            guest_nice: next_or_zero(&mut tokens)?,
        })
    }

    fn local_use(&self) -> u64 {
        self.user
            .saturating_add(self.nice)
            .saturating_add(self.system)
            .saturating_add(self.irq)
            .saturating_add(self.soft_irq)
            .saturating_add(self.guest)
            .saturating_add(self.guest_nice)
    }

    fn stolen_use(&self) -> u64 {
        self.steal
    }

    fn _use(&self) -> u64 {
        self.local_use().saturating_add(self.stolen_use())
    }

    fn total(&self) -> u64 {
        self._use()
            .saturating_add(self.idle)
            .saturating_add(self.io_wait)
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::CpuCounters;
    use crate::SimpleServerStatus;
    use std::time::Duration;

    #[test]
    fn cpu() {
        let mut status = SimpleServerStatus::new();

        assert_eq!(status.cpu_usage(), None);
        assert_eq!(status.cpu_local_usage(), None);
        assert_eq!(status.cpu_stolen_usage(), None);

        status.update().unwrap();

        assert!(status.cpu_usage().is_some());
        assert!(status.cpu_local_usage().is_some());
        assert!(status.cpu_stolen_usage().is_some());

        std::thread::sleep(Duration::from_millis(100));
        status.update().unwrap();

        let usage = status.cpu_usage().unwrap();
        println!("cpu_usage: {}", usage);
        assert!(usage > 0.0);
        assert!(usage <= 1.0);

        let local_usage = status.cpu_local_usage().unwrap();
        println!("cpu_local_usage: {}", local_usage);
        assert!(local_usage > 0.0);
        assert!(local_usage <= 1.0);

        let stolen_usage = status.cpu_stolen_usage().unwrap();
        println!("cpu_stolen_usage: {}", stolen_usage);
        assert!(stolen_usage >= 0.0);
        assert!(stolen_usage <= 1.0);
    }

    #[test]
    fn cpu_counters() {
        let counters = CpuCounters::sample().unwrap();
        println!("cpu_counters: {:?}", counters);
    }
}
