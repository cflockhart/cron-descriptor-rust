#[macro_use]
extern crate rust_i18n;
#[macro_use]
extern crate strfmt;

mod description_builder;


i18n!("locales");

mod string_utils {
    pub fn not_contains_any(str: &String, chars: &[char]) -> bool {
        str.chars().all(|c| ! chars.contains(&c))
    }

    pub fn is_numeric(s: &str) -> bool {
        for c in s.chars() {
            if !c.is_numeric() {
                return false;
            }
        }
        return true;
    }
}
mod date_time_utils {
    static DAYS_OF_WEEK_ARR: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    static MONTHS_ARR: [&str; 12] = ["JAN", "FEB", "MAR", "APR", "MAY", "JUN",
        "JUL", "AUG", "SEP", "OCT", "NOV", "DEC"];

    use chrono::Weekday;
    // use datetime::{ISO, LocalTime};
    // use datetime::fmt;
    use crate::cronparser::Options;

    pub fn format_time(hours_expression: &String, minutes_expression: &String, opts: &Options) -> String {
        format_time_secs(&hours_expression, &minutes_expression, &"".to_string(), opts)
    }

    pub fn format_time_secs(hours_expression: &String,
                            minutes_expression: &String,
                            seconds_expression: &String,
                            opts: &Options) -> String {
        let mut hour: i8 = hours_expression.parse().unwrap();
        let mut period: String = "".to_string();

        if !opts.twenty_four_hour_time {
            period = if hour >= 12 {
                t!("time_pm")
            } else {
                t!("time_am")
            };
            if !period.len() > 0 {
                period = " ".to_string() + &period;
            }
            if hour > 12 {
                hour -= 12;
            }
            if hour == 0 {
                hour = 12;
            }
        }

        let minutes = minutes_expression.parse::<i8>().unwrap().to_string();
        let mut seconds: String = "".to_string();

        if !seconds_expression.is_empty() {
            seconds = ":".to_string() + &seconds_expression.parse::<i8>().unwrap().to_string();
            seconds = format!("{:0>2}", seconds);
        }

        format!("{0}:{1}{2}{3}", format!("{:0>2}", hour), format!("{:0>2}", minutes), seconds, period)
    }


    pub fn get_day_of_week_name(day_of_week: usize) -> String {
        use chrono::Weekday;
        let day_str = DAYS_OF_WEEK_ARR[day_of_week % 7];
        t!(day_str)
    }
}


pub fn format_minutes(minutes_expression: &str) -> String {
    todo!()
}

mod cronparser {
    pub enum CasingTypeEnum {
        Title,
        Sentence,
        LowerCase,
    }

    pub enum DescriptionTypeEnum {
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
        pub throw_exception_on_parse_error: bool,
        pub casing_type: CasingTypeEnum,
        pub verbose: bool,
        pub zero_based_day_of_week: bool,
        pub twenty_four_hour_time: bool,
        pub need_space_between_words: bool,
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
        use string_builder::Builder;

        use crate::cronparser;
        use crate::cronparser::{CasingTypeEnum, DescriptionTypeEnum, Options};
        use crate::date_time_utils::{format_time, format_time_secs};

        const SPECIAL_CHARACTERS: [char; 4] = ['/', '-', ',', '*'];

        #[derive(Debug)]
        pub struct ParseException {
            s: String,
            error_offset: u8,
        }

        mod expression_parser {
            /* Cron reference
      ┌───────────── minute (0 - 59)
      │ ┌───────────── hour (0 - 23)
      │ │ ┌───────────── day of month (1 - 31)
      │ │ │ ┌───────────── month (1 - 12)
      │ │ │ │ ┌───────────── day of week (0 - 6) (Sunday to Saturday; 7 is also Sunday on some systems)
      │ │ │ │ │
      │ │ │ │ │
      │ │ │ │ │
      * * * * *  command to execute
     */



            use lazy_static::lazy_static;
            use regex::Regex;

            use crate::cronparser::cron_expression_descriptor::ParseException;
            use crate::cronparser::Options;

