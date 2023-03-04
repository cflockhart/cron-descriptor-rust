#[cfg(test)]
mod tests {
    extern crate strfmt;

    use std::collections::HashMap;

    use strfmt::strfmt;

    use crate::cronparser::cron_expression_descriptor;
    use crate::cronparser::Options;


    #[test]
    fn test_every_second() {
        assert_eq!("Every second", cron_expression_descriptor::get_description_cron(String::from("* * * * * *")));
        assert_eq!("Every second", cron_expression_descriptor::get_description_cron_options(String::from("* * * * * *"), &Options::twenty_four_hour()));
    }

    #[test]
    fn test_every45seconds() {
        assert_eq!("Every 45 seconds", cron_expression_descriptor::get_description_cron("*/45 * * * * *".to_string()));
        assert_eq!("Every 45 seconds", cron_expression_descriptor::get_description_cron_options("*/45 * * * * *".to_string(), &Options::twenty_four_hour()));
    }

    #[test]
    fn test_minute_span() {
        assert_eq!("Every minute between 11:00 AM and 11:10 AM", cron_expression_descriptor::get_description_cron("0-10 11 * * *".to_string()));
        assert_eq!("Every minute between 11:00 and 11:10", cron_expression_descriptor::get_description_cron_options("0-10 11 * * *".to_string(), &Options::twenty_four_hour()));
        assert_eq!("Every minute, at 1:00 AM", cron_expression_descriptor::get_description_cron("* 1 * * *".to_string()));
        assert_eq!("Every minute, at 12:00 AM", cron_expression_descriptor::get_description_cron("* 0 * * *".to_string()));
    }

    #[test]
    fn test_every_minute() {
        assert_eq!("Every minute", cron_expression_descriptor::get_description_cron("* * * * *".to_string()));
        assert_eq!("Every minute", cron_expression_descriptor::get_description_cron("*/1 * * * *".to_string()));
        assert_eq!("Every minute", cron_expression_descriptor::get_description_cron("0 0/1 * * * ?".to_string()));
    }

    #[test]
    fn test_every_hour() {
        assert_eq!("Every hour", cron_expression_descriptor::get_description_cron("0 0 * * * ?".to_string()));
        assert_eq!("Every hour", cron_expression_descriptor::get_description_cron("0 0 0/1 * * ?".to_string()));
        assert_eq!("Every hour", cron_expression_descriptor::get_description_cron("0 * * * *".to_string()));
    }

    #[test]
    fn test_every_xminutes() {
        assert_eq!("Every 5 minutes", cron_expression_descriptor::get_description_cron("*/5 * * * *".to_string()));
        assert_eq!("Every 5 minutes", cron_expression_descriptor::get_description_cron("0 */5 * * * *".to_string()));
        assert_eq!("Every 10 minutes", cron_expression_descriptor::get_description_cron("0 0/10 * * * ?".to_string()));
    }

    #[test]
    fn test_daily_at_time() {
        assert_eq!("At 11:30 AM", cron_expression_descriptor::get_description_cron("30 11 * * *".to_string()));
        assert_eq!("At 11:30", cron_expression_descriptor::get_description_cron_options("30 11 * * *".to_string(), &Options::twenty_four_hour()));
        assert_eq!("At 11:00 AM", cron_expression_descriptor::get_description_cron("0 11 * * *".to_string()));
    }

    #[test]
    fn test_time_of_day_certain_days_of_week() {
        assert_eq!("At 11:00 PM, Monday through Friday", cron_expression_descriptor::get_description_cron("0 23 ? * MON-FRI".to_string()));
        assert_eq!("At 23:00, Monday through Friday", cron_expression_descriptor::get_description_cron_options("0 23 ? * MON-FRI".to_string(), &Options::twenty_four_hour()));
        assert_eq!("At 11:30 AM, Monday through Friday", cron_expression_descriptor::get_description_cron("30 11 * * 1-5".to_string()));
    }

    #[test]
    fn test_one_month_only() {
        assert_eq!("Every minute, only in March", cron_expression_descriptor::get_description_cron("* * * 3 *".to_string()));
    }

