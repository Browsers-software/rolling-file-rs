use std::time::{Duration, SystemTimeError};

pub const UNIX_EPOCH: SystemTime = SystemTime {
    duration_since_epoch: Duration::from_secs(0),
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SystemTime {
    duration_since_epoch: Duration,
}

impl SystemTime {
    pub fn duration_since(&self, earlier: SystemTime) -> Result<Duration, SystemTimeError> {
        let duration = self.duration_since_epoch - earlier.duration_since_epoch;
        return Ok(duration);
    }

    pub fn now() -> SystemTime {
        // no need to make this correct in tests atm
        // (but otherwise could e.g use real SystemTime here and convert to duration)
        let duration_since_epoch = Duration::from_secs(10);
        SystemTime { duration_since_epoch }
    }

    #[allow(unused_variables)]
    pub fn with_ymd_and_hms(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> SystemTime {
        let epoch = epoch_from(year, month, day, hour, min, sec);
        SystemTime {
            duration_since_epoch: epoch,
        }
    }
}

pub fn epoch_from(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> Duration {
    let number_of_days_passed_years: usize = (1970..year).map(|y| number_of_days_in_a_year(y)).sum();

    let number_of_days_passed_this_year_months: usize = (0..month).map(|m| number_of_days_in_a_month(year, m)).sum();

    let days_passed_this_month: u64 = (day as u64 - 1).max(0);

    let number_of_days_passed: u64 =
        number_of_days_passed_years as u64 + number_of_days_passed_this_year_months as u64 + days_passed_this_month;

    let number_of_hours_passed: u64 = number_of_days_passed * 24 + hour as u64;

    let number_of_minutes_passed: u64 = number_of_hours_passed * 60 + min as u64;
    let number_of_seconds_passed: u64 = number_of_minutes_passed * 60 + sec as u64;

    Duration::from_secs(number_of_seconds_passed)
}

fn number_of_days_in_a_year(year: i32) -> usize {
    return if year % 4 == 0 && year % 400 != 0 { 366 } else { 365 };
}

fn number_of_days_in_a_month(year: i32, month: u32) -> usize {
    if month == 0 {
        return 0;
    }

    if month == 1 || month == 3 || month == 5 || month == 7 || month == 8 || month == 10 || month == 12 {
        31
    } else if month == 2 && ((year % 400 == 0) || year % 4 == 0 && year % 100 != 0) {
        29
    } else if month == 2 {
        28
    } else {
        30
    }
}