            pub fn parse(expression: String, options: &Options) -> Result<Vec<String>, ParseException> {
                let mut parsed: Vec<&str> = vec![""; 7];
                if expression.trim().is_empty() {
                    lazy_static! {
                        static ref err_str: String = t!("expression_empty_exception");
                    }
                    let result = Err(ParseException {
                        s: expression,
                        error_offset: 0
                    });
                    result
                } else {
                    let expression_parts: Vec<&str> = expression.trim().split_whitespace().collect();
                    if expression_parts.len() < 5 {
                        let result1 = Err(ParseException {
                            s: expression,
                            error_offset: 0
                        });
                        return result1;
                    } else if expression_parts.len() == 5 {
                        parsed[0] = "";
                        (1..5).for_each(|i| parsed[i] = expression_parts[i - 1]);
                    } else if expression_parts.len() == 6 {
                        lazy_static! {
                            static ref YEAR_RE: Regex = Regex::new(r"\d{4}$").unwrap();
                        }
                        if YEAR_RE.is_match(expression_parts[5]) {
                            (1..6).for_each(|i| parsed[i] = expression_parts[i - 1]);
                        } else {
                            (0..5).for_each(|i| parsed[i] = expression_parts[i]);
                        }
                    } else if expression_parts.len() == 7 {
                        (0..6).for_each(|i| parsed[i] = expression_parts[i]);
                    } else {
                        let result2 = Err(ParseException {
                            s: expression,
                            error_offset: 7
                        });
                        return result2;
                    }

                    let normalized_expr = normalise_expression(parsed, options);
                    Ok(normalized_expr)
                }
            }

            fn normalise_expression(expression_parts: Vec<&str>, options: &Options) -> Vec<String> {
                static DAYS_OF_WEEK_ARR: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
                static MONTHS_ARR: [&str; 12] = ["JAN", "FEB", "MAR", "APR", "MAY", "JUN",
                    "JUL", "AUG", "SEP", "OCT", "NOV", "DEC"];
                let mut normalised: Vec<String> = vec!["".to_string(); 7];

                normalised[3] = expression_parts[3].replace("?", "*");
                normalised[5] = expression_parts[5].clone().replace("?", "*");
                normalised[0] = if expression_parts[0].starts_with("0/") { // seconds
                    expression_parts[0].replace("0/", "*/")
                } else {
                    expression_parts[0].to_string()
                };
                normalised[1] = if expression_parts[1].starts_with("0/") { // minutes
                    expression_parts[1].replace("0/", "*/")
                } else {
                    expression_parts[1].to_string()
                };
                normalised[2] = if expression_parts[2].starts_with("0/") { // hours
                    expression_parts[2].replace("0/", "*/")
                } else {
                    expression_parts[2].to_string()
                };
                normalised[3] = if expression_parts[3].starts_with("1/") { // hours
                    expression_parts[3].replace("1/", "*/")
                } else {
                    expression_parts[3].to_string()
                };
                normalised[4] = if expression_parts[4].starts_with("1/") { // hours
                    expression_parts[4].replace("1/", "*/")
                } else {
                    expression_parts[4].to_string()
                };
                normalised[5] = if expression_parts[5].starts_with("1/") { // hours
                    expression_parts[5].replace("1/", "*/")
                } else {
                    expression_parts[5].to_string()
                };

                fn is_numeric(s: &str) -> bool {
                    for c in s.chars() {
                        if !c.is_numeric() {
                            return false;
                        }
                    }
                    return true;
                }

                for i in 0..expression_parts.len() {
                    if expression_parts[i] == "*/1" {
                        normalised[i] = "*".to_string();
                    }
                }
                /*

                        // convert SUN-SAT format to 0-6 format
                        if(!StringUtils.isNumeric(expressionParts[5])) {
                            for (int i = 0; i <= 6; i++) {
                                expressionParts[5] = expressionParts[5].clone().replace(DateAndTimeUtils.getDayOfWeekName(i + 1), String.valueOf(i));
                            }
                        }

                        // convert JAN-DEC format to 1-12 format
                        if(!StringUtils.isNumeric(expressionParts[4])) {
                            for (int i = 1; i <= 12; i++) {
                                DateTime currentMonth = new DateTime().withDayOfMonth(1).withMonthOfYear(i);
                                String currentMonthDescription = currentMonth.toString("MMM", Locale.ENGLISH).toUpperCase();
                                expressionParts[4] = expressionParts[4].clone().replace(currentMonthDescription, String.valueOf(i));
                            }
                        }

                        // convert 0 second to (empty)
                        if ("0".equals(expressionParts[0])) {
                            expressionParts[0] = StringUtils.EMPTY;
                        }

                        // convert 0 DOW to 7 so that 0 for Sunday in zeroBasedDayOfWeek is valid
                        if((options == null || options.isZeroBasedDayOfWeek()) && "0".equals(expressionParts[5])) {
                            expressionParts[5] = "7";
                        }
                 */
                if !is_numeric(expression_parts[5]) {
                    for i in 0..=6 {
                        normalised[5] = expression_parts[5].replace(DAYS_OF_WEEK_ARR[i], i.to_string().as_str());
                    }
                }

                if !is_numeric(expression_parts[4]) {
                    for i in 0..=11 {
                        normalised[4] = expression_parts[4].replace(MONTHS_ARR[i], i.to_string().as_str());
                    }
                }

                // convert 0 second to (empty)
                if "0" == expression_parts[0] {
                    normalised[0] = "".to_string();
                }

                // convert 0 DOW to 7 so that 0 for Sunday in zeroBasedDayOfWeek is valid
                // this logic is copied from the Java version and seems different than the C#
                // version.
                if options.zero_based_day_of_week && "0" == expression_parts[5] {
                    normalised[5] = "7".to_string();
                }

                // Bunch of logic in the C# version is missing from the Java version,
                // such as regex handling of the DOW, stepping and between ranges.
                normalised
            }
        }

