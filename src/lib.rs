//! # dsmr-parse
//!
//! A small library for parsing DSMR 5 (Dutch Smart Meter Requirements) telegrams from Dutch electricity meters.
//!
//! The central struct is [Telegram] with its single method [Telegram::read_from]. Pass anything that implements `std::io::Read`
//! to it, and it will try to read a valid and conforming DSMR telegram with the correct CRC from it.
//!
//! ## Usage
//!
//! ### Reading from a serial port
//!
//! ```no_run
//! use dsmr_parse::Telegram;
//!
//! // Open serial port (example path)
//! let port = serialport::new("/dev/ttyUSB0", 115_200);
//!
//! // Parse telegram
//! match Telegram::read_from(port.open().unwrap()) {
//!     Ok(Some(telegram)) => println!("Read telegram: {telegram:?}"),
//!     Ok(None) => eprintln!("No complete telegram read"),
//!     Err(e) => eprintln!("Parse error: {e}"),
//! }
//! ```
//!
//! ### Reading from a byte slice
//!
//! ```
//! use dsmr_parse::Telegram;
//!
//! let telegram_data = b"/XMX5LGBBFG1009394887\r\n\r\n1-3:0.2.8(42)\r\n0-0:1.0.0(190101125431W)\r\n1-0:1.8.1(004169.415*kWh)\r\n!1234\r\n";
//!
//! match Telegram::read_from(telegram_data.as_slice()) {
//!     Ok(Some(telegram)) => println!("Read telegram: {telegram:?}"),
//!     Ok(None) => println!("No complete telegram in data"),
//!     Err(e) => eprintln!("Parse error: {e}"),
//! }
//! ```
//!
//! ## Potential Pitfalls
//!
//! - CRC Validation: telegrams with incorrect CRC checksums will be rejected. Ensure data integrity during transmission,
//!   especially when reading from serial ports with poor connections.
//! - Serial Port Configuration: DSMR meters typically use 115200 baud, 8N1: 8 data bits, no parity, 1 stop bit. Incorrect
//!   settings will result in garbled data and parse errors.
//!
//! ## Specification
//!
//! [P1 Companion Standard 5.0.2](https://www.netbeheernederland.nl/publicatie/dsmr-502-p1-companion-standard)

pub use telegram::*;
pub use tst::*;
pub use unit_value::*;

mod line_reader;
mod telegram;
mod tst;
mod unit_value;
