use cron_descriptor;
extern crate strfmt;

use cron_descriptor::cronparser::cron_expression_descriptor;
use cron_descriptor::cronparser::Options;
use cron_descriptor::cronparser::cron_expression_descriptor::ParseException;
mod test_utils;
use crate::test_utils::unwrapped_description_options;
use crate::test_utils::unwrapped_description;

#[test]
fn test_parse_exception() {
    match cron_expression_descriptor::get_description_cron("******") {
        Ok(_) => panic!("Got OK, it's not OK"),
        Err(parse_err) => 
         assert_eq!(ParseException { s: "******".to_string(), error_offset: 0 }, parse_err) 
    }

}

#[test]
fn test_parse_exception2() {
    match cron_expression_descriptor::get_description_cron("* * * * * * * *") {
        Ok(_) => panic!("Got OK, it's not OK"),
        Err(parse_err) => 
         assert_eq!(ParseException { s: "* * * * * * * *".to_string(), error_offset: 7 }, parse_err) 
    }

}

#[test]
fn test_every_second() {
    rust_i18n::set_locale("en");
    assert_eq!(
        "Every second",
        cron_expression_descriptor::get_description_cron("* * * * * *").unwrap()
    );
    assert_eq!(
        "Every second",
        unwrapped_description_options(
            "* * * * * *",
            &Options::twenty_four_hour()
        )
    );
}

#[test]
fn test_every45seconds() {
    assert_eq!(
        "Every 45 seconds",
        cron_expression_descriptor::get_description_cron("*/45 * * * * *").unwrap()

    );
    assert_eq!(
        "Every 45 seconds",
        unwrapped_description_options(
            "*/45 * * * * *",
            &Options::twenty_four_hour()
        )
    );
}

#[test]
fn test_minute_span() {
    assert_eq!(
        "Every minute between 11:00 AM and 11:10 AM",
        cron_expression_descriptor::get_description_cron("0-10 11 * * *").unwrap()
    );
    assert_eq!(
        "Every minute between 11:00 and 11:10",
        unwrapped_description_options(
            "0-10 11 * * *",
            &Options::twenty_four_hour()
        )
    );
    assert_eq!(
        "Every minute, at 1:00 AM",
        cron_expression_descriptor::get_description_cron("* 1 * * *").unwrap()
    );
    assert_eq!(
        "Every minute, at 12:00 AM",
        cron_expression_descriptor::get_description_cron("* 0 * * *").unwrap()
    );
}

#[test]
fn test_every_minute() {
    assert_eq!(
        "Every minute",
        cron_expression_descriptor::get_description_cron("* * * * *").unwrap()
    );
    assert_eq!(
        "Every minute",
        cron_expression_descriptor::get_description_cron("*/1 * * * *").unwrap()
    );
    assert_eq!(
        "Every minute",
        cron_expression_descriptor::get_description_cron("0 0/1 * * * ?").unwrap()
    );
}

#[test]
fn test_every_hour() {
    assert_eq!(
        "Every hour",
        cron_expression_descriptor::get_description_cron("0 0 * * * ?").unwrap()
    );
    assert_eq!(
        "Every hour",
        cron_expression_descriptor::get_description_cron("0 0 0/1 * * ?").unwrap()
    );
    assert_eq!(
        "Every hour",
        cron_expression_descriptor::get_description_cron("0 * * * *").unwrap()
    );
}

#[test]
fn test_every_xminutes() {
    assert_eq!(
        "Every 5 minutes",
        unwrapped_description("*/5 * * * *")
    );
    assert_eq!(
        "Every 5 minutes",
        unwrapped_description("0 */5 * * * *")
    );
    assert_eq!(
        "Every 10 minutes",
        unwrapped_description("0 0/10 * * * ?")
    );
}

#[test]
fn test_daily_at_time() {
    assert_eq!(
        "At 11:30 AM",
        unwrapped_description("30 11 * * *")
    );
    assert_eq!(
        "At 11:30",
        unwrapped_description_options(
            "30 11 * * *",
            &Options::twenty_four_hour()
        )
    );
    assert_eq!(
        "At 11:00 AM",
        unwrapped_description("0 11 * * *")
    );
}

