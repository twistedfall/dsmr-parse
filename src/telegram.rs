use core::str;
use std::io::Read;
use std::str::FromStr;

pub use error::Error;
use log::{trace, warn};

use crate::Tst;
use crate::line_reader::LineReader;
use crate::unit_value::UnitValue;

mod error;

/// Valid telegram
#[derive(Debug, PartialOrd, PartialEq, Clone, Default)]
pub struct Telegram {
	pub ident: String,
	pub version: Option<String>,
	pub electricity_date: Option<Tst>,
	pub electricity_equipment_id: Option<String>,
	pub electricity_consumed_tariff_1: Option<UnitValue<f64>>,
	pub electricity_consumed_tariff_2: Option<UnitValue<f64>>,
	pub electricity_generated_tariff_1: Option<UnitValue<f64>>,
	pub electricity_generated_tariff_2: Option<UnitValue<f64>>,
	pub current_tariff: Option<String>,
	pub power: Option<UnitValue<f64>>,
	pub return_power: Option<UnitValue<f64>>,
	pub power_failure_count: Option<u32>,
	pub long_power_failure_count: Option<u32>,
	pub power_failure_log: Vec<PowerFailureEntry>,
	pub voltage_sag_l1_count: Option<u32>,
	pub voltage_sag_l2_count: Option<u32>,
	pub voltage_sag_l3_count: Option<u32>,
	pub voltage_swell_l1_count: Option<u32>,
	pub voltage_swell_l2_count: Option<u32>,
	pub voltage_swell_l3_count: Option<u32>,
	pub message: Option<String>,
	pub voltage_l1: Option<UnitValue<f64>>,
	pub voltage_l2: Option<UnitValue<f64>>,
	pub voltage_l3: Option<UnitValue<f64>>,
	pub current_l1: Option<UnitValue<u16>>,
	pub current_l2: Option<UnitValue<u16>>,
	pub current_l3: Option<UnitValue<u16>>,
	pub power_l1: Option<UnitValue<f64>>,
	pub power_l2: Option<UnitValue<f64>>,
	pub power_l3: Option<UnitValue<f64>>,
	pub return_power_l1: Option<UnitValue<f64>>,
	pub return_power_l2: Option<UnitValue<f64>>,
	pub return_power_l3: Option<UnitValue<f64>>,
	pub device_type: Option<String>,
	pub gas_equipment_id: Option<String>,
	pub gas_date: Option<Tst>,
	pub gas_consumed: Option<UnitValue<f64>>,
}