    #[test]
    fn test_two_months_only() {
        assert_eq!("Every minute, only in March and June", cron_expression_descriptor::get_description_cron("* * * 3,6 *".to_string()));
    }

    #[test]
    fn test_two_times_each_afternoon() {
        assert_eq!("At 2:30 PM and 4:30 PM", cron_expression_descriptor::get_description_cron("30 14,16 * * *".to_string()));
        assert_eq!("At 14:30 and 16:30", cron_expression_descriptor::get_description_cron_options("30 14,16 * * *".to_string(), &Options::twenty_four_hour()));
    }

    #[test]
    fn test_three_times_daily() {
        assert_eq!("At 6:30 AM, 2:30 PM and 4:30 PM", cron_expression_descriptor::get_description_cron("30 6,14,16 * * *".to_string()));
        assert_eq!("At 06:30, 14:30 and 16:30", cron_expression_descriptor::get_description_cron_options("30 6,14,16 * * *".to_string(), &Options::twenty_four_hour()));
    }

    #[test]
    fn test_once_aweek() {
        assert_eq!("At 9:46 AM, only on Sunday", cron_expression_descriptor::get_description_cron("46 9 * * 0".to_string()));
        assert_eq!("At 9:46 AM, only on Sunday", cron_expression_descriptor::get_description_cron("46 9 * * 7".to_string()));
        assert_eq!("At 9:46 AM, only on Monday", cron_expression_descriptor::get_description_cron("46 9 * * 1".to_string()));
        assert_eq!("At 9:46 AM, only on Saturday", cron_expression_descriptor::get_description_cron("46 9 * * 6".to_string()));
    }

    #[test]
    fn test_once_aweek_non_zero_based() {
        let opts = Options::options();
        let options = Options {
            zero_based_day_of_week: false,
            ..opts
        };
        assert_eq!("At 9:46 AM, only on Sunday", cron_expression_descriptor::get_description_cron_options("46 9 * * 1".to_string(), &options));
        assert_eq!("At 9:46 AM, only on Monday", cron_expression_descriptor::get_description_cron_options("46 9 * * 2".to_string(), &options));
        assert_eq!("At 9:46 AM, only on Saturday", cron_expression_descriptor::get_description_cron_options("46 9 * * 7".to_string(), &options));
    }

    #[test]
    fn test_twice_aweek() {
        assert_eq!("At 9:46 AM, only on Monday and Tuesday", cron_expression_descriptor::get_description_cron("46 9 * * 1,2".to_string()));
        assert_eq!("At 9:46 AM, only on Sunday and Saturday", cron_expression_descriptor::get_description_cron("46 9 * * 0,6".to_string()));
        assert_eq!("At 9:46 AM, only on Saturday and Sunday", cron_expression_descriptor::get_description_cron("46 9 * * 6,7".to_string()));
    }

    #[test]
    fn test_twice_aweek_non_zero_based() {
        let options = Options {
            zero_based_day_of_week: false,
            ..Options::options()
        };
        assert_eq!("At 9:46 AM, only on Sunday and Monday", cron_expression_descriptor::get_description_cron_options("46 9 * * 1,2".to_string(), &options));
        assert_eq!("At 9:46 AM, only on Friday and Saturday", cron_expression_descriptor::get_description_cron_options("46 9 * * 6,7".to_string(), &options));
    }

    #[test]
    fn test_day_of_month() {
        assert_eq!("At 12:23 PM, on day 15 of the month", cron_expression_descriptor::get_description_cron("23 12 15 * *".to_string()));
        assert_eq!("At 12:23, on day 15 of the month", cron_expression_descriptor::get_description_cron_options("23 12 15 * *".to_string(), &Options::twenty_four_hour()));
    }

    #[test]
    fn test_month_name() {
        assert_eq!("At 12:23 PM, only in January", cron_expression_descriptor::get_description_cron("23 12 * JAN *".to_string()));
    }

    #[test]
    fn test_day_of_month_with_question_mark() {
        assert_eq!("At 12:23 PM, only in January", cron_expression_descriptor::get_description_cron("23 12 ? JAN *".to_string()));
    }