        pub fn get_description(description_type: DescriptionTypeEnum,
                               expression: String,
                               options: &cronparser::Options,
                               locale: String) -> String {
            rust_i18n::set_locale(&locale);
            let expression_parts = expression_parser::parse(expression, options).unwrap();
            // TODO fill out the rest of the get* functions.

            let description_res = match description_type {
                DescriptionTypeEnum::FULL => get_full_description(expression_parts, options),
                DescriptionTypeEnum::TIMEOFDAY => get_time_of_day_description(&expression_parts, options),
                DescriptionTypeEnum::SECONDS => get_seconds_description(&expression_parts, options),
                DescriptionTypeEnum::MINUTES => get_minutes_description(&expression_parts, options),
                DescriptionTypeEnum::HOURS => get_hours_description(&expression_parts, options),
                DescriptionTypeEnum::DAYOFWEEK => get_day_of_week_description(&expression_parts, options),
                DescriptionTypeEnum::MONTH => get_month_description(&expression_parts, options),
                DescriptionTypeEnum::DAYOFMONTH => get_day_of_month_description(&expression_parts, options),
                DescriptionTypeEnum::YEAR => get_year_description(&expression_parts, options)
            };
            description_res
        }

        // TODO 2022-04-23 fill out these get*description functions and drill down to fill it all out.

        // From the C# code, not Java.
        fn get_full_description(expression_parts: Vec<String>, options: &Options) -> String {
            let time_segment = get_time_of_day_description(&expression_parts, options);
            let day_of_month_desc = get_day_of_month_description(&expression_parts, options);
            let month_desc = get_month_description(&expression_parts, options);
            let day_of_week_desc = get_day_of_week_description(&expression_parts, options);
            let year_desc = get_year_description(&expression_parts, options);
            let desc1 = format!("{0}{1}{2}{3}{4}",
                                time_segment,
                                day_of_month_desc,
                                day_of_week_desc,
                                month_desc,
                                year_desc);
            let desc2 = transform_verbosity(desc1, options);
            transform_case(&desc2, options)
        }


