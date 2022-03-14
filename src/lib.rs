#[macro_use]
extern crate rust_i18n;

i18n!("locales");

mod date_time_utils {
    use crate::cronparser::Options;

    pub fn format_time(hours_expression: &str, minutes_expression: &str, opts: &Options) -> String {
        format_time_secs(hours_expression, minutes_expression, "", opts)
    }

    pub fn format_time_secs(hours_expression: &str,
                       minutes_expression: &str,
                       seconds_expression: &str,
                       opts: &Options) -> String {
        let hour: u8 = hours_expression.parse().unwrap();
        let minutes: u8 = minutes_expression.parse().unwrap();

        if opts.twenty_four_hour_time {
            if seconds_expression.is_empty() {
               todo!()
            } else {
               todo!()
            }
        }
        todo!()
    }

    pub fn format_minutes(minutes_expression: &str) -> String {
        todo!()
    }
}

mod cronparser {
    enum CasingTypeEnum {
        Title,
        Sentence,
        LowerCase,
    }

    enum DescriptionTypeEnum {
        FULL,
        TIMEOFDAY,
        SECONDS,
        MINUTES,
        HOURS,
        DAYOFWEEK,
        MONTH,
        DAYOFMONTH,
        YEAR,
    }

    pub struct Options {
        throw_exception_on_parse_error: bool,
        casing_type: CasingTypeEnum,
        verbose: bool,
        pub zero_based_day_of_week: bool,
        pub twenty_four_hour_time: bool,
        need_space_between_words: bool,
    }

    impl Options {
        pub fn options() -> Options {
            return Options {
                throw_exception_on_parse_error: true,
                casing_type: CasingTypeEnum::Sentence,
                verbose: false,
                zero_based_day_of_week: true,
                twenty_four_hour_time: false,
                need_space_between_words: true,
            };
        }

        pub fn twenty_four_hour() -> Options {
            let opts = Options::options();
            let opts2 = Options {
                twenty_four_hour_time: true,
                ..opts
            };
            return opts2;
        }
    }


    pub mod cron_expression_descriptor {
        use crate::cronparser;
        use crate::cronparser::DescriptionTypeEnum;
        use crate::cronparser::DescriptionTypeEnum::FULL;
        use regex::Regex;

        const SPECIAL_CHARACTERS: [char; 4] = ['/', '-', ',', '*'];

        pub struct ParseException {
            s: &'static str,
            error_offset: u8,
        }

        mod expression_parser {
            use lazy_static::lazy_static;
            use regex::Regex;
            use crate::cronparser::cron_expression_descriptor::ParseException;
            use crate::cronparser::Options;

            pub fn parse(expression: &String, options: &Options) -> Result<Vec<&str>, ParseException> {
                let mut parsed: Vec<&str> = vec!["", "", "", "", "", "", ""];
                if expression.trim().is_empty() {
                    let err_msg = t!("expression_empty_exception");
                    Err(ParseException {
                        s: err_msg,
                        error_offset: 0,
                    })
                } else {
                    let expression_parts = expression.trim().split(' ').collect();
                    if expression_parts.len() < 5 {
                        return Err(ParseException {
                            s: expression,
                            error_offset: 0,
                        })
                    } else if expression_parts.len() == 5 {
                        parsed[0] = "";
                        parsed[1..5] = expression_parts[0..4];
                    } else if expression_parts.len() == 6 {
                        lazy_static! {
                            static ref year_re: Regex = Regex::new(r"\d{4}$").unwrap();
                        }
                        if year_re.is_match(expression_parts[5]) {
                            parsed[1..6] = expression_parts[0..5];
                        } else {
                            parsed[0..5] = expression_parts[0..5];
                        }
                    } else if expression_parts.len() == 7 {
                        parsed[0..6] = expression_parts[0..6];
                    } else {
                        return Err(ParseException {
                            s: expression,
                            error_offset: 7,
                        })
                    }

                    normalise_expression(&mut parsed, options);
                    Ok(parsed);
                }
            }

