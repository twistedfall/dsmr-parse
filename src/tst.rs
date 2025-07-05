use core::str;

/// A point in time as reported by the meter
///
/// `year` is normalized from 2 digits by mapping it to 1969..=2068 range
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tst {
	/// Year, normalized from 2 digits by mapping it to 1969..=2068 range
	pub year: u16,
	pub month: u8,
	pub day: u8,
	pub hour: u8,
	pub minute: u8,
	pub second: u8,
	/// True if Daylight Saving Time is active
	pub dst: bool,
}

impl Tst {
	pub fn try_from_bytes(bytes: &[u8]) -> Option<Self> {
		let mut parts = bytes.chunks(2);
		let year = normalize_two_digit_year(str::from_utf8(parts.next()?).ok()?.parse().ok()?);
		let month = str::from_utf8(parts.next()?).ok()?.parse().ok()?;
		let day = str::from_utf8(parts.next()?).ok()?.parse().ok()?;
		let hour = str::from_utf8(parts.next()?).ok()?.parse().ok()?;
		let minute = str::from_utf8(parts.next()?).ok()?.parse().ok()?;
		let second = str::from_utf8(parts.next()?).ok()?.parse().ok()?;
		let dst = parts.next()? == b"S";
		Some(Self {
			year,
			month,
			day,
			hour,
			minute,
			second,
			dst,
		})
	}

	/// Convert current [Tst] to [jiff::Zoned] in the indicated timezone
	#[cfg(feature = "jiff")]
	pub fn to_jiff(&self, timezone: &jiff::tz::TimeZone) -> Option<jiff::Zoned> {
		jiff::civil::DateTime::new(
			i16::try_from(self.year).unwrap_or(i16::MAX),
			i8::try_from(self.month).unwrap_or(i8::MAX),
			i8::try_from(self.day).unwrap_or(i8::MAX),
			i8::try_from(self.hour).unwrap_or(i8::MAX),
			i8::try_from(self.minute).unwrap_or(i8::MAX),
			i8::try_from(self.second).unwrap_or(i8::MAX),
			0,
		)
		.map(|d| timezone.to_ambiguous_zoned(d))
		.ok()
		.and_then(|d| {
			let strategy = if self.dst {
				jiff::tz::Disambiguation::Earlier
			} else {
				jiff::tz::Disambiguation::Later
			};
			d.disambiguate(strategy).ok()
		})
	}

	/// Convert current [Tst] to [chrono::DateTime] in the indicated timezone
	#[cfg(feature = "chrono")]
	pub fn to_chrono<Tz: chrono::TimeZone>(&self, timezone: &Tz) -> Option<chrono::DateTime<Tz>> {
		let d = timezone.from_local_datetime(&chrono::NaiveDateTime::new(
			chrono::NaiveDate::from_ymd_opt(i32::from(self.year), u32::from(self.month), u32::from(self.day))?,
			chrono::NaiveTime::from_hms_opt(u32::from(self.hour), u32::from(self.minute), u32::from(self.second))?,
		));
		match d {
			chrono::LocalResult::None => None,
			chrono::LocalResult::Single(d) => Some(d),
			chrono::LocalResult::Ambiguous(min, max) => Some(if self.dst {
				min
			} else {
				max
			}),
		}
	}
}

fn normalize_two_digit_year(year: u16) -> u16 {
	if (69..=99).contains(&year) {
		year + 1900
	} else if (0..=68).contains(&year) {
		year + 2000
	} else {
		year
	}
}
