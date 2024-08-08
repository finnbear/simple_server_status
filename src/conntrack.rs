use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default)]
pub struct ConntrackStatus {
    count: Option<usize>,
}

impl ConntrackStatus {
    pub fn update(&mut self) -> io::Result<()> {
        self.count = None;
        self.count = Some(Self::sample()?);
        Ok(())
    }

    fn sample() -> io::Result<usize> {
        let proc_stat = File::open("/proc/net/nf_conntrack")?;
        let reader = BufReader::new(proc_stat);
        let mut ret = 0usize;
        for line in reader.lines() {
            line?;
            ret = ret.saturating_add(1);
        }
        Ok(ret)
    }

    pub fn sessions(&self) -> Option<usize> {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use crate::SimpleServerStatus;

    #[test]
    fn conntrack() {
        let mut status = SimpleServerStatus::default();

        assert_eq!(status.conntrack_sessions(), None);

        status.update().unwrap();

        let connections = status.conntrack_sessions().unwrap();
        println!("conntrack_sessions: {}", connections);
    }
}