#[test]
fn test_time_of_day_certain_days_of_week() {
    assert_eq!(
        "At 11:00 PM, Monday through Friday",
        unwrapped_description("0 23 ? * MON-FRI")
    );
    assert_eq!(
        "At 23:00, Monday through Friday",
        unwrapped_description_options(
            "0 23 ? * MON-FRI",
            &Options::twenty_four_hour()
        )
    );
    assert_eq!(
        "At 11:30 AM, Monday through Friday",
        unwrapped_description("30 11 * * 1-5")
    );
}

#[test]
fn test_one_month_only() {
    assert_eq!(
        "Every minute, only in March",
        unwrapped_description("* * * 3 *")
    );
}

#[test]
fn test_two_months_only() {
    assert_eq!(
        "Every minute, only in March and June",
        unwrapped_description("* * * 3,6 *")
    );
}

#[test]
fn test_two_times_each_afternoon() {
    assert_eq!(
        "At 2:30 PM and 4:30 PM",
        unwrapped_description("30 14,16 * * *")
    );
    assert_eq!(
        "At 14:30 and 16:30",
        unwrapped_description_options(
            "30 14,16 * * *",
            &Options::twenty_four_hour()
        )
    );
}

#[test]
fn test_three_times_daily() {
    assert_eq!(
        "At 6:30 AM, 2:30 PM and 4:30 PM",
        unwrapped_description("30 6,14,16 * * *")
    );
    assert_eq!(
        "At 06:30, 14:30 and 16:30",
        unwrapped_description_options(
            "30 6,14,16 * * *",
            &Options::twenty_four_hour()
        )
    );
}

#[test]
fn test_once_aweek() {
    assert_eq!(
        "At 9:46 AM, only on Sunday",
        unwrapped_description("46 9 * * 0")
    );
    assert_eq!(
        "At 9:46 AM, only on Sunday",
        unwrapped_description("46 9 * * 7")
    );
    assert_eq!(
        "At 9:46 AM, only on Monday",
        unwrapped_description("46 9 * * 1")
    );
    assert_eq!(
        "At 9:46 AM, only on Saturday",
        unwrapped_description("46 9 * * 6")
    );
}

#[test]
fn test_once_aweek_non_zero_based() {
    let options = Options {
        zero_based_day_of_week: false,
        ..Options::options()
    };
    assert_eq!(
        "At 9:46 AM, only on Sunday",
        unwrapped_description_options(
            "46 9 * * 1",
            &options
        )
    );
    assert_eq!(
        "At 9:46 AM, only on Monday",
        unwrapped_description_options(
            "46 9 * * 2",
            &options
        )
    );
    assert_eq!(
        "At 9:46 AM, only on Saturday",
        unwrapped_description_options(
            "46 9 * * 7",
            &options
        )
    );
}

#[test]
fn test_twice_aweek() {
    assert_eq!(
        "At 9:46 AM, only on Monday and Tuesday",
        unwrapped_description("46 9 * * 1,2")
    );
    assert_eq!(
        "At 9:46 AM, only on Sunday and Saturday",
        unwrapped_description("46 9 * * 0,6")
    );
    assert_eq!(
        "At 9:46 AM, only on Saturday and Sunday",
        unwrapped_description("46 9 * * 6,7")
    );
}

#[test]
fn test_twice_aweek_non_zero_based() {
    let options = Options {
        zero_based_day_of_week: false,
        ..Options::options()
    };
    assert_eq!(
        "At 9:46 AM, only on Sunday and Monday",
        unwrapped_description_options(
            "46 9 * * 1,2",
            &options
        )
    );
    assert_eq!(
        "At 9:46 AM, only on Friday and Saturday",
        unwrapped_description_options(
            "46 9 * * 6,7",
            &options
        )
    );
}

