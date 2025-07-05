use dsmr_parse::{PowerFailureEntry, Telegram, Tst, UnitValue};
use matches::assert_matches;

#[test]
fn test_read_from() {
	let message = include_bytes!("telegram.txt");
	let tgrm = Telegram::read_from(message.as_slice());
	assert_matches!(tgrm, Ok(Some(..)));
	let tgrm_ref = Telegram {
		ident: "XMX5LGBBFG1098765432".to_string(),
		version: Some("42".to_string()),
		electricity_date: Some(Tst {
			year: 2019,
			month: 1,
			day: 1,
			hour: 12,
			minute: 54,
			second: 31,
			dst: false,
		}),
		electricity_equipment_id: Some("E0034567890123456".to_string()),
		electricity_consumed_tariff_1: Some(UnitValue::with_unit(4169.415, "kWh")),
		electricity_consumed_tariff_2: Some(UnitValue::with_unit(4884.452, "kWh")),
		electricity_generated_tariff_1: Some(UnitValue::with_unit(0., "kWh")),
		electricity_generated_tariff_2: Some(UnitValue::with_unit(0., "kWh")),
		current_tariff: Some("\0\u{1}".to_string()),
		power: Some(UnitValue::with_unit(0.741, "kW")),
		return_power: Some(UnitValue::with_unit(0., "kW")),
		power_failure_count: Some(2),
		long_power_failure_count: Some(0),
		power_failure_log: vec![],
		voltage_sag_l1_count: Some(0),
		voltage_sag_l2_count: None,
		voltage_sag_l3_count: None,
		voltage_swell_l1_count: Some(0),
		voltage_swell_l2_count: None,
		voltage_swell_l3_count: None,
		message: Some("".to_string()),
		voltage_l1: None,
		voltage_l2: None,
		voltage_l3: None,
		current_l1: Some(UnitValue::with_unit(4, "A")),
		current_l2: None,
		current_l3: None,
		power_l1: Some(UnitValue::with_unit(0.741, "kW")),
		power_l2: None,
		power_l3: None,
		return_power_l1: Some(UnitValue::with_unit(0.0, "kW")),
		return_power_l2: None,
		return_power_l3: None,
		device_type: Some("003".to_string()),
		gas_equipment_id: Some("G0062749506872590".to_string()),
		gas_date: Some(Tst {
			year: 2019,
			month: 1,
			day: 1,
			hour: 12,
			minute: 0,
			second: 0,
			dst: false,
		}),
		gas_consumed: Some(UnitValue::with_unit(1619.203, "m3")),
	};
	assert_eq!(tgrm.unwrap(), Some(tgrm_ref));
}

#[test]
fn test_read_from2() {
	let message = include_bytes!("telegram2.txt");
	let tgrm = Telegram::read_from(message.as_slice());
	assert_matches!(tgrm, Ok(Some(..)));
	let tgrm_ref = Telegram {
		ident: "ISK5\\2M550T-4567".to_string(),
		version: Some("50".to_string()),
		electricity_date: Some(Tst {
			year: 2023,
			month: 3,
			day: 9,
			hour: 19,
			minute: 10,
			second: 49,
			dst: false,
		}),
		electricity_equipment_id: Some("E0087654210987654".to_string()),
		electricity_consumed_tariff_1: Some(UnitValue::with_unit(2.232, "kWh")),
		electricity_consumed_tariff_2: Some(UnitValue::with_unit(0., "kWh")),
		electricity_generated_tariff_1: Some(UnitValue::with_unit(0., "kWh")),
		electricity_generated_tariff_2: Some(UnitValue::with_unit(0., "kWh")),
		current_tariff: Some("\0\u{1}".to_string()),
		power: Some(UnitValue::with_unit(0.277, "kW")),
		return_power: Some(UnitValue::with_unit(0., "kW")),
		power_failure_count: Some(5),
		long_power_failure_count: Some(3),
		power_failure_log: vec![PowerFailureEntry {
			end_date: Tst {
				year: 2020,
				month: 11,
				day: 6,
				hour: 15,
				minute: 40,
				second: 35,
				dst: false,
			},
			duration: UnitValue::with_unit(518, "s"),
		}],
		voltage_sag_l1_count: Some(0),
		voltage_sag_l2_count: Some(0),
		voltage_sag_l3_count: Some(0),
		voltage_swell_l1_count: Some(1),
		voltage_swell_l2_count: Some(1),
		voltage_swell_l3_count: Some(1),
		message: Some("".to_string()),
		voltage_l1: Some(UnitValue::with_unit(232., "V")),
		voltage_l2: Some(UnitValue::with_unit(233.6, "V")),
		voltage_l3: Some(UnitValue::with_unit(239.6, "V")),
		current_l1: Some(UnitValue::with_unit(0, "A")),
		current_l2: Some(UnitValue::with_unit(0, "A")),
		current_l3: Some(UnitValue::with_unit(1, "A")),
		power_l1: Some(UnitValue::with_unit(0.023, "kW")),
		power_l2: Some(UnitValue::with_unit(0.064, "kW")),
		power_l3: Some(UnitValue::with_unit(0.206, "kW")),
		return_power_l1: Some(UnitValue::with_unit(0.0, "kW")),
		return_power_l2: Some(UnitValue::with_unit(0.0, "kW")),
		return_power_l3: Some(UnitValue::with_unit(0.0, "kW")),
		device_type: Some("003".to_string()),
		gas_equipment_id: Some("".to_string()),
		gas_date: Some(Tst {
			year: 2023,
			month: 3,
			day: 9,
			hour: 19,
			minute: 10,
			second: 1,
			dst: false,
		}),
		gas_consumed: Some(UnitValue::with_unit(0.595, "m3")),
	};
	assert_eq!(tgrm.unwrap(), Some(tgrm_ref));
}

