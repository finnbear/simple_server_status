use std::io;
use std::str::SplitAsciiWhitespace;

#[cfg(feature = "cpu")]
mod cpu;
#[cfg(feature = "ram")]
mod ram;
#[cfg(feature = "tcp")]
mod tcp;

/// Provides simple APIs to measure status of Linux servers.
pub struct SimpleServerStatus {
    #[cfg(feature = "cpu")]
    cpu: cpu::CpuStatus,
    #[cfg(feature = "ram")]
    ram: ram::RamStatus,
    #[cfg(feature = "tcp")]
    tcp: tcp::TcpStatus,
}

impl SimpleServerStatus {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "cpu")]
            cpu: cpu::CpuStatus::default(),
            #[cfg(feature = "ram")]
            ram: ram::RamStatus::default(),
            #[cfg(feature = "tcp")]
            tcp: tcp::TcpStatus::default(),
        }
    }

    /// Make a new measurement, clearing the old one.
    pub fn update(&mut self) -> io::Result<()> {
        #[cfg(feature = "cpu")]
        self.cpu.update()?;
        #[cfg(feature = "ram")]
        self.ram.update()?;
        #[cfg(feature = "tcp")]
        self.tcp.update()?;
        Ok(())
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
    token
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "could not parse u64"))
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
