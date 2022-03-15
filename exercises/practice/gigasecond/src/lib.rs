use time::Duration;
use time::PrimitiveDateTime as DateTime;

const GIGASECOND: i64 = 1_000_000_000;

pub fn after(start: DateTime) -> DateTime {
    start + Duration::seconds(GIGASECOND)
}
