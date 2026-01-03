#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    /// Return the plural noun for this time unit.
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    /// Returns the singular noun for this time unit.
    fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

fn rough_time_to_english(rt: &RoughTime) -> String {
    match rt {
        RoughTime::InThePast(units, count) => {
            format!("{count} {} ago", units.plural())
        }
        RoughTime::JustNow => "just now".to_string(),
        RoughTime::InTheFuture(units, count) => {
            format!("{count} {} ago", units.plural())
        }
    }
}

fn rough_time_to_english_take(rt: RoughTime) -> String {
    match rt {
        RoughTime::InThePast(units, count) => {
            format!("{count} {} ago", units.plural())
        }
        RoughTime::JustNow => "just now".to_string(),
        RoughTime::InTheFuture(units, count) => {
            format!("{count} {} ago", units.plural())
        }
    }
}

fn main() {
    let tmp = RoughTime::InThePast(TimeUnit::Hours, 7);

    let string = rough_time_to_english(&tmp);

    println!("{string}");

    let string = rough_time_to_english_take(tmp);

    println!("{string}");
}
