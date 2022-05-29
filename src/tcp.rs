use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default)]
pub struct TcpStatus {
    count: Option<usize>,
}

impl TcpStatus {
    pub fn update(&mut self) -> io::Result<()> {
        self.count = None;
        self.count = Some(Self::sample()?);
        Ok(())
    }

    fn sample() -> io::Result<usize> {
        let proc_stat = File::open("/proc/net/tcp")?;
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

    pub fn connections(&self) -> Option<usize> {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use crate::SimpleServerStatus;

    #[test]
    fn tcp() {
        let mut status = SimpleServerStatus::new();

        assert_eq!(status.tcp_connections(), None);

        status.update().unwrap();

        let connections = status.tcp_connections().unwrap();
        println!("tcp_connections: {}", connections);
    }
}
