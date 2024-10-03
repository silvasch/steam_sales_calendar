use chrono::Utc;

#[derive(Debug, PartialEq)]
pub struct Sale<'a> {
    pub name: &'a str,
    pub start: chrono::DateTime<Utc>,
    pub end: chrono::DateTime<Utc>,
}

impl<'a> Sale<'a> {
    pub fn new(name: &'a str, start: chrono::DateTime<Utc>, end: chrono::DateTime<Utc>) -> Self {
        Self { name, start, end }
    }
}

#[macro_export]
macro_rules! sale {
    ($name:literal, $year_start:literal-$month_start:literal-$day_start:literal, $year_end:literal-$month_end:literal-$day_end:literal) => {
        $crate::Sale::new(
            $name,
            format!(
                "{}-{}-{} 17:00:00 UTC",
                $year_start, $month_start, $day_start
            )
            .parse()
            .unwrap(),
            format!("{}-{}-{} 17:00:00 UTC", $year_end, $month_end, $day_end)
                .parse()
                .unwrap(),
        )
    };
}

#[macro_export]
macro_rules! sales {
    ($($name:literal: $year_start:literal-$month_start:literal-$day_start:literal -> $year_end:literal-$month_end:literal-$day_end:literal),*$(,)?) => {
        [
            $(
                $crate::sale!($name, $year_start-$month_start-$day_start, $year_end-$month_end-$day_end),
            )*
        ]
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_test() {
        let expected = vec![
            Sale::new(
                "Steam Fest",
                "2024-01-01 17:00:00 UTC".parse().unwrap(),
                "2024-01-07 17:00:00 UTC".parse().unwrap(),
            ),
            Sale::new(
                "Steam Fest 2",
                "2024-02-01 17:00:00 UTC".parse().unwrap(),
                "2024-02-07 17:00:00 UTC".parse().unwrap(),
            ),
        ];

        let actual = sales! {
            "Steam Fest": 2024-1-1 -> 2024-1-7,
            "Steam Fest 2": 2024-2-1 -> 2024-2-7,
        };

        assert_eq!(expected, actual);
    }
}