#[test]
fn test_read_from3() {
	let message = include_bytes!("telegram3.txt");
	let tgrm = Telegram::read_from(message.as_slice());
	assert_matches!(tgrm, Ok(Some(..)));
	let tgrm_ref = Telegram {
		ident: "ISk5\\2MT382-1000".to_string(),
		version: Some("50".to_string()),
		electricity_date: Some(Tst {
			year: 2010,
			month: 12,
			day: 9,
			hour: 11,
			minute: 30,
			second: 20,
			dst: false,
		}),
		electricity_equipment_id: Some("K8EG004046395507".to_string()),
		electricity_consumed_tariff_1: Some(UnitValue::with_unit(123456.789, "kWh")),
		electricity_consumed_tariff_2: Some(UnitValue::with_unit(123456.789, "kWh")),
		electricity_generated_tariff_1: Some(UnitValue::with_unit(123456.789, "kWh")),
		electricity_generated_tariff_2: Some(UnitValue::with_unit(123456.789, "kWh")),
		current_tariff: Some("\0\u{2}".to_string()),
		power: Some(UnitValue::with_unit(1.193, "kW")),
		return_power: Some(UnitValue::with_unit(0.0, "kW")),
		power_failure_count: Some(4),
		long_power_failure_count: Some(2),
		power_failure_log: vec![
			PowerFailureEntry {
				end_date: Tst {
					year: 2010,
					month: 12,
					day: 8,
					hour: 15,
					minute: 24,
					second: 15,
					dst: false,
				},
				duration: UnitValue::with_unit(240, "s"),
			},
			PowerFailureEntry {
				end_date: Tst {
					year: 2010,
					month: 12,
					day: 8,
					hour: 15,
					minute: 10,
					second: 4,
					dst: false,
				},
				duration: UnitValue::with_unit(301, "s"),
			},
		],
		voltage_sag_l1_count: Some(2),
		voltage_sag_l2_count: Some(1),
		voltage_sag_l3_count: Some(0),
		voltage_swell_l1_count: Some(0),
		voltage_swell_l2_count: Some(3),
		voltage_swell_l3_count: Some(0),
		message: Some("0123456789:;<=>?0123456789:;<=>?0123456789:;<=>?0123456789:;<=>?0123456789:;<=>?".to_string()),
		voltage_l1: Some(UnitValue::with_unit(220.1, "V")),
		voltage_l2: Some(UnitValue::with_unit(220.2, "V")),
		voltage_l3: Some(UnitValue::with_unit(220.3, "V")),
		current_l1: Some(UnitValue::with_unit(1, "A")),
		current_l2: Some(UnitValue::with_unit(2, "A")),
		current_l3: Some(UnitValue::with_unit(3, "A")),
		power_l1: Some(UnitValue::with_unit(1.111, "kW")),
		power_l2: Some(UnitValue::with_unit(2.222, "kW")),
		power_l3: Some(UnitValue::with_unit(3.333, "kW")),
		return_power_l1: Some(UnitValue::with_unit(4.444, "kW")),
		return_power_l2: Some(UnitValue::with_unit(5.555, "kW")),
		return_power_l3: Some(UnitValue::with_unit(6.666, "kW")),
		device_type: Some("003".to_string()),
		gas_equipment_id: Some("2222ABCD123456789".to_string()),
		gas_date: Some(Tst {
			year: 2010,
			month: 12,
			day: 9,
			hour: 11,
			minute: 25,
			second: 0,
			dst: false,
		}),
		gas_consumed: Some(UnitValue::with_unit(12785.123, "m3")),
	};
	assert_eq!(tgrm.unwrap(), Some(tgrm_ref));
}

#[test]
fn test_empty() {
	let tgrm = Telegram::read_from(b"" as &[u8]);
	assert_matches!(tgrm, Ok(None));

	let tgrm = Telegram::read_from(b"Garbage" as &[u8]);
	assert_matches!(tgrm, Ok(None));
}
