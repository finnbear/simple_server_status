use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default)]
pub struct UdpStatus {
    count: Option<usize>,
}

impl UdpStatus {
    pub fn update(&mut self) -> io::Result<()> {
        self.count = None;
        self.count = Some(Self::sample()?);
        Ok(())
    }

    fn sample() -> io::Result<usize> {
        let proc_stat = File::open("/proc/net/udp")?;
        let reader = BufReader::new(proc_stat);
        let mut ret = 0usize;
        for line in reader.lines() {
            let line = line?;
            if line.contains(':') {
                ret = ret.saturating_add(1);
            }
        }
        Ok(ret)
    }

    pub fn sockets(&self) -> Option<usize> {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use crate::SimpleServerStatus;

    #[test]
    fn udp() {
        let mut status = SimpleServerStatus::default();

        assert_eq!(status.udp_sockets(), None);

        status.update().unwrap();

        let connections = status.udp_sockets().unwrap();
        println!("udp_sockets: {}", connections);
    }
}