            fn normalise_expression(expression_parts: &mut Vec<&str>, options: &Options) {
                expression_parts[3] = &*expression_parts[3].replace("?", "*");
                expression_parts[5] = &*expression_parts[5].replace("?", "*");
                expression_parts[0] = if expression_parts[0].starts_with("0/") { // seconds
                    &*expression_parts[0].replace("0/", "*/")
                } else {
                    expression_parts[0]
                };
                expression_parts[1] = if expression_parts[1].starts_with("0/") { // minutes
                    &*expression_parts[1].replace("0/", "*/")
                } else {
                    expression_parts[1]
                };
                expression_parts[2] = if expression_parts[2].starts_with("0/") { // hours
                    &*expression_parts[2].replace("0/", "*/")
                } else {
                    expression_parts[2]
                };
                expression_parts[3] = if expression_parts[3].starts_with("1/") { // hours
                    &*expression_parts[3].replace("1/", "*/")
                } else {
                    expression_parts[3]
                };
                expression_parts[4] = if expression_parts[4].starts_with("1/") { // hours
                    &*expression_parts[4].replace("1/", "*/")
                } else {
                    expression_parts[4]
                };
                expression_parts[5] = if expression_parts[5].starts_with("1/") { // hours
                    &*expression_parts[5].replace("1/", "*/")
                } else {
                    expression_parts[5]
                };

                fn is_numeric(s: &str) -> bool {
                    let mut is_num = true;
                    for c in s {
                        if ! c.is_numeric() {
                            is_num = false;
                            break
                        }
                    }
                    is_num
                }

                for i in 0..expression_parts.len() {
                    if expression_parts[i] == "*/1" {
                        expression_parts[i] = "*";
                    }
                }

                if ! is_numeric(expression_parts[5]) {
                   for i in 0..7 {
                       expression_parts[5] = &*expression_parts[5]
                   }
                }


            }
        }

        pub fn get_description(description_type: DescriptionTypeEnum,
                               expression: &String,
                               options: &cronparser::Options,
                               locale: &String) -> String {
            rust_i18n::set_locale(locale);
            let expression_parts = expression_parser::parse(expression, options);
            let descriptionRes = match description_type {
                DescriptionTypeEnum::FULL => get_full_description(expression_parts, options),
                DescriptionTypeEnum::TIMEOFDAY => {}
                DescriptionTypeEnum::SECONDS => {}
                DescriptionTypeEnum::MINUTES => {}
                DescriptionTypeEnum::HOURS => {}
                DescriptionTypeEnum::DAYOFWEEK => {}
                DescriptionTypeEnum::MONTH => {}
                DescriptionTypeEnum::DAYOFMONTH => {}
                DescriptionTypeEnum::YEAR => {}
            };
            todo!()
        }

        pub fn get_description_1(cron: &String) -> String {
            todo!()
        }