        fn transform_verbosity(description: String, options: &Options) -> String {
            let mut desc_temp = description.clone();
            if !options.verbose {
                desc_temp = desc_temp.replace(&t!("messages.every_minute"), &t!("every_minute"));
                desc_temp = desc_temp.replace(&t!("messages.every_1_hour"), &t!("every_hour"));
            }
            desc_temp
        }

        fn transform_case(description: &str, options: &Options) -> String {
            match &options.casing_type {
                CasingTypeEnum::Sentence => description[0..1].to_uppercase() + &description[1..],
                CasingTypeEnum::Title => description[0..1].to_uppercase() + &description[1..],
                CasingTypeEnum::LowerCase => description.to_lowercase()
            }
        }

        fn get_year_description(expression_parts: &Vec<String>, options: &Options) -> String {
            todo!()
        }

        fn get_day_of_week_description(expression_parts: &Vec<String>, options: &Options) -> String {
            todo!()
        }

        fn get_minutes_description(expression_parts: &Vec<String>, options: &Options) -> String {
            todo!()
        }

        fn get_seconds_description(expression_parts: &Vec<String>, options: &Options) -> String {
            // Use the builder structure from the Java version
            todo!()
        }

        fn get_hours_description(expression_parts: &Vec<String>, options: &Options) -> String {
            todo!()
        }

        fn get_month_description(expression_parts: &Vec<String>, options: &Options) -> String {
            todo!()
        }

        fn get_day_of_month_description(expression_parts: &Vec<String>, options: &Options) -> String {
            todo!()
        }

        fn get_time_of_day_description(expression_parts: &Vec<String>, options: &Options) -> String {
            let seconds_expression = &expression_parts[0];
            let minutes_expression = &expression_parts[1];
            let hours_expression = &expression_parts[2];

            let mut description = Builder::default();

            if minutes_expression.chars().all(|c| !SPECIAL_CHARACTERS.contains(&c))
                && hours_expression.chars().all(|c| !SPECIAL_CHARACTERS.contains(&c))
                && seconds_expression.chars().all(|c| !SPECIAL_CHARACTERS.contains(&c)) {
                description.append(t!("at"));
                if options.need_space_between_words {
                    description.append(" ");
                }
                description.append(format_time_secs(hours_expression,
                                                    minutes_expression,
                                                    seconds_expression,
                                                    options));
            } else if seconds_expression == "" && minutes_expression.contains("-")
                && !minutes_expression.contains(",")
                && hours_expression.chars().all(|c| !SPECIAL_CHARACTERS.contains(&c)) {
                let mut minute_parts = minutes_expression.split("-");
                let msg0 = format_time(hours_expression,
                                       &minute_parts.next().unwrap().to_string(),
                                       options);
                let msg1 = format_time(hours_expression,
                                       &minute_parts.next().unwrap().to_string(),
                                       options);
                description.append(t!("messages.every_minute_between",0 = &msg0, 1 = &msg1));
            } else if seconds_expression == "" && hours_expression.contains(",")
                && !hours_expression.contains("-")
                && !minutes_expression.chars().all(|c| !SPECIAL_CHARACTERS.contains(&c)) {
                let hour_parts: Vec<_> = hours_expression.split(",").collect();
                let hpsz = hour_parts.len();
                description.append(t!("at"));

                for (i, hp) in hour_parts.iter().enumerate() {
                    description.append(" ");
                    description.append(format_time(&hp.to_string(),
                                                   minutes_expression, options));
                    if i < hpsz - 2 {
                        description.append(",");
                    }
                    if i == hpsz - 2 {
                        description.append(" ");
                        description.append(t!("and"));
                    }
                }


            } else {
                let seconds_description = get_seconds_description(expression_parts, options);
                let minutes_description = get_minutes_description(expression_parts, options);
                let hours_description = get_hours_description(expression_parts, options);



            }
            "".to_string()
            // return description.to_string();
        }

        pub fn get_description_1(cron: String) -> String { todo!() }

        pub fn get_description_2(cron: String, options: &cronparser::Options) -> String {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate strfmt;

    use std::collections::HashMap;

    use strfmt::strfmt;

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
        let nth_str = i18.replace("{nth}", "first");
        assert_eq!("on the first monday of the month", nth_str);
    }

