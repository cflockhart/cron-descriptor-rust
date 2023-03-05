#[macro_use]
extern crate rust_i18n;
extern crate strfmt;

use string_builder::Builder;

mod description_builder;
mod test;

i18n!("locales");

mod string_utils {
    pub fn not_contains_any(str: &String, chars: &[char]) -> bool {
        str.chars().all(|c| !chars.contains(&c))
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
    pub static DAYS_OF_WEEK_ARR: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    pub static MONTHS_ARR: [&str; 12] = ["january", "february", "march", "april", "may", "june",
        "july", "august", "september", "october", "november", "december"];


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
        let formatted_hours = if opts.twenty_four_hour_time {
            format!("{:0>2}", hour)
        } else {
            format!("{}", hour)
        };
        format!("{0}:{1}{2}{3}", formatted_hours, format!("{:0>2}", minutes), seconds, period)
    }


    pub fn get_day_of_week_name(day_of_week: usize) -> String {
        let day_str = DAYS_OF_WEEK_ARR[day_of_week % 7];
        t!(day_str)
    }
}


pub fn format_minutes(minutes_expression: &str) -> String {
    if minutes_expression.contains(",") {
        let mparts = minutes_expression.split(",");
        let mut formatted_expression = Builder::default();
        for mpt in mparts {
            formatted_expression.append(format!("{:02}", mpt.parse::<i8>().unwrap()));
            formatted_expression.append(",");
        }
        formatted_expression.string().unwrap()
    } else {
        format!("{:02}", minutes_expression.parse::<i8>().unwrap())
    }
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
        use std::collections::HashMap;
        use lazy_static::lazy_static;
        use string_builder::Builder;

        use crate::{cronparser, string_utils};
        use crate::cronparser::{CasingTypeEnum, DescriptionTypeEnum, Options};
        use crate::date_time_utils::{format_time, format_time_secs};
        use crate::description_builder::{DayOfMonthDescriptionBuilder, DayOfWeekDescriptionBuilder, HoursDescriptionBuilder, MinutesDescriptionBuilder, MonthDescriptionBuilder, SecondsDescriptionBuilder, YearDescriptionBuilder};
        use crate::description_builder::DescriptionBuilder;

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

            use crate::cronparser::cron_expression_descriptor::ParseException;
            use crate::cronparser::Options;
            use regex::Regex;

            pub fn parse(expression: String, options: &Options) -> Result<Vec<String>, ParseException> {
                let mut parsed: Vec<&str> = vec![""; 7];
                if expression.trim().is_empty() {
                    lazy_static! {
                        static ref ERR_STR: String = t!("expression_empty_exception");
                    }
                    let result = Err(ParseException {
                        s: expression,
                        error_offset: 0,
                    });
                    result
                } else {
                    let expression_parts: Vec<&str> = expression.trim().split_whitespace().collect();
                    if expression_parts.len() < 5 {
                        let result1 = Err(ParseException {
                            s: expression,
                            error_offset: 0,
                        });
                        return result1;
                    } else if expression_parts.len() == 5 {
                        parsed[0] = "";
                        (1..=5).for_each(|i| parsed[i] = expression_parts[i - 1]);
                        // println!("length is 5: {}", parsed[5]);
                    } else if expression_parts.len() == 6 {
                        lazy_static! {
                            static ref YEAR_RE: Regex = Regex::new(r"\d{4}$").unwrap();
                        }
                        if YEAR_RE.is_match(expression_parts[5]) {
                            (1..=6).for_each(|i| parsed[i] = expression_parts[i - 1]);
                        } else {
                            (0..6).for_each(|i| parsed[i] = expression_parts[i]);
                        }
                    } else if expression_parts.len() == 7 {
                        (0..=6).for_each(|i| parsed[i] = expression_parts[i]);
                    } else {
                        let result2 = Err(ParseException {
                            s: expression,
                            error_offset: 7,
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

                (0..expression_parts.len()).for_each(|i| {
                    normalised[i] = expression_parts[i].to_string();
                });

                normalised[3] = normalised[3].replace("?", "*");
                normalised[5] = normalised[5].replace("?", "*");
                
                (0..=2).for_each(|i| normalised[i] = 
                    if normalised[i].starts_with("0/") { 
                        normalised[i].replace("0/", "*/")
                    } else {
                        normalised[i].to_string()
                    }
                );

                (3..=5).for_each(|i| normalised[i] = 
                    if normalised[i].starts_with("1/") { 
                        normalised[i].replace("1/", "*/")
                    } else {
                        normalised[i].to_string()
                    }
                );

                fn is_numeric(s: &str) -> bool {
                    for c in s.chars() {
                        if !c.is_numeric() {
                            return false;
                        }
                    }
                    return true;
                }

                for i in 0..normalised.len() {
                    if normalised[i] == "*/1" {
                        normalised[i] = "*".to_string();
                    }
                }
                // println!("normalised after replacing */1: {:?}", normalised);
                // convert SUN-SAT format to 0-6 format
                if !is_numeric(&normalised[5]) {
                    for i in 0..=6 {
                        normalised[5] = normalised[5].replace(DAYS_OF_WEEK_ARR[i], i.to_string().as_str());
                    }
                }

                // convert JAN-DEC format to 1-12 format
                if !is_numeric(&normalised[4]) {
                    for i in 1..12 {
                        normalised[4] = normalised[4].replace(MONTHS_ARR[i-1], i.to_string().as_str());
                    }
                }

                // convert 0 second to (empty)
                if "0" == normalised[0] {
                    normalised[0] = "".to_string();
                }

                // convert 0 DOW to 7 so that 0 for Sunday in zeroBasedDayOfWeek is valid
                // this logic is copied from the Java version and seems different than the C#
                // version.
                if options.zero_based_day_of_week && "0" == normalised[5] {
                    normalised[5] = "7".to_string();
                }

                // println!("normalised: {:?}", normalised);
                // Bunch of logic in the C# version is missing from the Java version,
                // such as regex handling of the DOW, stepping and between ranges.
                normalised
            }
        }

        pub fn get_description(description_type: DescriptionTypeEnum,
                               expression: String,
                               options: &Options,
                               locale: String) -> String {
            rust_i18n::set_locale(&locale);
            let expression_parts = expression_parser::parse(expression, options).unwrap();

            let description_res = match description_type {
                DescriptionTypeEnum::FULL => get_full_description(&expression_parts, options),
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

        // Frem the C# code, not Java.
        fn get_full_description(expression_parts: &Vec<String>, options: &Options) -> String {
            let time_segment = get_time_of_day_description(&expression_parts, options);
            let day_of_month_desc = get_day_of_month_description(&expression_parts, options);
            let month_desc = get_month_description(&expression_parts, options);
            let day_of_week_desc = get_day_of_week_description(&expression_parts, options);
            let year_desc = get_year_description(&expression_parts, options);
            let week_or_month_desc = if "*" == &expression_parts[3] {
                day_of_week_desc
            } else {
                day_of_month_desc
            };
            let desc1 = format!("{0}{1}{2}{3}",
                                time_segment,
                                week_or_month_desc,
                                month_desc,
                                year_desc);
            // eprintln!("time: \"{}\"; day_of_month: \"{}\"; month: \"{}\"; year: \"{}\"",
            //           time_segment, week_or_month_desc, month_desc, year_desc);
            // println!("before verbosity: {}", desc1);
            let desc2 = transform_verbosity(desc1, options);
            transform_case(&desc2, options)
        }


        fn transform_verbosity(description: String, options: &Options) -> String {
            let mut desc_temp = description.clone();
            if !options.verbose {
                desc_temp = desc_temp.replace(&t!("messages.every_1_minute"), &t!("messages.every_minute"));
                desc_temp = desc_temp.replace(&t!("messages.every_1_hour"), &t!("messages.every_hour"));
                desc_temp = desc_temp.replace(&t!("messages.every_1_day"), &t!("messages.every_day"));
                desc_temp = desc_temp.replace(&format!(", {}", &t!("messages.every_minute")), "");
                desc_temp = desc_temp.replace(&format!(", {}", &t!("messages.every_hour")), "");
                desc_temp = desc_temp.replace(&format!(", {}", &t!("messages.every_day")), "");
                desc_temp = desc_temp.replace(&format!(", {}", &t!("messages.every_year")), "");
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
            let builder = YearDescriptionBuilder { options };
            builder.get_segment_description(&expression_parts[6],
                                            format!(", {}", t!("messages.every_year")))
        }

        fn get_day_of_week_description(expression_parts: &Vec<String>, options: &Options) -> String {
            let builder = DayOfWeekDescriptionBuilder { options };
            // println!("in get_day_of_week_description, expr: {}", &expression_parts[5]);
            builder.get_segment_description(&expression_parts[5],
                                            format!(", {}", t!("messages.every_day")))
        }

        fn get_minutes_description(expression_parts: &Vec<String>, options: &Options) -> String {
            let builder = MinutesDescriptionBuilder { options };
            builder.get_segment_description(&expression_parts[1],
                                            t!("messages.every_minute"))
        }

        fn get_seconds_description(expression_parts: &Vec<String>, options: &Options) -> String {
            let builder = SecondsDescriptionBuilder { options };
            builder.get_segment_description(&expression_parts[0],
                                            t!("messages.every_second"))
        }

        fn get_hours_description(expression_parts: &Vec<String>, options: &Options) -> String {
            let builder = HoursDescriptionBuilder { options };
            builder.get_segment_description(&expression_parts[2],
                                            t!("messages.every_hour"))
        }

        fn get_month_description(expression_parts: &Vec<String>, options: &Options) -> String {
            let builder = MonthDescriptionBuilder { options };
            builder.get_segment_description(&expression_parts[4], "".to_string())
        }

        fn get_day_of_month_description(expression_parts: &Vec<String>, options: &Options) -> String {
            use regex::Regex;
            use strfmt::strfmt;
            let exp = expression_parts[3].replace("?", "*");
            let description = if "L" == exp {
                format!(", {}", t!("messages.on_the_last_day_of_the_month"))
            } else if "WL" == exp || "LW" == exp {
                format!(", {}", t!("messages.on_the_last_weekday_of_the_month"))
            } else {
                lazy_static! {
                    static ref DOM_RE: Regex = Regex::new(r"(\dW)|(W\d)").unwrap();
                }
                if DOM_RE.is_match(&exp) {
                    let capt = DOM_RE.captures_iter(&exp).next().unwrap();
                    let no_w = capt[0].replace("W", "");
                    let day_number = no_w.parse::<u8>().unwrap();
                    let day_string = if day_number == 1 {
                        t!("messages.first_weekday")
                    } else {
                        t!("messages.weekday_nearest_day", 0 = &no_w)
                    };
                    let fmt_str = format!(", {}", t!("messages.on_the_of_the_month"));
                    let mut vars = HashMap::new();
                    vars.insert("0".to_string(), day_string);
                    strfmt(&fmt_str, &vars).unwrap()
                } else {
                    let builder = DayOfMonthDescriptionBuilder { options };
                    // eprintln!("in get_day_of_month_description, exp: {}", exp);
                    builder.get_segment_description(&exp,
                                                    format!(", {}", t!("messages.every_day")))
                }
            };
            description
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
            } else if minutes_expression.contains("-") 
                    && !minutes_expression.contains("/") 
                    && string_utils::not_contains_any(hours_expression, &SPECIAL_CHARACTERS) {
                let mut minute_parts = minutes_expression.split("-");
                let msg0 = format_time(hours_expression,
                                       &minute_parts.next().unwrap().to_string(),
                                       options);
                let msg1 = format_time(hours_expression,
                                       &minute_parts.next().unwrap().to_string(),
                                       options);
                description.append(t!("messages.every_minute_between",0 = &msg0, 1 = &msg1));
            } else if hours_expression.contains(",") 
                && string_utils::not_contains_any(minutes_expression, &SPECIAL_CHARACTERS) {
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
                // println!("file: {}, line: {}", file!(), line!());
                // println!("seconds_description: {} minutes_description: {}, hours_description: {}",
                //   seconds_description, minutes_description, hours_description);
                description.append(seconds_description);
                if description.len() > 0 && ! minutes_description.is_empty() {
                    description.append(", ");
                }
                description.append(minutes_description);
                if description.len() > 0 && ! hours_description.is_empty()  {
                    description.append(", ");
                }
                description.append(hours_description);

            }
            description.string().unwrap()
        }

        pub fn get_description_cron(expression: String) -> String {
            // println!("Expression: {}", expression);
            get_description(DescriptionTypeEnum::FULL, expression,
                            &Options::options(), rust_i18n::locale())
        }

        pub fn get_description_cron_options(expression: String, options: &cronparser::Options) -> String {
            get_description(DescriptionTypeEnum::FULL, expression,
                            options, rust_i18n::locale())
        }

        pub fn get_description_cron_locale(expression: String, locale: String) -> String {
            get_description(DescriptionTypeEnum::FULL, expression,
                            &Options::options(), locale)
        }

        pub fn get_description_cron_options_locale(expression: String,
                                                   options: &Options,
                                                   locale: String) -> String {
            get_description(DescriptionTypeEnum::FULL, expression,
                            options, locale)
        }


        pub fn get_description_cron_type_expr(desc_type: DescriptionTypeEnum,
                                              expression: String) -> String {
            get_description(desc_type, expression,
                            &Options::options(), rust_i18n::locale())
        }

        pub fn get_description_cron_type_expr_locale(desc_type: DescriptionTypeEnum,
                                                     expression: String,
                                                     locale: String) -> String {
            get_description(desc_type, expression,
                            &Options::options(), locale)
        }

        pub fn get_description_cron_type_expr_opts(desc_type: DescriptionTypeEnum,
                                                   expression: String,
                                                   options: &Options) -> String {
            get_description(desc_type, expression,
                            options, rust_i18n::locale())
        }
    }
}