    #[test]
    fn test_month_name_range2() {
        assert_eq!("At 12:23 PM, January through February", cron_expression_descriptor::get_description_cron("23 12 * JAN-FEB *".to_string()));
    }

    #[test]
    fn test_month_name_range3() {
        assert_eq!("At 12:23 PM, January through March", cron_expression_descriptor::get_description_cron("23 12 * JAN-MAR *".to_string()));
    }

    #[test]
    fn test_month_name_ranges() {
        assert_eq!("At 3:00 AM, only in January through March and May through June", cron_expression_descriptor::get_description_cron("0 0 3 * 1-3,5-6 *".to_string()));
    }

    #[test]
    fn test_day_of_week_name() {
        assert_eq!("At 12:23 PM, only on Sunday", cron_expression_descriptor::get_description_cron("23 12 * * SUN".to_string()));
    }

    #[test]
    fn test_day_of_week_range() {
        assert_eq!("Every 5 minutes, at 3:00 PM, Monday through Friday", cron_expression_descriptor::get_description_cron("*/5 15 * * MON-FRI".to_string()));
        assert_eq!("Every 5 minutes, at 3:00 PM, Sunday through Saturday", cron_expression_descriptor::get_description_cron("*/5 15 * * 0-6".to_string()));
        assert_eq!("Every 5 minutes, at 3:00 PM, Saturday through Sunday", cron_expression_descriptor::get_description_cron("*/5 15 * * 6-7".to_string()));
    }

    #[test]
    fn test_day_of_week_ranges() {
        assert_eq!("At 3:00 AM, only on Sunday, Tuesday through Thursday and Saturday", cron_expression_descriptor::get_description_cron("0 0 3 * * 0,2-4,6".to_string()));
    }

    #[test]
    fn test_day_of_week_once_in_month() {
        assert_eq!("Every minute, on the third Monday of the month", cron_expression_descriptor::get_description_cron("* * * * MON#3".to_string()));
        assert_eq!("Every minute, on the third Sunday of the month", cron_expression_descriptor::get_description_cron("* * * * 0#3".to_string()));
    }

    #[test]
    fn test_last_day_of_the_week_of_the_month() {
        assert_eq!("Every minute, on the last Thursday of the month", cron_expression_descriptor::get_description_cron("* * * * 4L".to_string()));
        assert_eq!("Every minute, on the last Sunday of the month", cron_expression_descriptor::get_description_cron("* * * * 0L".to_string()));
    }

    #[test]
    fn test_last_day_of_the_month() {
        assert_eq!("Every 5 minutes, on the last day of the month, only in January", cron_expression_descriptor::get_description_cron("*/5 * L JAN *".to_string()));
    }

    #[test]
    fn test_time_of_day_with_seconds() {
        assert_eq!("At 2:02:30 PM", cron_expression_descriptor::get_description_cron("30 02 14 * * *".to_string()));
    }

    #[test]
    fn test_second_internvals() {
        assert_eq!("Seconds 5 through 10 past the minute", cron_expression_descriptor::get_description_cron("5-10 * * * * *".to_string()));
    }

    #[test]
    fn test_second_minutes_hours_intervals() {
        assert_eq!("Seconds 5 through 10 past the minute, minutes 30 through 35 past the hour, between 10:00 AM and 12:00 PM",
                   cron_expression_descriptor::get_description_cron("5-10 30-35 10-12 * * *".to_string()));
    }

    #[test]
    fn test_every5minutes_at30seconds() {
        assert_eq!("At 30 seconds past the minute, every 5 minutes", cron_expression_descriptor::get_description_cron("30 */5 * * * *".to_string()));
    }

    #[test]
    fn test_minutes_past_the_hour_range() {
        assert_eq!("At 30 minutes past the hour, between 10:00 AM and 1:00 PM, only on Wednesday and Friday",
                   cron_expression_descriptor::get_description_cron("0 30 10-13 ? * WED,FRI".to_string()));
    }

    #[test]
    fn test_seconds_past_the_minute_interval() {
        assert_eq!("At 10 seconds past the minute, every 5 minutes", cron_expression_descriptor::get_description_cron("10 0/5 * * * ?".to_string()));
    }

