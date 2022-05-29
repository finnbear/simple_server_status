use crate::{next, sanitize_division};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind};

#[derive(Debug, Default)]
pub struct RamStatus {
    total: u64,
    free: u64,
    available: u64,
    buffers: u64,
    cached: u64,
    slab_reclaimable: u64,
    swap_total: u64,
    swap_free: u64,
}

impl RamStatus {
    pub fn update(&mut self) -> io::Result<()> {
        *self = Self::default();
        *self = Self::sample()?;
        Ok(())
    }

    pub fn sample() -> io::Result<Self> {
        let proc_stat = File::open("/proc/meminfo")?;
        let reader = BufReader::new(proc_stat);
        let mut ret = Self::default();
        for line in reader.lines() {
            let line = line?;
            let mut tokens = line.split_ascii_whitespace();
            let field_name = match tokens.next() {
                Some(token) => token.trim_end_matches(':'),
                None => continue,
            };
            let field = match field_name {
                "MemTotal" => &mut ret.total,
                "MemFree" => &mut ret.free,
                "MemAvailable" => &mut ret.available,
                "Buffers" => &mut ret.buffers,
                "Cached" => &mut ret.cached,
                "SReclaimable" => &mut ret.slab_reclaimable,
                "SwapTotal" => &mut ret.swap_total,
                "SwapFree" => &mut ret.swap_free,
                _ => continue,
            };
            let value = next(&mut tokens)?;

            // Unit
            match tokens.next() {
                Some("kB") | Some("Kb") | Some("kb") | Some("KB") | None => {}
                _ => {
                    return Err(io::Error::new(
                        ErrorKind::InvalidData,
                        "/proc/meminfo unsupported unit",
                    ))
                }
            }

            // Convert to bytes.
            *field = value * 1024;
        }

        if ret.total == 0 {
            Err(io::Error::new(
                ErrorKind::InvalidData,
                "/proc/meminfo reports no memory",
            ))
        } else {
            Ok(ret)
        }
    }

    pub fn usage(&self) -> Option<f32> {
        sanitize_division(self.used(), self.total)
    }

    pub fn swap_usage(&self) -> Option<f32> {
        sanitize_division(self.swap_used(), self.swap_total)
    }

    fn used(&self) -> u64 {
        self.total
            .saturating_sub(self.free)
            .saturating_sub(self.buffers)
            .saturating_sub(self.cached)
            .saturating_sub(self.slab_reclaimable)
    }

    fn swap_used(&self) -> u64 {
        self.swap_total.saturating_sub(self.swap_free)
    }
}

#[cfg(test)]
mod tests {
    use crate::SimpleServerStatus;

    #[test]
    fn ram() {
        let mut status = SimpleServerStatus::new();

        assert_eq!(status.ram_usage(), None);
        assert_eq!(status.ram_swap_usage(), None);

        status.update().unwrap();

        let usage = status.ram_usage().unwrap();
        println!("ram_usage: {}", usage);
        assert!(usage > 0.0);
        assert!(usage <= 1.0);

        let swap_usage = status.ram_swap_usage().unwrap();
        println!("ram_swap_usage: {}", swap_usage);
        assert!(swap_usage >= 0.0);
        assert!(swap_usage <= 1.0);
    }
}
