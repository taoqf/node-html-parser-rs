use rust_decimal::prelude::FromPrimitive;
use rust_decimal::prelude::ToPrimitive;

#[allow(dead_code)]
pub(crate) fn decimal2f64(num: &rust_decimal::Decimal) -> f64 {
	return num.to_f64().unwrap_or_default();
}

#[allow(dead_code)]
pub(crate) fn opt_decimal2f64(num: &Option<rust_decimal::Decimal>) -> f64 {
	return match num {
		Some(n) => decimal2f64(n),
		None => 0.0,
	};
}

#[allow(dead_code)]
pub(crate) fn f642decimal(num: f64) -> rust_decimal::Decimal {
	return rust_decimal::Decimal::from_f64(num).unwrap_or_default();
}