    #[test]
    fn test_strfmt_text() {
        let mut vars = HashMap::new();
        vars.insert("name".to_string(), "Craig");
        let i18 = t!("messages.on_the_day_of_the_month", 0 = "monday");
        let formatted = strfmt(&i18, &vars).unwrap();
        assert_eq!("on the Craig monday of the month", formatted);
    }

    #[test]
    fn test_every_second() {
        assert_eq!("Every second", cron_expression_descriptor::get_description_1(String::from("* * * * * *")));
        assert_eq!("Every second", cron_expression_descriptor::get_description_2(String::from("* * * * * *"), &Options::twenty_four_hour()));
    }

    #[test]
    fn test_every45seconds() {
        assert_eq!("Every 45 seconds", cron_expression_descriptor::get_description_1("*/45 * * * * *".to_string()));
        assert_eq!("Every 45 seconds", cron_expression_descriptor::get_description_2("*/45 * * * * *".to_string(), &Options::twenty_four_hour()));
    }

    #[test]
    fn test_minute_span() {
        assert_eq!("Every minute between 11:00 AM and 11:10 AM", cron_expression_descriptor::get_description_1("0-10 11 * * *".to_string()));
        assert_eq!("Every minute between 11:00 and 11:10", cron_expression_descriptor::get_description_2("0-10 11 * * *".to_string(), &Options::twenty_four_hour()));
        assert_eq!("Every minute, at 1:00 AM", cron_expression_descriptor::get_description_1("* 1 * * *".to_string()));
        assert_eq!("Every minute, at 12:00 AM", cron_expression_descriptor::get_description_1("* 0 * * *".to_string()));
    }

    #[test]
    fn test_every_minute() {
        assert_eq!("Every minute", cron_expression_descriptor::get_description_1("* * * * *".to_string()));
        assert_eq!("Every minute", cron_expression_descriptor::get_description_1("*/1 * * * *".to_string()));
        assert_eq!("Every minute", cron_expression_descriptor::get_description_1("0 0/1 * * * ?".to_string()));
    }

    #[test]
    fn test_every_hour() {
        assert_eq!("Every hour", cron_expression_descriptor::get_description_1("0 0 * * * ?".to_string()));
        assert_eq!("Every hour", cron_expression_descriptor::get_description_1("0 0 0/1 * * ?".to_string()));
        assert_eq!("Every hour", cron_expression_descriptor::get_description_1("0 * * * *".to_string()));
    }

    #[test]
    fn test_every_xminutes() {
        assert_eq!("Every 5 minutes", cron_expression_descriptor::get_description_1("*/5 * * * *".to_string()));
        assert_eq!("Every 5 minutes", cron_expression_descriptor::get_description_1("0 */5 * * * *".to_string()));
        assert_eq!("Every 10 minutes", cron_expression_descriptor::get_description_1("0 0/10 * * * ?".to_string()));
    }

    #[test]
    fn test_daily_at_time() {
        assert_eq!("At 11:30 AM", cron_expression_descriptor::get_description_1("30 11 * * *".to_string()));
        assert_eq!("At 11:30", cron_expression_descriptor::get_description_2("30 11 * * *".to_string(), &Options::twenty_four_hour()));
        assert_eq!("At 11:00 AM", cron_expression_descriptor::get_description_1("0 11 * * *".to_string()));
    }

    #[test]
    fn test_time_of_day_certain_days_of_week() {
        assert_eq!("At 11:00 PM, Monday through Friday", cron_expression_descriptor::get_description_1("0 23 ? * MON-FRI".to_string()));
        assert_eq!("At 23:00, Monday through Friday", cron_expression_descriptor::get_description_2("0 23 ? * MON-FRI".to_string(), &Options::twenty_four_hour()));
        assert_eq!("At 11:30 AM, Monday through Friday", cron_expression_descriptor::get_description_1("30 11 * * 1-5".to_string()));
    }

    #[test]
    fn test_one_month_only() {
        assert_eq!("Every minute, only in March", cron_expression_descriptor::get_description_1("* * * 3 *".to_string()));
    }