#[test]
fn test_day_of_month() {
    assert_eq!(
        "At 12:23 PM, on day 15 of the month",
        unwrapped_description("23 12 15 * *")
    );
    assert_eq!(
        "At 12:23, on day 15 of the month",
        unwrapped_description_options(
            "23 12 15 * *",
            &Options::twenty_four_hour()
        )
    );
}

#[test]
fn test_month_name() {
    assert_eq!(
        "At 12:23 PM, only in January",
        unwrapped_description("23 12 * JAN *")
    );
}

#[test]
fn test_day_of_month_with_question_mark() {
    assert_eq!(
        "At 12:23 PM, only in January",
        unwrapped_description("23 12 ? JAN *")
    );
}

#[test]
fn test_month_name_range2() {
    assert_eq!(
        "At 12:23 PM, January through February",
        unwrapped_description("23 12 * JAN-FEB *")
    );
}

#[test]
fn test_month_name_range3() {
    assert_eq!(
        "At 12:23 PM, January through March",
        unwrapped_description("23 12 * JAN-MAR *")
    );
}

#[test]
fn test_month_name_ranges() {
    assert_eq!(
        "At 3:00 AM, only in January through March and May through June",
        unwrapped_description("0 0 3 * 1-3,5-6 *")
    );
}

#[test]
fn test_day_of_week_name() {
    assert_eq!(
        "At 12:23 PM, only on Sunday",
        unwrapped_description("23 12 * * SUN")
    );
}

#[test]
fn test_day_of_week_range() {
    assert_eq!(
        "Every 5 minutes, at 3:00 PM, Monday through Friday",
        unwrapped_description("*/5 15 * * MON-FRI")
    );
    assert_eq!(
        "Every 5 minutes, at 3:00 PM, Sunday through Saturday",
        unwrapped_description("*/5 15 * * 0-6")
    );
    assert_eq!(
        "Every 5 minutes, at 3:00 PM, Saturday through Sunday",
        unwrapped_description("*/5 15 * * 6-7")
    );
}

#[test]
fn test_day_of_week_ranges() {
    assert_eq!(
        "At 3:00 AM, only on Sunday, Tuesday through Thursday and Saturday",
        unwrapped_description("0 0 3 * * 0,2-4,6")
    );
}

#[test]
fn test_day_of_week_once_in_month() {
    assert_eq!(
        "Every minute, on the third Monday of the month",
        unwrapped_description("* * * * MON#3")
    );
    assert_eq!(
        "Every minute, on the third Sunday of the month",
        unwrapped_description("* * * * 0#3")
    );
}

#[test]
fn test_last_day_of_the_week_of_the_month() {
    assert_eq!(
        "Every minute, on the last Thursday of the month",
        unwrapped_description("* * * * 4L")
    );
    assert_eq!(
        "Every minute, on the last Sunday of the month",
        unwrapped_description("* * * * 0L")
    );
}

#[test]
fn test_last_day_of_the_month() {
    assert_eq!(
        "Every 5 minutes, on the last day of the month, only in January",
        unwrapped_description("*/5 * L JAN *")
    );
}

#[test]
fn test_time_of_day_with_seconds() {
    assert_eq!(
        "At 2:02:30 PM",
        unwrapped_description("30 02 14 * * *")
    );
}

#[test]
fn test_second_internvals() {
    assert_eq!(
        "Seconds 5 through 10 past the minute",
        unwrapped_description("5-10 * * * * *")
    );
}

#[test]
fn test_second_minutes_hours_intervals() {
    assert_eq!("Seconds 5 through 10 past the minute, minutes 30 through 35 past the hour, between 10:00 AM and 12:00 PM",
                   unwrapped_description("5-10 30-35 10-12 * * *"));
}

#[test]
fn test_every5minutes_at30seconds() {
    assert_eq!(
        "At 30 seconds past the minute, every 5 minutes",
        unwrapped_description("30 */5 * * * *")
    );
}

#[test]
fn test_minutes_past_the_hour_range() {
    assert_eq!(
        "At 30 minutes past the hour, between 10:00 AM and 1:00 PM, only on Wednesday and Friday",
        unwrapped_description("0 30 10-13 ? * WED,FRI")
    );
}