        pub fn get_description_2(cron: &String, options: &cronparser::Options) -> String {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cronparser::cron_expression_descriptor;
    use crate::cronparser::Options;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_localized_text() {
        assert_eq!("5 through 9", t!("messages.between_weekday_description_format", 0 = "5", 1 = "9"));
        let i18 = t!("messages.on_the_day_of_the_month", 0 = "monday");
        let nth_str = i18.replace("{{nth}}", "first");
        assert_eq!("on the first monday of the month", nth_str);
    }


    #[test]
    fn test_every_second() {
        assert_eq!("Every second", cron_expression_descriptor::get_description_2(String::from("* * * * * *")));
        assert_eq!("Every second", cron_expression_descriptor::get_description_2(String::from("* * * * * *"), Options::twenty_four_hour()));
    }

    #[test]
    fn test_every45seconds() {
        assert_eq!("Every 45 seconds", cron_expression_descriptor::get_description_2("*/45 * * * * *"));
        assert_eq!("Every 45 seconds", cron_expression_descriptor::get_description_2("*/45 * * * * *", Options::twenty_four_hour()));
    }

    #[test]
    fn test_minute_span() {
        assert_eq!("Every minute between 11:00 AM and 11:10 AM", cron_expression_descriptor::get_description_2("0-10 11 * * *"));
        assert_eq!("Every minute between 11:00 and 11:10", cron_expression_descriptor::get_description_2("0-10 11 * * *", Options::twenty_four_hour()));
        assert_eq!("Every minute, at 1:00 AM", cron_expression_descriptor::get_description_2("* 1 * * *"));
        assert_eq!("Every minute, at 12:00 AM", cron_expression_descriptor::get_description_2("* 0 * * *"));
    }

    #[test]
    fn test_every_minute() {
        assert_eq!("Every minute", cron_expression_descriptor::get_description_2("* * * * *"));
        assert_eq!("Every minute", cron_expression_descriptor::get_description_2("*/1 * * * *"));
        assert_eq!("Every minute", cron_expression_descriptor::get_description_2("0 0/1 * * * ?"));
    }

    #[test]
    fn test_every_hour() {
        assert_eq!("Every hour", cron_expression_descriptor::get_description_2("0 0 * * * ?"));
        assert_eq!("Every hour", cron_expression_descriptor::get_description_2("0 0 0/1 * * ?"));
        assert_eq!("Every hour", cron_expression_descriptor::get_description_2("0 * * * *"));
    }

    #[test]
    fn test_every_xminutes() {
        assert_eq!("Every 5 minutes", cron_expression_descriptor::get_description_2("*/5 * * * *"));
        assert_eq!("Every 5 minutes", cron_expression_descriptor::get_description_2("0 */5 * * * *"));
        assert_eq!("Every 10 minutes", cron_expression_descriptor::get_description_2("0 0/10 * * * ?"));
    }

    #[test]
    fn test_daily_at_time() {
        assert_eq!("At 11:30 AM", cron_expression_descriptor::get_description_2("30 11 * * *"));
        assert_eq!("At 11:30", cron_expression_descriptor::get_description_2("30 11 * * *", Options::twenty_four_hour()));
        assert_eq!("At 11:00 AM", cron_expression_descriptor::get_description_2("0 11 * * *"));
    }

    #[test]
    fn test_time_of_day_certain_days_of_week() {
        assert_eq!("At 11:00 PM, Monday through Friday", cron_expression_descriptor::get_description_2("0 23 ? * MON-FRI"));
        assert_eq!("At 23:00, Monday through Friday", cron_expression_descriptor::get_description_2("0 23 ? * MON-FRI", Options::twenty_four_hour()));
        assert_eq!("At 11:30 AM, Monday through Friday", cron_expression_descriptor::get_description_2("30 11 * * 1-5"));
    }

    #[test]
    fn test_one_month_only() {
        assert_eq!("Every minute, only in March", cron_expression_descriptor::get_description_2("* * * 3 *"));
    }

    #[test]
    fn test_two_months_only() {
        assert_eq!("Every minute, only in March and June", cron_expression_descriptor::get_description_2("* * * 3,6 *"));
    }

    #[test]
    fn test_two_times_each_afternoon() {
        assert_eq!("At 2:30 PM and 4:30 PM", cron_expression_descriptor::get_description_2("30 14,16 * * *"));
        assert_eq!("At 14:30 and 16:30", cron_expression_descriptor::get_description_2("30 14,16 * * *", Options::twenty_four_hour()));
    }

    #[test]
    fn test_three_times_daily() {
        assert_eq!("At 6:30 AM, 2:30 PM and 4:30 PM", cron_expression_descriptor::get_description_2("30 6,14,16 * * *"));
        assert_eq!("At 06:30, 14:30 and 16:30", cron_expression_descriptor::get_description_2("30 6,14,16 * * *", Options::twenty_four_hour()));
    }

    #[test]
    fn test_once_aweek() {
        assert_eq!("At 9:46 AM, only on Sunday", cron_expression_descriptor::get_description_2("46 9 * * 0"));
        assert_eq!("At 9:46 AM, only on Sunday", cron_expression_descriptor::get_description_2("46 9 * * 7"));
        assert_eq!("At 9:46 AM, only on Monday", cron_expression_descriptor::get_description_2("46 9 * * 1"));
        assert_eq!("At 9:46 AM, only on Saturday", cron_expression_descriptor::get_description_2("46 9 * * 6"));
    }

    #[test]
    fn test_once_aweek_non_zero_based() {
        let opts = Options::options();
        let options = Options {
            zero_based_day_of_week: false,
            ..opts
        };
        assert_eq!("At 9:46 AM, only on Sunday", cron_expression_descriptor::get_description_2("46 9 * * 1", options));
        assert_eq!("At 9:46 AM, only on Monday", cron_expression_descriptor::get_description_2("46 9 * * 2", options));
        assert_eq!("At 9:46 AM, only on Saturday", cron_expression_descriptor::get_description_2("46 9 * * 7", options));
    }

    #[test]
    fn test_twice_aweek() {
        assert_eq!("At 9:46 AM, only on Monday and Tuesday", cron_expression_descriptor::get_description_2("46 9 * * 1,2"));
        assert_eq!("At 9:46 AM, only on Sunday and Saturday", cron_expression_descriptor::get_description_2("46 9 * * 0,6"));
        assert_eq!("At 9:46 AM, only on Saturday and Sunday", cron_expression_descriptor::get_description_2("46 9 * * 6,7"));
    }

    #[test]
    fn test_twice_aweek_non_zero_based() {
        let options = Options {
            zero_based_day_of_week: false,
            ..Options::options()
        };
        assert_eq!("At 9:46 AM, only on Sunday and Monday", cron_expression_descriptor::get_description_2("46 9 * * 1,2", options));
        assert_eq!("At 9:46 AM, only on Friday and Saturday", cron_expression_descriptor::get_description_2("46 9 * * 6,7", options));
    }

    #[test]
    fn test_day_of_month() {
        assert_eq!("At 12:23 PM, on day 15 of the month", cron_expression_descriptor::get_description_2("23 12 15 * *"));
        assert_eq!("At 12:23, on day 15 of the month", cron_expression_descriptor::get_description_2("23 12 15 * *", Options::twenty_four_hour()));
    }

    #[test]
    fn test_month_name() {
        assert_eq!("At 12:23 PM, only in January", cron_expression_descriptor::get_description_2("23 12 * JAN *"));
    }

    #[test]
    fn test_day_of_month_with_question_mark() {
        assert_eq!("At 12:23 PM, only in January", cron_expression_descriptor::get_description_2("23 12 ? JAN *"));
    }

    #[test]
    fn test_month_name_range2() {
        assert_eq!("At 12:23 PM, January through February", cron_expression_descriptor::get_description_2("23 12 * JAN-FEB *"));
    }

    #[test]
    fn test_month_name_range3() {
        assert_eq!("At 12:23 PM, January through March", cron_expression_descriptor::get_description_2("23 12 * JAN-MAR *"));
    }

    #[test]
    fn test_month_name_ranges() {
        assert_eq!("At 3:00 AM, only in January through March and May through June", cron_expression_descriptor::get_description_2("0 0 3 * 1-3,5-6 *"));
    }

    #[test]
    fn test_day_of_week_name() {
        assert_eq!("At 12:23 PM, only on Sunday", cron_expression_descriptor::get_description_2("23 12 * * SUN"));
    }

    #[test]
    fn test_day_of_week_range() {
        assert_eq!("Every 5 minutes, at 3:00 PM, Monday through Friday", cron_expression_descriptor::get_description_2("*/5 15 * * MON-FRI"));
        assert_eq!("Every 5 minutes, at 3:00 PM, Sunday through Saturday", cron_expression_descriptor::get_description_2("*/5 15 * * 0-6"));
        assert_eq!("Every 5 minutes, at 3:00 PM, Saturday through Sunday", cron_expression_descriptor::get_description_2("*/5 15 * * 6-7"));
    }

    #[test]
    fn test_day_of_week_ranges() {
        assert_eq!("At 3:00 AM, only on Sunday, Tuesday through Thursday and Saturday", cron_expression_descriptor::get_description_2("0 0 3 * * 0,2-4,6"));
    }

    #[test]
    fn test_day_of_week_once_in_month() {
        assert_eq!("Every minute, on the third Monday of the month", cron_expression_descriptor::get_description_2("* * * * MON#3"));
        assert_eq!("Every minute, on the third Sunday of the month", cron_expression_descriptor::get_description_2("* * * * 0#3"));
    }

    #[test]
    fn test_last_day_of_the_week_of_the_month() {
        assert_eq!("Every minute, on the last Thursday of the month", cron_expression_descriptor::get_description_2("* * * * 4L"));
        assert_eq!("Every minute, on the last Sunday of the month", cron_expression_descriptor::get_description_2("* * * * 0L"));
    }

    #[test]
    fn test_last_day_of_the_month() {
        assert_eq!("Every 5 minutes, on the last day of the month, only in January", cron_expression_descriptor::get_description_2("*/5 * L JAN *"));
    }

    #[test]
    fn test_time_of_day_with_seconds() {
        assert_eq!("At 2:02:30 PM", cron_expression_descriptor::get_description_2("30 02 14 * * *"));
    }

    #[test]
    fn test_second_internvals() {
        assert_eq!("Seconds 5 through 10 past the minute", cron_expression_descriptor::get_description_2("5-10 * * * * *"));
    }

    #[test]
    fn test_second_minutes_hours_intervals() {
        assert_eq!("Seconds 5 through 10 past the minute, minutes 30 through 35 past the hour, between 10:00 AM and 12:00 PM",
                   cron_expression_descriptor::get_description_2("5-10 30-35 10-12 * * *"));
    }

    #[test]
    fn test_every5minutes_at30seconds() {
        assert_eq!("At 30 seconds past the minute, every 5 minutes", cron_expression_descriptor::get_description_2("30 */5 * * * *"));
    }

    #[test]
    fn test_minutes_past_the_hour_range() {
        assert_eq!("At 30 minutes past the hour, between 10:00 AM and 1:00 PM, only on Wednesday and Friday",
                   cron_expression_descriptor::get_description_2("0 30 10-13 ? * WED,FRI"));
    }

    #[test]
    fn test_seconds_past_the_minute_interval() {
        assert_eq!("At 10 seconds past the minute, every 5 minutes", cron_expression_descriptor::get_description_2("10 0/5 * * * ?"));
    }

    #[test]
    fn test_between_with_interval() {
        assert_eq!("Every 3 minutes, minutes 02 through 59 past the hour, at 1:00 AM, 9:00 AM and 10:00 PM, between day 11 and 26 of the month, January through June",
                   cron_expression_descriptor::get_description_2("2-59/3 1,9,22 11-26 1-6 ?"));
    }

    #[test]
    fn test_recurring_first_of_month() {
        assert_eq!("At 6:00 AM", cron_expression_descriptor::get_description_2("0 0 6 1/1 * ?"));
    }

    #[test]
    fn test_minutes_past_the_hour() {
        assert_eq!("At 05 minutes past the hour", cron_expression_descriptor::get_description_2("0 5 0/1 * * ?"));
    }

    /**
     * @since https://github.com/RedHogs/cron-parser/issues/2
     */
    #[test]
    fn test_every_past_the_hour() {
        assert_eq!("At 00, 05, 10, 15, 20, 25, 30, 35, 40, 45, 50 and 55 minutes past the hour", cron_expression_descriptor::get_description_2("0 0,5,10,15,20,25,30,35,40,45,50,55 * ? * *"));
    }

    /**
     * @since https://github.com/RedHogs/cron-parser/issues/10
     */
    #[test]
    fn test_every_xminute_past_the_hour_with_interval() {
        assert_eq!("Every 2 minutes, minutes 00 through 30 past the hour, at 5:00 PM, Monday through Friday", cron_expression_descriptor::get_description_2("0 0-30/2 17 ? * MON-FRI"));
    }

    /**
     * @since https://github.com/RedHogs/cron-parser/issues/13
     */
    #[test]
    fn test_one_year_only_with_seconds() {
        assert_eq!("Every second, only in 2013", cron_expression_descriptor::get_description_2("* * * * * * 2013"));
    }

    #[test]
    fn test_one_year_only_without_seconds() {
        assert_eq!("Every minute, only in 2013", cron_expression_descriptor::get_description_2("* * * * * 2013"));
    }

    #[test]
    fn test_two_years_only() {
        assert_eq!("Every minute, only in 2013 and 2014", cron_expression_descriptor::get_description_2("* * * * * 2013,2014"));
    }

    #[test]
    fn test_year_range2() {
        assert_eq!("At 12:23 PM, January through February, 2013 through 2014", cron_expression_descriptor::get_description_2("23 12 * JAN-FEB * 2013-2014"));
    }

    #[test]
    fn test_year_range3() {
        assert_eq!("At 12:23 PM, January through March, 2013 through 2015", cron_expression_descriptor::get_description_2("23 12 * JAN-MAR * 2013-2015"));
    }

    #[test]
    fn test_issue26() {
        assert_eq!("At 05 and 10 minutes past the hour", cron_expression_descriptor::get_description_2("5,10 * * * *"));
        assert_eq!("At 05 and 10 minutes past the hour, at 12:00 AM", cron_expression_descriptor::get_description_2("5,10 0 * * *"));
        assert_eq!("At 05 and 10 minutes past the hour, on day 2 of the month", cron_expression_descriptor::get_description_2("5,10 * 2 * *"));
        assert_eq!("Every 10 minutes, on day 2 of the month", cron_expression_descriptor::get_description_2("5/10 * 2 * *"));

        assert_eq!("At 5 and 6 seconds past the minute", cron_expression_descriptor::get_description_2("5,6 0 * * * *"));
        assert_eq!("At 5 and 6 seconds past the minute, at 1:00 AM", cron_expression_descriptor::get_description_2("5,6 0 1 * * *"));
        assert_eq!("At 5 and 6 seconds past the minute, on day 2 of the month", cron_expression_descriptor::get_description_2("5,6 0 * 2 * *"));
    }
}