impl Telegram {
	/// Try to read a single telegram from a [Read] source
	///
	/// See [crate-level](crate) documentation for more details.
	pub fn read_from(src: impl Read) -> Result<Option<Self>, Error> {
		enum ParserState {
			WaitingForHeader,
			ReadingHeader,
			ReadingMessage,
		}

		const CRLF: &[u8] = b"\r\n";
		let mut parser_state = ParserState::WaitingForHeader;
		let mut out = Self::default();
		let mut crc = crc16::State::<crc16::ARC>::new();
		let mut expected_crc = None;
		#[expect(clippy::unbuffered_bytes)]
		let lines = LineReader::new(src.bytes());
		for line in lines {
			let mut line = line?;
			trace!("Got line: {}", String::from_utf8_lossy(&line));
			match parser_state {
				ParserState::WaitingForHeader => {
					if line.starts_with(b"/") {
						crc.update(&line);
						crc.update(CRLF);
						line.remove(0);
						out.ident = String::from_utf8(line)?;
						parser_state = ParserState::ReadingHeader;
					}
				}
				ParserState::ReadingHeader => {
					if line.is_empty() {
						crc.update(CRLF);
						parser_state = ParserState::ReadingMessage;
					} else {
						parser_state = ParserState::WaitingForHeader;
					}
				}
				ParserState::ReadingMessage => {
					const CRC_PREFIX: &[u8] = b"!";
					if let Some(crc_str) = line.strip_prefix(CRC_PREFIX) {
						crc.update(CRC_PREFIX);
						expected_crc = Some(u16::from_str_radix(str::from_utf8(crc_str)?, 16)?);
						break;
					} else {
						crc.update(&line);
						crc.update(CRLF);
						if let Some(line) = ParsedLine::parse(&line) {
							match line.obis.obis {
								b"0.2.8" => out.version = Some(line.value_str().to_string()),
								b"1.0.0" => out.electricity_date = Tst::try_from_bytes(line.value),
								b"96.1.1" => out.electricity_equipment_id = parse_octet_string(line.value),
								b"1.8.1" => out.electricity_consumed_tariff_1 = Some(line.value_str().parse()?),
								b"1.8.2" => out.electricity_consumed_tariff_2 = Some(line.value_str().parse()?),
								b"2.8.1" => out.electricity_generated_tariff_1 = Some(line.value_str().parse()?),
								b"2.8.2" => out.electricity_generated_tariff_2 = Some(line.value_str().parse()?),
								b"96.14.0" => out.current_tariff = parse_octet_string(line.value),
								b"1.7.0" => out.power = Some(line.value_str().parse()?),
								b"2.7.0" => out.return_power = Some(line.value_str().parse()?),
								b"96.7.21" => out.power_failure_count = Some(line.value_str().parse()?),
								b"96.7.9" => out.long_power_failure_count = Some(line.value_str().parse()?),
								b"99.97.0" => {
									out.power_failure_log = parse_buffer(line.value_str())
										.into_iter()
										.map(|(end_date, duration)| PowerFailureEntry { end_date, duration })
										.collect()
								}
								b"32.32.0" => out.voltage_sag_l1_count = Some(line.value_str().parse()?),
								b"52.32.0" => out.voltage_sag_l2_count = Some(line.value_str().parse()?),
								b"72.32.0" => out.voltage_sag_l3_count = Some(line.value_str().parse()?),
								b"32.36.0" => out.voltage_swell_l1_count = Some(line.value_str().parse()?),
								b"52.36.0" => out.voltage_swell_l2_count = Some(line.value_str().parse()?),
								b"72.36.0" => out.voltage_swell_l3_count = Some(line.value_str().parse()?),
								b"96.13.0" => out.message = parse_octet_string(line.value),
								b"32.7.0" => out.voltage_l1 = Some(line.value_str().parse()?),
								b"52.7.0" => out.voltage_l2 = Some(line.value_str().parse()?),
								b"72.7.0" => out.voltage_l3 = Some(line.value_str().parse()?),
								b"31.7.0" => out.current_l1 = Some(line.value_str().parse()?),
								b"51.7.0" => out.current_l2 = Some(line.value_str().parse()?),
								b"71.7.0" => out.current_l3 = Some(line.value_str().parse()?),
								b"21.7.0" => out.power_l1 = Some(line.value_str().parse()?),
								b"41.7.0" => out.power_l2 = Some(line.value_str().parse()?),
								b"61.7.0" => out.power_l3 = Some(line.value_str().parse()?),
								b"22.7.0" => out.return_power_l1 = Some(line.value_str().parse()?),
								b"42.7.0" => out.return_power_l2 = Some(line.value_str().parse()?),
								b"62.7.0" => out.return_power_l3 = Some(line.value_str().parse()?),
								b"24.1.0" => out.device_type = Some(line.value_str().to_string()),
								b"96.1.0" => out.gas_equipment_id = parse_octet_string(line.value),
								b"24.2.1" => {
									let (gas_date, gas_consumed) = parse_mbus_value(line.value_str());
									out.gas_date = Tst::try_from_bytes(gas_date.as_bytes());
									if let Some(gas_consumed) = gas_consumed {
										out.gas_consumed = Some(gas_consumed.parse()?);
									}
								}
								_ => warn!(
									"Unknown OBIS: {}-{}-{} with value: {}",
									line.obis.medium,
									line.obis.channel,
									String::from_utf8_lossy(line.obis.obis),
									String::from_utf8_lossy(line.value),
								),
							}
						}
					}
				}
			}
		}

		if let Some(expected_crc) = expected_crc {
			let actual_crc = crc.get();
			if actual_crc == expected_crc {
				Ok(Some(out))
			} else {
				Err(Error::CrcMismatch(actual_crc, expected_crc))
			}
		} else {
			Ok(None)
		}
	}
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct PowerFailureEntry {
	pub end_date: Tst,
	pub duration: UnitValue<u64>,
}

fn parse_mbus_value(value: &str) -> (&str, Option<&str>) {
	value
		.split_once(")(")
		.map_or((value, None), |(val1, val2)| (val1, Some(val2)))
}

fn parse_buffer(value: &str) -> Vec<(Tst, UnitValue<u64>)> {
	let mut out = vec![];
	let mut parts = value.split(")(");
	'extract: {
		let Some(count) = parts.next() else {
			break 'extract;
		};
		let Ok(count) = u8::from_str(count) else {
			break 'extract;
		};
		out.reserve(usize::from(count));
		let Some(obis) = parts.next() else {
			break 'extract;
		};
		let Some((obis, _)) = ParsedObis::parse(obis.as_bytes()) else {
			break 'extract;
		};
		if obis.obis != b"96.7.19" {
			break 'extract;
		}
		while let Some((date, value)) = parts.next().zip(parts.next()) {
			let Some(date) = Tst::try_from_bytes(date.as_bytes()) else {
				continue;
			};
			let Ok(value) = UnitValue::from_str(value) else {
				continue;
			};
			out.push((date, value));
		}
	}
	out
}