#[test]
fn test_seconds_past_the_minute_interval() {
    assert_eq!(
        "At 10 seconds past the minute, every 5 minutes",
        unwrapped_description("10 0/5 * * * ?")
    );
}

#[test]
fn test_between_with_interval() {
    assert_eq!("Every 3 minutes, minutes 02 through 59 past the hour, at 1:00 AM, 9:00 AM and 10:00 PM, between day 11 and 26 of the month, January through June",
                   unwrapped_description("2-59/3 1,9,22 11-26 1-6 ?"));
}

#[test]
fn test_recurring_first_of_month() {
    assert_eq!(
        "At 6:00 AM",
        unwrapped_description("0 0 6 1/1 * ?")
    );
}

#[test]
fn test_minutes_past_the_hour() {
    assert_eq!(
        "At 05 minutes past the hour",
        unwrapped_description("0 5 0/1 * * ?")
    );
}

/**
 * @since https://github.com/RedHogs/cron-parser/issues/2
 */
#[test]
fn test_every_past_the_hour() {
    assert_eq!(
        "At 00, 05, 10, 15, 20, 25, 30, 35, 40, 45, 50 and 55 minutes past the hour",
        unwrapped_description(
            "0 0,5,10,15,20,25,30,35,40,45,50,55 * ? * *"
        )
    );
}

/**
 * @since https://github.com/RedHogs/cron-parser/issues/10
 */
#[test]
fn test_every_xminute_past_the_hour_with_interval() {
    assert_eq!(
        "Every 2 minutes, minutes 00 through 30 past the hour, at 5:00 PM, Monday through Friday",
        unwrapped_description("0 0-30/2 17 ? * MON-FRI")
    );
}

/**
 * @since https://github.com/RedHogs/cron-parser/issues/13
 */
#[test]
fn test_one_year_only_with_seconds() {
    assert_eq!(
        "Every second, only in 2013",
        unwrapped_description("* * * * * * 2013")
    );
}

#[test]
fn test_one_year_only_without_seconds() {
    assert_eq!(
        "Every minute, only in 2013",
        unwrapped_description("* * * * * 2013")
    );
}

#[test]
fn test_two_years_only() {
    assert_eq!(
        "Every minute, only in 2013 and 2014",
        unwrapped_description("* * * * * 2013,2014")
    );
}

#[test]
fn test_year_range2() {
    assert_eq!(
        "At 12:23 PM, January through February, 2013 through 2014",
        unwrapped_description("23 12 * JAN-FEB * 2013-2014")
    );
}

#[test]
fn test_year_range3() {
    assert_eq!(
        "At 12:23 PM, January through March, 2013 through 2015",
        unwrapped_description("23 12 * JAN-MAR * 2013-2015")
    );
}

#[test]
fn test_issue26() {
    assert_eq!(
        "At 05 and 10 minutes past the hour",
        unwrapped_description("5,10 * * * *")
    );
    assert_eq!(
        "At 05 and 10 minutes past the hour, at 12:00 AM",
        unwrapped_description("5,10 0 * * *")
    );
    assert_eq!(
        "At 05 and 10 minutes past the hour, on day 2 of the month",
        unwrapped_description("5,10 * 2 * *")
    );
    assert_eq!(
        "Every 10 minutes, on day 2 of the month",
        unwrapped_description("5/10 * 2 * *")
    );

    assert_eq!(
        "At 5 and 6 seconds past the minute",
        unwrapped_description("5,6 0 * * * *")
    );
    assert_eq!(
        "At 5 and 6 seconds past the minute, at 1:00 AM",
        unwrapped_description("5,6 0 1 * * *")
    );
    assert_eq!(
        "At 5 and 6 seconds past the minute, on day 2 of the month",
        unwrapped_description("5,6 0 * 2 * *")
    );
}

// #[macro_use]
// extern crate rust_i18n;

// use rust_i18n::set_locale;
// i18n!("locales");
// rust_i18n::set_locale("es");
