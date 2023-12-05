use std::io;
use std::str::SplitAsciiWhitespace;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(feature = "cpu")]
mod cpu;
#[cfg(feature = "net")]
mod net;
#[cfg(feature = "ram")]
mod ram;
#[cfg(feature = "tcp")]
mod tcp;

/// Provides simple APIs to measure status of Linux servers.
#[derive(Default)]
pub struct SimpleServerStatus {
    #[cfg(feature = "cpu")]
    cpu: cpu::CpuStatus,
    #[cfg(feature = "net")]
    net: net::NetStatus,
    #[cfg(feature = "ram")]
    ram: ram::RamStatus,
    #[cfg(feature = "tcp")]
    tcp: tcp::TcpStatus,
}

impl SimpleServerStatus {
    #[deprecated = "use Default::default()"]
    pub fn new() -> Self {
        Self::default()
    }

    /// Make a new measurement, clearing the old one.
    ///
    /// If an error occurs while updating any one component, all the other updates will still be
    /// attempted. A maximum of one error will be returned.
    pub fn update(&mut self) -> io::Result<()> {
        #[allow(unused_mut)]
        let mut result = Ok(());
        #[cfg(feature = "cpu")]
        {
            result = self.cpu.update().and(result);
        }
        #[cfg(feature = "net")]
        {
            result = self.net.update().and(result);
        }
        #[cfg(feature = "ram")]
        {
            result = self.ram.update().and(result);
        }
        #[cfg(feature = "tcp")]
        {
            result = self.tcp.update().and(result);
        }
        result
    }

    /// Returns the fraction (0.0..=1.0) of cpu used between the last two calls to `update`.
    #[cfg(feature = "cpu")]
    pub fn cpu_usage(&self) -> Option<f32> {
        self.cpu.usage()
    }

    /// Returns the fraction (0.0..=1.0) of cpu used local to the current OS between the last two
    /// calls to `update`.
    #[cfg(feature = "cpu")]
    pub fn cpu_local_usage(&self) -> Option<f32> {
        self.cpu.local_usage()
    }

    /// Returns the fraction (0.0..=1.0) of cpu used outside of the current OS (i.e. the hypervisor)
    /// between the last two calls to `update`.
    #[cfg(feature = "cpu")]
    pub fn cpu_stolen_usage(&self) -> Option<f32> {
        self.cpu.stolen_usage()
    }

    /// Returns the average transmitted/received bytes per second between the last two calls to `update`.
    ///
    /// Aggregates all network interfaces (except `lo`).
    #[cfg(feature = "net")]
    pub fn net_bandwidth(&self) -> Option<u64> {
        self.net.bandwidth()
    }

    /// Returns the average received bytes per second between the last two calls to `update`.
    ///
    /// Aggregates all network interfaces (except `lo`).
    #[cfg(feature = "net")]
    pub fn net_reception_bandwidth(&self) -> Option<u64> {
        self.net.reception_bandwidth()
    }

    /// Returns the average transmitted bytes per second between the last two calls to `update`.
    ///
    /// Aggregates all network interfaces (except `lo`).
    #[cfg(feature = "net")]
    pub fn net_transmission_bandwidth(&self) -> Option<u64> {
        self.net.transmission_bandwidth()
    }

    /// Returns the fraction (0.0..=1.0) of ram used as of the last call to `update`.
    #[cfg(feature = "ram")]
    pub fn ram_usage(&self) -> Option<f32> {
        self.ram.usage()
    }

    /// Returns the fraction (0.0..=1.0) of ram swap used as of the last call to `update`.
    #[cfg(feature = "ram")]
    pub fn ram_swap_usage(&self) -> Option<f32> {
        self.ram.swap_usage()
    }

    /// Returns the number of TCP connections as of the last call to `update`.
    #[cfg(feature = "tcp")]
    pub fn tcp_connections(&self) -> Option<usize> {
        self.tcp.connections()
    }
}

/// Parse the next u64 from a string of tokens. Will return error if it doesn't exist or could not
/// be parsed.
#[allow(unused)]
fn next(tokens: &mut SplitAsciiWhitespace) -> io::Result<u64> {
    let token = tokens
        .next()
        .ok_or(io::Error::new(io::ErrorKind::UnexpectedEof, "missing u64"))?;
    token.parse().map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("could not parse '{}' as u64: {:?}", token, e),
        )
    })
}

/// Parses the next u64 from a string of tokens. Will return error if it couldn't be parsed, and 0
/// if it doesn't exist.
#[allow(unused)]
fn next_or_zero(tokens: &mut SplitAsciiWhitespace) -> io::Result<u64> {
    let token = tokens
        .next()
        .ok_or(io::Error::new(io::ErrorKind::UnexpectedEof, "missing u64"))?;
    Ok(token.parse::<u64>().unwrap_or(0))
}

/// Outputs between 0 and 1 (None in the case of dividing by 0).
#[allow(unused)]
fn sanitize_division(numerator: u64, denominator: u64) -> Option<f32> {
    if denominator == 0 {
        None
    } else {
        let ret = numerator as f64 / denominator as f64;
        if ret.is_finite() {
            Some(ret.clamp(0.0, 1.0) as f32)
        } else {
            None
        }
    }
}

#[allow(unused)]
fn unix_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

// CPU, net, etc. measurements are relative.
#[macro_export]
macro_rules! delta {
    ($old: expr, $new: expr, $numerator: ident, $denominator: ident) => {{
        let numerator = $new.$numerator().saturating_sub($old.$numerator());
        let denominator = $new.$denominator().saturating_sub($old.$denominator());
        crate::sanitize_division(numerator, denominator)
    }};
}