    #[test]
    fn test_two_months_only() {
        assert_eq!("Every minute, only in March and June", cron_expression_descriptor::get_description_1("* * * 3,6 *".to_string()));
    }

    #[test]
    fn test_two_times_each_afternoon() {
        assert_eq!("At 2:30 PM and 4:30 PM", cron_expression_descriptor::get_description_1("30 14,16 * * *".to_string()));
        assert_eq!("At 14:30 and 16:30", cron_expression_descriptor::get_description_2("30 14,16 * * *".to_string(), &Options::twenty_four_hour()));
    }

    #[test]
    fn test_three_times_daily() {
        assert_eq!("At 6:30 AM, 2:30 PM and 4:30 PM", cron_expression_descriptor::get_description_1("30 6,14,16 * * *".to_string()));
        assert_eq!("At 06:30, 14:30 and 16:30", cron_expression_descriptor::get_description_2("30 6,14,16 * * *".to_string(), &Options::twenty_four_hour()));
    }

    #[test]
    fn test_once_aweek() {
        assert_eq!("At 9:46 AM, only on Sunday", cron_expression_descriptor::get_description_1("46 9 * * 0".to_string()));
        assert_eq!("At 9:46 AM, only on Sunday", cron_expression_descriptor::get_description_1("46 9 * * 7".to_string()));
        assert_eq!("At 9:46 AM, only on Monday", cron_expression_descriptor::get_description_1("46 9 * * 1".to_string()));
        assert_eq!("At 9:46 AM, only on Saturday", cron_expression_descriptor::get_description_1("46 9 * * 6".to_string()));
    }

    #[test]
    fn test_once_aweek_non_zero_based() {
        let opts = Options::options();
        let options = Options {
            zero_based_day_of_week: false,
            ..opts
        };
        assert_eq!("At 9:46 AM, only on Sunday", cron_expression_descriptor::get_description_2("46 9 * * 1".to_string(), &options));
        assert_eq!("At 9:46 AM, only on Monday", cron_expression_descriptor::get_description_2("46 9 * * 2".to_string(), &options));
        assert_eq!("At 9:46 AM, only on Saturday", cron_expression_descriptor::get_description_2("46 9 * * 7".to_string(), &options));
    }

    #[test]
    fn test_twice_aweek() {
       assert_eq!("At 9:46 AM, only on Monday and Tuesday", cron_expression_descriptor::get_description_1("46 9 * * 1,2".to_string()));
       assert_eq!("At 9:46 AM, only on Sunday and Saturday", cron_expression_descriptor::get_description_1("46 9 * * 0,6".to_string()));
       assert_eq!("At 9:46 AM, only on Saturday and Sunday", cron_expression_descriptor::get_description_1("46 9 * * 6,7".to_string()));
    }

    #[test]
    fn test_twice_aweek_non_zero_based() {
        let options = Options {
            zero_based_day_of_week: false,
            ..Options::options()
        };
        assert_eq!("At 9:46 AM, only on Sunday and Monday", cron_expression_descriptor::get_description_2("46 9 * * 1,2".to_string(), &options));
        assert_eq!("At 9:46 AM, only on Friday and Saturday", cron_expression_descriptor::get_description_2("46 9 * * 6,7".to_string(), &options));
    }

    #[test]
    fn test_day_of_month() {
        assert_eq!("At 12:23 PM, on day 15 of the month", cron_expression_descriptor::get_description_1("23 12 15 * *".to_string()));
        assert_eq!("At 12:23, on day 15 of the month", cron_expression_descriptor::get_description_2("23 12 15 * *".to_string(), &Options::twenty_four_hour()));
    }

    #[test]
    fn test_month_name() {
        assert_eq!("At 12:23 PM, only in January", cron_expression_descriptor::get_description_1("23 12 * JAN *".to_string()));
    }

    #[test]
    fn test_day_of_month_with_question_mark() {
        assert_eq!("At 12:23 PM, only in January", cron_expression_descriptor::get_description_1("23 12 ? JAN *".to_string()));
    }

