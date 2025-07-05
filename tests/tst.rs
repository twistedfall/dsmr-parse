use dsmr_parse::Tst;

#[test]
fn test_parse_tst() {
	assert_eq!(
		Tst {
			year: 2018,
			month: 10,
			day: 28,
			hour: 2,
			minute: 30,
			second: 0,
			dst: true,
		},
		Tst::try_from_bytes(b"181028023000S").unwrap()
	);
	assert_eq!(
		Tst {
			year: 2018,
			month: 10,
			day: 28,
			hour: 2,
			minute: 30,
			second: 0,
			dst: false,
		},
		Tst::try_from_bytes(b"181028023000W").unwrap()
	);
}

#[test]
#[cfg(feature = "jiff")]
fn test_to_jiff() {
	let tz = jiff::tz::TimeZone::get("Europe/Amsterdam").expect("Can't load Amsterdam timezone");
	assert_eq!(
		jiff::Timestamp::from_second(1540686600).unwrap().to_zoned(tz.clone()),
		Tst::try_from_bytes(b"181028023000S").unwrap().to_jiff(&tz).unwrap()
	);
	assert_eq!(
		jiff::Timestamp::from_second(1540690200).unwrap().to_zoned(tz.clone()),
		Tst::try_from_bytes(b"181028023000W").unwrap().to_jiff(&tz).unwrap()
	);
}
