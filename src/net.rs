use crate::{next, unix_millis};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{io, mem};

#[derive(Debug, Default)]
pub struct NetStatus {
    old: NetCounters,
    new: NetCounters,
}

#[derive(Debug, Default)]
#[allow(unused)]
struct NetCounters {
    unix_millis: u64,
    rx_bytes: u64,
    rx_packets: u64,
    rx_errors: u64,
    rx_dropped: u64,
    rx_fifo_errors: u64,
    rx_frame_errors: u64,
    rx_compressed: u64,
    rx_multicast: u64,
    tx_bytes: u64,
    tx_packets: u64,
    tx_errors: u64,
    tx_dropped: u64,
    tx_fifo_errors: u64,
    tx_collisions: u64,
    tx_carrier_errors: u64,
    tx_compressed: u64,
}

impl NetStatus {
    pub fn update(&mut self) -> io::Result<()> {
        self.old = mem::take(&mut self.new);
        self.new = NetCounters::sample()?;
        Ok(())
    }

    pub fn bandwidth(&self) -> Option<u64> {
        self.rate(|counters| counters.rx_bytes.saturating_add(counters.tx_bytes))
    }

    pub fn reception_bandwidth(&self) -> Option<u64> {
        self.rate(|counters| counters.rx_bytes)
    }

    pub fn transmission_bandwidth(&self) -> Option<u64> {
        self.rate(|counters| counters.tx_bytes)
    }

    /// Calculates rate of change per second.
    fn rate(&self, bytes: impl Fn(&NetCounters) -> u64) -> Option<u64> {
        let bytes = bytes(&self.new).saturating_sub(bytes(&self.old));
        let millis = self.new.unix_millis.saturating_sub(self.old.unix_millis);
        (bytes * 1000).checked_div(millis)
    }
}

impl NetCounters {
    fn sample() -> io::Result<Self> {
        let proc_stat = File::open("/proc/net/dev")?;
        let reader = BufReader::new(proc_stat);
        let mut ret = Self::default();
        ret.unix_millis = unix_millis();
        for line in reader.lines().skip(2) {
            let line = line?;
            let mut tokens = line.split_ascii_whitespace();

            match tokens.next() {
                Some("lo:") | None => continue,
                _ => {}
            };

            ret.rx_bytes = ret.rx_bytes.saturating_add(next(&mut tokens)?);
            ret.rx_packets = ret.rx_packets.saturating_add(next(&mut tokens)?);
            ret.rx_errors = ret.rx_errors.saturating_add(next(&mut tokens)?);
            ret.rx_dropped = ret.rx_dropped.saturating_add(next(&mut tokens)?);
            ret.rx_fifo_errors = ret.rx_fifo_errors.saturating_add(next(&mut tokens)?);
            ret.rx_frame_errors = ret.rx_frame_errors.saturating_add(next(&mut tokens)?);
            ret.rx_compressed = ret.rx_compressed.saturating_add(next(&mut tokens)?);
            ret.rx_multicast = ret.rx_multicast.saturating_add(next(&mut tokens)?);
            ret.tx_bytes = ret.tx_bytes.saturating_add(next(&mut tokens)?);
            ret.tx_packets = ret.tx_packets.saturating_add(next(&mut tokens)?);
            ret.tx_errors = ret.tx_errors.saturating_add(next(&mut tokens)?);
            ret.tx_dropped = ret.tx_dropped.saturating_add(next(&mut tokens)?);
            ret.tx_fifo_errors = ret.tx_fifo_errors.saturating_add(next(&mut tokens)?);
            ret.tx_collisions = ret.tx_collisions.saturating_add(next(&mut tokens)?);
            ret.tx_carrier_errors = ret.tx_carrier_errors.saturating_add(next(&mut tokens)?);
            ret.tx_compressed = ret.tx_compressed.saturating_add(next(&mut tokens)?);
        }
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use crate::net::NetCounters;
    use crate::SimpleServerStatus;
    use std::time::Duration;

    #[test]
    fn net() {
        let mut status = SimpleServerStatus::default();

        assert_eq!(status.net_bandwidth(), None);
        assert_eq!(status.net_reception_bandwidth(), None);
        assert_eq!(status.net_transmission_bandwidth(), None);

        status.update().unwrap();

        assert!(status.net_bandwidth().is_some());
        assert!(status.net_reception_bandwidth().is_some());
        assert!(status.net_transmission_bandwidth().is_some());

        std::thread::sleep(Duration::from_millis(250));
        status.update().unwrap();

        let bandwidth = status.net_bandwidth().unwrap();
        println!("net_bandwidth: {}", bandwidth);
        let reception_bandwidth = status.net_reception_bandwidth().unwrap();
        println!("net_reception_bandwidth: {}", reception_bandwidth);
        let transmission_bandwidth = status.net_transmission_bandwidth().unwrap();
        println!("net_transmission_bandwidth: {}", transmission_bandwidth);
    }

    #[test]
    fn net_counters() {
        let counters = NetCounters::sample().unwrap();
        println!("net_counters: {:?}", counters);
    }
}