    #[test]
    fn test_month_name_range2() {
        assert_eq!("At 12:23 PM, January through February", cron_expression_descriptor::get_description_1("23 12 * JAN-FEB *".to_string()));
    }

    #[test]
    fn test_month_name_range3() {
        assert_eq!("At 12:23 PM, January through March", cron_expression_descriptor::get_description_1("23 12 * JAN-MAR *".to_string()));
    }

    #[test]
    fn test_month_name_ranges() {
        assert_eq!("At 3:00 AM, only in January through March and May through June", cron_expression_descriptor::get_description_1("0 0 3 * 1-3,5-6 *".to_string()));
    }

    #[test]
    fn test_day_of_week_name() {
        assert_eq!("At 12:23 PM, only on Sunday", cron_expression_descriptor::get_description_1("23 12 * * SUN".to_string()));
    }

    #[test]
    fn test_day_of_week_range() {
        assert_eq!("Every 5 minutes, at 3:00 PM, Monday through Friday", cron_expression_descriptor::get_description_1("*/5 15 * * MON-FRI".to_string()));
        assert_eq!("Every 5 minutes, at 3:00 PM, Sunday through Saturday", cron_expression_descriptor::get_description_1("*/5 15 * * 0-6".to_string()));
        assert_eq!("Every 5 minutes, at 3:00 PM, Saturday through Sunday", cron_expression_descriptor::get_description_1("*/5 15 * * 6-7".to_string()));
    }

    #[test]
    fn test_day_of_week_ranges() {
        assert_eq!("At 3:00 AM, only on Sunday, Tuesday through Thursday and Saturday", cron_expression_descriptor::get_description_1("0 0 3 * * 0,2-4,6".to_string()));
    }

    #[test]
    fn test_day_of_week_once_in_month() {
        assert_eq!("Every minute, on the third Monday of the month", cron_expression_descriptor::get_description_1("* * * * MON#3".to_string()));
        assert_eq!("Every minute, on the third Sunday of the month", cron_expression_descriptor::get_description_1("* * * * 0#3".to_string()));
    }

    #[test]
    fn test_last_day_of_the_week_of_the_month() {
        assert_eq!("Every minute, on the last Thursday of the month", cron_expression_descriptor::get_description_1("* * * * 4L".to_string()));
        assert_eq!("Every minute, on the last Sunday of the month", cron_expression_descriptor::get_description_1("* * * * 0L".to_string()));
    }

    #[test]
    fn test_last_day_of_the_month() {
        assert_eq!("Every 5 minutes, on the last day of the month, only in January", cron_expression_descriptor::get_description_1("*/5 * L JAN *".to_string()));
    }

    #[test]
    fn test_time_of_day_with_seconds() {
        assert_eq!("At 2:02:30 PM", cron_expression_descriptor::get_description_1("30 02 14 * * *".to_string()));
    }

    #[test]
    fn test_second_internvals() {
        assert_eq!("Seconds 5 through 10 past the minute", cron_expression_descriptor::get_description_1("5-10 * * * * *".to_string()));
    }

    #[test]
    fn test_second_minutes_hours_intervals() {
        assert_eq!("Seconds 5 through 10 past the minute, minutes 30 through 35 past the hour, between 10:00 AM and 12:00 PM",
                   cron_expression_descriptor::get_description_1("5-10 30-35 10-12 * * *".to_string()));
    }

    #[test]
    fn test_every5minutes_at30seconds() {
        assert_eq!("At 30 seconds past the minute, every 5 minutes", cron_expression_descriptor::get_description_1("30 */5 * * * *".to_string()));
    }

    #[test]
    fn test_minutes_past_the_hour_range() {
        assert_eq!("At 30 minutes past the hour, between 10:00 AM and 1:00 PM, only on Wednesday and Friday",
                   cron_expression_descriptor::get_description_1("0 30 10-13 ? * WED,FRI".to_string()));
    }