    #[test]
    fn test_between_with_interval() {
        assert_eq!("Every 3 minutes, minutes 02 through 59 past the hour, at 1:00 AM, 9:00 AM and 10:00 PM, between day 11 and 26 of the month, January through June",
                   cron_expression_descriptor::get_description_cron("2-59/3 1,9,22 11-26 1-6 ?".to_string()));
    }

    #[test]
    fn test_recurring_first_of_month() {
        assert_eq!("At 6:00 AM", cron_expression_descriptor::get_description_cron("0 0 6 1/1 * ?".to_string()));
    }

    #[test]
    fn test_minutes_past_the_hour() {
        assert_eq!("At 05 minutes past the hour", cron_expression_descriptor::get_description_cron("0 5 0/1 * * ?".to_string()));
    }

    /**
     * @since https://github.com/RedHogs/cron-parser/issues/2
     */
    #[test]
    fn test_every_past_the_hour() {
        assert_eq!("At 00, 05, 10, 15, 20, 25, 30, 35, 40, 45, 50 and 55 minutes past the hour", cron_expression_descriptor::get_description_cron("0 0,5,10,15,20,25,30,35,40,45,50,55 * ? * *".to_string()));
    }

    /**
     * @since https://github.com/RedHogs/cron-parser/issues/10
     */
    #[test]
    fn test_every_xminute_past_the_hour_with_interval() {
        assert_eq!("Every 2 minutes, minutes 00 through 30 past the hour, at 5:00 PM, Monday through Friday", cron_expression_descriptor::get_description_cron("0 0-30/2 17 ? * MON-FRI".to_string()));
    }

    /**
     * @since https://github.com/RedHogs/cron-parser/issues/13
     */
    #[test]
    fn test_one_year_only_with_seconds() {
        assert_eq!("Every second, only in 2013", cron_expression_descriptor::get_description_cron("* * * * * * 2013".to_string()));
    }

    #[test]
    fn test_one_year_only_without_seconds() {
        assert_eq!("Every minute, only in 2013", cron_expression_descriptor::get_description_cron("* * * * * 2013".to_string()));
    }

    #[test]
    fn test_two_years_only() {
        assert_eq!("Every minute, only in 2013 and 2014", cron_expression_descriptor::get_description_cron("* * * * * 2013,2014".to_string()));
    }

    #[test]
    fn test_year_range2() {
        assert_eq!("At 12:23 PM, January through February, 2013 through 2014", cron_expression_descriptor::get_description_cron("23 12 * JAN-FEB * 2013-2014".to_string()));
    }

    #[test]
    fn test_year_range3() {
        assert_eq!("At 12:23 PM, January through March, 2013 through 2015", cron_expression_descriptor::get_description_cron("23 12 * JAN-MAR * 2013-2015".to_string()));
    }

    #[test]
    fn test_issue26() {
        assert_eq!("At 05 and 10 minutes past the hour", cron_expression_descriptor::get_description_cron("5,10 * * * *".to_string()));
        assert_eq!("At 05 and 10 minutes past the hour, at 12:00 AM", cron_expression_descriptor::get_description_cron("5,10 0 * * *".to_string()));
        assert_eq!("At 05 and 10 minutes past the hour, on day 2 of the month", cron_expression_descriptor::get_description_cron("5,10 * 2 * *".to_string()));
        assert_eq!("Every 10 minutes, on day 2 of the month", cron_expression_descriptor::get_description_cron("5/10 * 2 * *".to_string()));

        assert_eq!("At 5 and 6 seconds past the minute", cron_expression_descriptor::get_description_cron("5,6 0 * * * *".to_string()));
        assert_eq!("At 5 and 6 seconds past the minute, at 1:00 AM", cron_expression_descriptor::get_description_cron("5,6 0 1 * * *".to_string()));
        assert_eq!("At 5 and 6 seconds past the minute, on day 2 of the month", cron_expression_descriptor::get_description_cron("5,6 0 * 2 * *".to_string()));
    }
}