fn parse_octet_string(value: &[u8]) -> Option<String> {
	let mut out = String::with_capacity(value.len() / 2);
	for bytes in value.chunks(2) {
		out.push(char::from(u8::from_str_radix(str::from_utf8(bytes).ok()?, 16).ok()?));
	}
	Some(out)
}

struct ParsedLine<'l> {
	pub obis: ParsedObis<'l>,
	pub value: &'l [u8],
}

impl ParsedLine<'_> {
	fn parse(line: &[u8]) -> Option<ParsedLine> {
		#[expect(clippy::enum_variant_names)]
		enum State {
			WaitingForValue,
			ReadingValue,
			WaitingForAnotherValue,
		}

		let (obis, line) = ParsedObis::parse(line)?;
		let mut out = ParsedLine { obis, value: &[] };
		let mut state = State::WaitingForValue;
		let mut start_offset = 0;
		for (offset, byte) in line.iter().enumerate() {
			let c = char::from(*byte);
			state = match state {
				State::WaitingForValue => match c {
					'(' => {
						start_offset = offset + 1;
						State::ReadingValue
					}
					_ => return None,
				},
				State::ReadingValue => match c {
					')' => {
						out.value = line.get(start_offset..offset)?;
						State::WaitingForAnotherValue
					}
					_ => State::ReadingValue,
				},
				State::WaitingForAnotherValue => match c {
					'(' => State::ReadingValue,
					_ => return None,
				},
			};
		}
		if matches!(state, State::WaitingForAnotherValue) {
			Some(out)
		} else {
			None
		}
	}

	pub fn value_str(&self) -> &str {
		str::from_utf8(self.value).unwrap_or("")
	}
}

#[derive(Debug)]
struct ParsedObis<'l> {
	/// Group A: Specifies the medium of the object (0= Abstract Objects, 1=Electricity, 7=gas, Etc.)
	pub medium: u8,
	/// Group B: Specifies the channel (useful, for example, when a data concentrator is connected to several meters).
	pub channel: u8,
	/// Group C: Specifies the Physical Value (Current, Voltage, Energy, etc.).
	/// Group D: Identifies types, or the result of the processing of physical quantities identified by values in value groups A and C, according to various specific algorithms.
	/// Group E: Identifies further processing or classification of quantities identified by values in value groups A to D.
	pub obis: &'l [u8],
}

impl ParsedObis<'_> {
	fn parse(line: &[u8]) -> Option<(ParsedObis, &[u8])> {
		enum State {
			WaitingForObisMedium,
			ReadingObisMedium,
			WaitingForObisChannel,
			ReadingObisChannel,
			WaitingForObis,
			ReadingObis,
			Done,
		}

		let mut out = ParsedObis {
			medium: 0,
			channel: 0,
			obis: &[],
		};
		let mut state = State::WaitingForObisMedium;
		let mut start_offset = 0;
		// todo: more careful parsing of obis sub-values
		for (offset, byte) in line.iter().enumerate() {
			let c = char::from(*byte);
			state = match state {
				State::WaitingForObisMedium => match c {
					c if c.is_ascii_digit() => {
						start_offset = offset;
						State::ReadingObisMedium
					}
					_ => return None,
				},
				State::ReadingObisMedium => match c {
					c if c.is_ascii_digit() => State::ReadingObisMedium,
					'-' => {
						out.medium = str::from_utf8(line.get(start_offset..offset)?).ok()?.parse().ok()?;
						State::WaitingForObisChannel
					}
					_ => return None,
				},
				State::WaitingForObisChannel => match c {
					c if c.is_ascii_digit() => {
						start_offset = offset;
						State::ReadingObisChannel
					}
					_ => return None,
				},
				State::ReadingObisChannel => match c {
					c if c.is_ascii_digit() => State::ReadingObisChannel,
					':' => {
						out.channel = str::from_utf8(line.get(start_offset..offset)?).ok()?.parse().ok()?;
						State::WaitingForObis
					}
					_ => return None,
				},
				State::WaitingForObis => match c {
					c if c.is_ascii_digit() => {
						start_offset = offset;
						State::ReadingObis
					}
					_ => return None,
				},
				State::ReadingObis => match c {
					c if c.is_ascii_digit() || c == '.' => State::ReadingObis,
					'(' => {
						out.obis = line.get(start_offset..offset)?;
						start_offset = offset;
						State::Done
					}
					_ => return None,
				},
				State::Done => break,
			};
		}
		match state {
			State::ReadingObis => {
				out.obis = line.get(start_offset..)?;
				Some((out, &[]))
			}
			State::Done => Some((out, line.get(start_offset..)?)),
			_ => None,
		}
	}
}