    #[test]
    fn test_seconds_past_the_minute_interval() {
        assert_eq!("At 10 seconds past the minute, every 5 minutes", cron_expression_descriptor::get_description_1("10 0/5 * * * ?".to_string()));
    }

    #[test]
    fn test_between_with_interval() {
        assert_eq!("Every 3 minutes, minutes 02 through 59 past the hour, at 1:00 AM, 9:00 AM and 10:00 PM, between day 11 and 26 of the month, January through June",
                   cron_expression_descriptor::get_description_1("2-59/3 1,9,22 11-26 1-6 ?".to_string()));
    }

    #[test]
    fn test_recurring_first_of_month() {
        assert_eq!("At 6:00 AM", cron_expression_descriptor::get_description_1("0 0 6 1/1 * ?".to_string()));
    }

    #[test]
    fn test_minutes_past_the_hour() {
        assert_eq!("At 05 minutes past the hour", cron_expression_descriptor::get_description_1("0 5 0/1 * * ?".to_string()));
    }

    /**
     * @since https://github.com/RedHogs/cron-parser/issues/2
     */
    #[test]
    fn test_every_past_the_hour() {
        assert_eq!("At 00, 05, 10, 15, 20, 25, 30, 35, 40, 45, 50 and 55 minutes past the hour", cron_expression_descriptor::get_description_1("0 0,5,10,15,20,25,30,35,40,45,50,55 * ? * *".to_string()));
    }

    /**
     * @since https://github.com/RedHogs/cron-parser/issues/10
     */
    #[test]
    fn test_every_xminute_past_the_hour_with_interval() {
        assert_eq!("Every 2 minutes, minutes 00 through 30 past the hour, at 5:00 PM, Monday through Friday", cron_expression_descriptor::get_description_1("0 0-30/2 17 ? * MON-FRI".to_string()));
    }

    /**
     * @since https://github.com/RedHogs/cron-parser/issues/13
     */
    #[test]
    fn test_one_year_only_with_seconds() {
        assert_eq!("Every second, only in 2013", cron_expression_descriptor::get_description_1("* * * * * * 2013".to_string()));
    }

    #[test]
    fn test_one_year_only_without_seconds() {
        assert_eq!("Every minute, only in 2013", cron_expression_descriptor::get_description_1("* * * * * 2013".to_string()));
    }

    #[test]
    fn test_two_years_only() {
        assert_eq!("Every minute, only in 2013 and 2014", cron_expression_descriptor::get_description_1("* * * * * 2013,2014".to_string()));
    }

    #[test]
    fn test_year_range2() {
        assert_eq!("At 12:23 PM, January through February, 2013 through 2014", cron_expression_descriptor::get_description_1("23 12 * JAN-FEB * 2013-2014".to_string()));
    }

    #[test]
    fn test_year_range3() {
        assert_eq!("At 12:23 PM, January through March, 2013 through 2015", cron_expression_descriptor::get_description_1("23 12 * JAN-MAR * 2013-2015".to_string()));
    }

    #[test]
    fn test_issue26() {
        assert_eq!("At 05 and 10 minutes past the hour", cron_expression_descriptor::get_description_1("5,10 * * * *".to_string()));
        assert_eq!("At 05 and 10 minutes past the hour, at 12:00 AM", cron_expression_descriptor::get_description_1("5,10 0 * * *".to_string()));
        assert_eq!("At 05 and 10 minutes past the hour, on day 2 of the month", cron_expression_descriptor::get_description_1("5,10 * 2 * *".to_string()));
        assert_eq!("Every 10 minutes, on day 2 of the month", cron_expression_descriptor::get_description_1("5/10 * 2 * *".to_string()));

        assert_eq!("At 5 and 6 seconds past the minute", cron_expression_descriptor::get_description_1("5,6 0 * * * *".to_string()));
        assert_eq!("At 5 and 6 seconds past the minute, at 1:00 AM", cron_expression_descriptor::get_description_1("5,6 0 1 * * *".to_string()));
        assert_eq!("At 5 and 6 seconds past the minute, on day 2 of the month", cron_expression_descriptor::get_description_1("5,6 0 * 2 * *".to_string()));
    }
}
