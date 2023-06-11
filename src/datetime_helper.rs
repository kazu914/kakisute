use chrono::{DateTime, Local, LocalResult, NaiveDateTime, TimeZone};

const DATE_FORMAT: &str = "%Y_%m_%d_%H_%M_%S";
const DATE_FORMAT_LENGTH: usize = 19;

pub fn datetime_to_string(datetime: DateTime<Local>) -> String {
    datetime.format(DATE_FORMAT).to_string()
}

pub fn string_to_datetime(datetime: &str) -> LocalResult<DateTime<Local>> {
    if datetime.chars().count() < DATE_FORMAT_LENGTH {
        return LocalResult::None;
    };

    let prefix_date = &datetime[0..DATE_FORMAT_LENGTH];

    let created_at = NaiveDateTime::parse_from_str(prefix_date, DATE_FORMAT);
    if let Ok(created_at) = created_at {
        Local.from_local_datetime(&created_at)
    } else {
        LocalResult::None
    }
}

#[cfg(test)]
extern crate speculate;
#[cfg(test)]
use speculate::speculate;

#[cfg(test)]
speculate! {
    use chrono::TimeZone;
    describe "datetime_to_string" {
        before {
            let date = Local.ymd(2022,1,10).and_hms(16,30,15);;
        }

        it "encode datetime to string" {
            let actual = datetime_to_string(date);
            assert_eq!(actual,"2022_01_10_16_30_15")
        }
    }

    describe "string_to_datetime" {
        it "return local date when format is correct" {
            let given = "2022_01_10_16_30_15";
            let expected = Local.ymd(2022,1,10).and_hms(16,30,15);;

            let actual = string_to_datetime(given);

            assert_eq!(actual.single(),Some(expected));
        }

        it "return none when file_name is shorter than fomrat" {
            let given = "2022_01_10_16_30";
            let actual = string_to_datetime(given);

            assert_eq!(actual.single(),None);
        }

        it "return local date when date time is invalid" {
            let given = "2022_13_10_16_30_15";
            let actual = string_to_datetime(given);

            assert_eq!(actual.single(),None);
        }
    }
}
