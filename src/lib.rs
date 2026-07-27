#![forbid(unsafe_code)]

#[macro_use]
extern crate rust_i18n;
extern crate strfmt;

mod description_builder;
pub mod error;

pub use error::{CronError, Result as CronResult};

rust_i18n::i18n!("locales");

mod string_utils {
    pub fn not_contains_any(s: &str, chars: &[char]) -> bool {
        !s.chars().any(|c| chars.contains(&c))
    }

    pub fn is_numeric(s: &str) -> bool {
        !s.is_empty() && s.chars().all(|c| c.is_numeric())
    }
}

mod date_time_utils {
    pub static DAYS_OF_WEEK_ARR: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    pub static MONTHS_ARR: [&str; 12] = [
        "january",
        "february",
        "march",
        "april",
        "may",
        "june",
        "july",
        "august",
        "september",
        "october",
        "november",
        "december",
    ];

    use crate::cronparser::Options;

    pub fn format_time(hours_expression: &str, minutes_expression: &str, opts: &Options) -> String {
        format_time_secs(hours_expression, minutes_expression, "", opts)
    }

    pub fn format_time_secs(
        hours_expression: &str,
        minutes_expression: &str,
        seconds_expression: &str,
        opts: &Options,
    ) -> String {
        let mut hour: i8 = hours_expression.parse().unwrap_or(0);
        let mut period = String::new();

        if !opts.twenty_four_hour_time {
            period = if hour >= 12 {
                t!("time_pm")
            } else {
                t!("time_am")
            };
            if !period.is_empty() {
                period = format!(" {}", period);
            }
            if hour > 12 {
                hour -= 12;
            }
            if hour == 0 {
                hour = 12;
            }
        }

        let minutes_val = minutes_expression.parse::<i8>().unwrap_or(0);
        let minutes = minutes_val.to_string();
        let mut seconds = String::new();

        let minutes_num = minutes.parse::<i8>().unwrap_or(0);
        if !seconds_expression.is_empty() {
            let secs_val = seconds_expression.parse::<i8>().unwrap_or(0);
            seconds = format!(":{:02}", secs_val);
        }
        let formatted_hours = if opts.twenty_four_hour_time {
            format!("{:02}", hour)
        } else {
            format!("{}", hour)
        };
        format!(
            "{}:{:02}{}{}",
            formatted_hours, minutes_num, seconds, period
        )
    }

    pub fn get_day_of_week_name(day_of_week: usize) -> String {
        let day_str = DAYS_OF_WEEK_ARR[day_of_week % 7];
        t!(day_str)
    }
}

pub fn format_minutes(minutes_expression: &str) -> String {
    if minutes_expression.contains(',') {
        let mparts = minutes_expression.split(',');
        let mut formatted_expression = String::new();
        for mpt in mparts {
            if let Ok(val) = mpt.parse::<i8>() {
                formatted_expression.push_str(&format!("{:02},", val));
            }
        }
        formatted_expression
    } else if let Ok(val) = minutes_expression.parse::<i8>() {
        format!("{:02}", val)
    } else {
        minutes_expression.to_string()
    }
}

pub mod cronparser {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CasingTypeEnum {
        Title,
        Sentence,
        LowerCase,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    #[derive(Debug, Clone)]
    pub struct Options {
        pub throw_exception_on_parse_error: bool,
        pub casing_type: CasingTypeEnum,
        pub verbose: bool,
        pub zero_based_day_of_week: bool,
        pub twenty_four_hour_time: bool,
        pub need_space_between_words: bool,
    }

    impl Default for Options {
        fn default() -> Self {
            Self::options()
        }
    }

    impl Options {
        #[allow(clippy::self_named_constructors)]
        pub fn options() -> Options {
            Options {
                throw_exception_on_parse_error: true,
                casing_type: CasingTypeEnum::Sentence,
                verbose: false,
                zero_based_day_of_week: true,
                twenty_four_hour_time: false,
                need_space_between_words: true,
            }
        }

        pub fn twenty_four_hour() -> Options {
            Options {
                twenty_four_hour_time: true,
                ..Options::options()
            }
        }
    }

    pub mod cron_expression_descriptor {
        use lazy_static::lazy_static;
        use std::collections::HashMap;

        use crate::cronparser::{CasingTypeEnum, DescriptionTypeEnum, Options};
        use crate::date_time_utils::{format_time, format_time_secs};
        use crate::description_builder::DescriptionBuilder;
        use crate::description_builder::{
            DayOfMonthDescriptionBuilder, DayOfWeekDescriptionBuilder, HoursDescriptionBuilder,
            MinutesDescriptionBuilder, MonthDescriptionBuilder, SecondsDescriptionBuilder,
            YearDescriptionBuilder,
        };
        use crate::string_utils;

        const SPECIAL_CHARACTERS: [char; 4] = ['/', '-', ',', '*'];

        #[derive(Debug, PartialEq, Eq, Clone)]
        pub struct ParseException {
            pub s: String,
            pub error_offset: u8,
        }

        mod expression_parser {
            use crate::cronparser::cron_expression_descriptor::ParseException;
            use crate::cronparser::Options;
            use lazy_static::lazy_static;
            use regex::Regex;

            pub fn parse(
                expression: &str,
                options: &Options,
            ) -> Result<Vec<String>, ParseException> {
                let mut parsed: Vec<&str> = vec![""; 7];
                if expression.trim().is_empty() {
                    Err(ParseException {
                        s: expression.to_string(),
                        error_offset: 0,
                    })
                } else {
                    let expression_parts: Vec<&str> = expression.split_whitespace().collect();
                    if expression_parts.len() < 5 {
                        return Err(ParseException {
                            s: expression.to_string(),
                            error_offset: 0,
                        });
                    } else if expression_parts.len() == 5 {
                        parsed[0] = "";
                        (1..=5).for_each(|i| parsed[i] = expression_parts[i - 1]);
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
                        return Err(ParseException {
                            s: expression.to_string(),
                            error_offset: 7,
                        });
                    }

                    let normalized_expr = normalise_expression(parsed, options);
                    Ok(normalized_expr)
                }
            }

            fn normalise_expression(expression_parts: Vec<&str>, options: &Options) -> Vec<String> {
                static DAYS_OF_WEEK_ARR: [&str; 7] =
                    ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
                static MONTHS_ARR: [&str; 12] = [
                    "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV",
                    "DEC",
                ];
                let mut normalised: Vec<String> = vec![String::new(); 7];

                for i in 0..expression_parts.len() {
                    normalised[i] = expression_parts[i].to_string();
                }

                normalised[3] = normalised[3].replace('?', "*");
                normalised[5] = normalised[5].replace('?', "*");

                for item in normalised.iter_mut().take(3) {
                    if item.starts_with("0/") {
                        *item = item.replace("0/", "*/");
                    }
                }

                for item in normalised.iter_mut().skip(3).take(3) {
                    if item.starts_with("1/") {
                        *item = item.replace("1/", "*/");
                    }
                }

                for item in &mut normalised {
                    if item == "*/1" {
                        *item = "*".to_string();
                    }
                }

                if !crate::string_utils::is_numeric(&normalised[5]) {
                    for (i, &day) in DAYS_OF_WEEK_ARR.iter().enumerate() {
                        normalised[5] = normalised[5].replace(day, &i.to_string());
                    }
                }

                if !crate::string_utils::is_numeric(&normalised[4]) {
                    for (i, &month) in MONTHS_ARR.iter().enumerate() {
                        normalised[4] = normalised[4].replace(month, &(i + 1).to_string());
                    }
                }

                if normalised[0] == "0" {
                    normalised[0] = String::new();
                }

                if options.zero_based_day_of_week && normalised[5] == "0" {
                    normalised[5] = "7".to_string();
                }

                normalised
            }
        }

        pub fn get_description(
            description_type: DescriptionTypeEnum,
            expression: &str,
            options: &Options,
            locale: &str,
        ) -> Result<String, ParseException> {
            rust_i18n::set_locale(locale);
            let expression_parsed = expression_parser::parse(expression, options);
            match expression_parsed {
                Ok(expression_parts) => {
                    let description_res = match description_type {
                        DescriptionTypeEnum::FULL => {
                            get_full_description(&expression_parts, options)
                        }
                        DescriptionTypeEnum::TIMEOFDAY => {
                            get_time_of_day_description(&expression_parts, options)
                        }
                        DescriptionTypeEnum::SECONDS => {
                            get_seconds_description(&expression_parts, options)
                        }
                        DescriptionTypeEnum::MINUTES => {
                            get_minutes_description(&expression_parts, options)
                        }
                        DescriptionTypeEnum::HOURS => {
                            get_hours_description(&expression_parts, options)
                        }
                        DescriptionTypeEnum::DAYOFWEEK => {
                            get_day_of_week_description(&expression_parts, options)
                        }
                        DescriptionTypeEnum::MONTH => {
                            get_month_description(&expression_parts, options)
                        }
                        DescriptionTypeEnum::DAYOFMONTH => {
                            get_day_of_month_description(&expression_parts, options)
                        }
                        DescriptionTypeEnum::YEAR => {
                            get_year_description(&expression_parts, options)
                        }
                    };
                    Ok(description_res)
                }
                Err(pe) => Err(pe),
            }
        }

        fn get_full_description(expression_parts: &[String], options: &Options) -> String {
            let time_segment = get_time_of_day_description(expression_parts, options);
            let day_of_month_desc = get_day_of_month_description(expression_parts, options);
            let month_desc = get_month_description(expression_parts, options);
            let day_of_week_desc = get_day_of_week_description(expression_parts, options);
            let year_desc = get_year_description(expression_parts, options);
            let week_or_month_desc = if expression_parts.get(3).is_some_and(|s| s == "*") {
                day_of_week_desc
            } else {
                day_of_month_desc
            };
            let desc1 = format!(
                "{}{}{}{}",
                time_segment, week_or_month_desc, month_desc, year_desc
            );
            let desc2 = transform_verbosity(&desc1, options);
            transform_case(&desc2, options)
        }

        fn transform_verbosity(description: &str, options: &Options) -> String {
            let mut desc_temp = description.to_string();
            if !options.verbose {
                desc_temp =
                    desc_temp.replace(&t!("messages.every_1_minute"), &t!("messages.every_minute"));
                desc_temp =
                    desc_temp.replace(&t!("messages.every_1_hour"), &t!("messages.every_hour"));
                desc_temp =
                    desc_temp.replace(&t!("messages.every_1_day"), &t!("messages.every_day"));
                desc_temp = desc_temp.replace(&format!(", {}", t!("messages.every_minute")), "");
                desc_temp = desc_temp.replace(&format!(", {}", t!("messages.every_hour")), "");
                desc_temp = desc_temp.replace(&format!(", {}", t!("messages.every_day")), "");
                desc_temp = desc_temp.replace(&format!(", {}", t!("messages.every_year")), "");
            }
            desc_temp
        }

        fn transform_case(description: &str, options: &Options) -> String {
            let mut chars = description.chars();
            match &options.casing_type {
                CasingTypeEnum::Sentence | CasingTypeEnum::Title => match chars.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
                },
                CasingTypeEnum::LowerCase => description.to_lowercase(),
            }
        }

        fn get_year_description(expression_parts: &[String], options: &Options) -> String {
            let builder = YearDescriptionBuilder { options };
            let year_part = expression_parts.get(6).map_or("", |s| s.as_str());
            builder.get_segment_description(year_part, format!(", {}", t!("messages.every_year")))
        }

        fn get_day_of_week_description(expression_parts: &[String], options: &Options) -> String {
            let builder = DayOfWeekDescriptionBuilder { options };
            let dow_part = expression_parts.get(5).map_or("", |s| s.as_str());
            builder.get_segment_description(dow_part, format!(", {}", t!("messages.every_day")))
        }

        fn get_minutes_description(expression_parts: &[String], options: &Options) -> String {
            let builder = MinutesDescriptionBuilder { options };
            let min_part = expression_parts.get(1).map_or("", |s| s.as_str());
            builder.get_segment_description(min_part, t!("messages.every_minute"))
        }

        fn get_seconds_description(expression_parts: &[String], options: &Options) -> String {
            let builder = SecondsDescriptionBuilder { options };
            let sec_part = expression_parts.first().map_or("", |s| s.as_str());
            builder.get_segment_description(sec_part, t!("messages.every_second"))
        }

        fn get_hours_description(expression_parts: &[String], options: &Options) -> String {
            let builder = HoursDescriptionBuilder { options };
            let hour_part = expression_parts.get(2).map_or("", |s| s.as_str());
            builder.get_segment_description(hour_part, t!("messages.every_hour"))
        }

        fn get_month_description(expression_parts: &[String], options: &Options) -> String {
            let builder = MonthDescriptionBuilder { options };
            let month_part = expression_parts.get(4).map_or("", |s| s.as_str());
            builder.get_segment_description(month_part, String::new())
        }

        fn get_day_of_month_description(expression_parts: &[String], options: &Options) -> String {
            use regex::Regex;
            use strfmt::strfmt;
            let raw_dom = expression_parts.get(3).map_or("", |s| s.as_str());
            let exp = raw_dom.replace('?', "*");
            if exp == "L" {
                format!(", {}", t!("messages.on_the_last_day_of_the_month"))
            } else if exp == "WL" || exp == "LW" {
                format!(", {}", t!("messages.on_the_last_weekday_of_the_month"))
            } else {
                lazy_static! {
                    static ref DOM_RE: Regex = Regex::new(r"(\dW)|(W\d)").unwrap();
                }
                if DOM_RE.is_match(&exp) {
                    if let Some(capt) = DOM_RE.captures_iter(&exp).next() {
                        let no_w = capt[0].replace('W', "");
                        let day_number = no_w.parse::<u8>().unwrap_or(0);
                        let day_string = if day_number == 1 {
                            t!("messages.first_weekday")
                        } else {
                            t!("messages.weekday_nearest_day", 0 = &no_w)
                        };
                        let fmt_str = format!(", {}", t!("messages.on_the_of_the_month"));
                        let mut vars = HashMap::new();
                        vars.insert("0".to_string(), day_string);
                        strfmt(&fmt_str, &vars).unwrap_or_default()
                    } else {
                        String::new()
                    }
                } else {
                    let builder = DayOfMonthDescriptionBuilder { options };
                    builder.get_segment_description(&exp, format!(", {}", t!("messages.every_day")))
                }
            }
        }

        fn get_time_of_day_description(expression_parts: &[String], options: &Options) -> String {
            let seconds_expression = expression_parts.first().map_or("", |s| s.as_str());
            let minutes_expression = expression_parts.get(1).map_or("", |s| s.as_str());
            let hours_expression = expression_parts.get(2).map_or("", |s| s.as_str());

            let mut description = String::new();

            if minutes_expression
                .chars()
                .all(|c| !SPECIAL_CHARACTERS.contains(&c))
                && hours_expression
                    .chars()
                    .all(|c| !SPECIAL_CHARACTERS.contains(&c))
                && seconds_expression
                    .chars()
                    .all(|c| !SPECIAL_CHARACTERS.contains(&c))
            {
                description.push_str(&t!("at"));
                if options.need_space_between_words {
                    description.push(' ');
                }
                description.push_str(&format_time_secs(
                    hours_expression,
                    minutes_expression,
                    seconds_expression,
                    options,
                ));
            } else if minutes_expression.contains('-')
                && !minutes_expression.contains('/')
                && string_utils::not_contains_any(hours_expression, &SPECIAL_CHARACTERS)
            {
                let mut minute_parts = minutes_expression.split('-');
                let m0 = minute_parts.next().unwrap_or("");
                let m1 = minute_parts.next().unwrap_or("");
                let msg0 = format_time(hours_expression, m0, options);
                let msg1 = format_time(hours_expression, m1, options);
                description.push_str(&t!("messages.every_minute_between", 0 = &msg0, 1 = &msg1));
            } else if hours_expression.contains(',')
                && string_utils::not_contains_any(minutes_expression, &SPECIAL_CHARACTERS)
            {
                let hour_parts: Vec<_> = hours_expression.split(',').collect();
                let hpsz = hour_parts.len();
                description.push_str(&t!("at"));

                for (i, hp) in hour_parts.iter().enumerate() {
                    description.push(' ');
                    description.push_str(&format_time(hp, minutes_expression, options));
                    if i < hpsz - 2 {
                        description.push(',');
                    }
                    if i == hpsz - 2 {
                        description.push(' ');
                        description.push_str(&t!("and"));
                    }
                }
            } else {
                let seconds_description = get_seconds_description(expression_parts, options);
                let minutes_description = get_minutes_description(expression_parts, options);
                let hours_description = get_hours_description(expression_parts, options);

                description.push_str(&seconds_description);
                if !description.is_empty() && !minutes_description.is_empty() {
                    description.push_str(", ");
                }
                description.push_str(&minutes_description);
                if !description.is_empty() && !hours_description.is_empty() {
                    description.push_str(", ");
                }
                description.push_str(&hours_description);
            }
            description
        }

        pub fn get_description_cron(expression: &str) -> Result<String, ParseException> {
            get_description(
                DescriptionTypeEnum::FULL,
                expression,
                &Options::options(),
                &rust_i18n::locale(),
            )
        }

        pub fn get_description_cron_options(
            expression: &str,
            options: &Options,
        ) -> Result<String, ParseException> {
            get_description(
                DescriptionTypeEnum::FULL,
                expression,
                options,
                &rust_i18n::locale(),
            )
        }

        pub fn get_description_cron_locale(
            expression: &str,
            locale: &str,
        ) -> Result<String, ParseException> {
            get_description(
                DescriptionTypeEnum::FULL,
                expression,
                &Options::options(),
                locale,
            )
        }

        pub fn get_description_cron_options_locale(
            expression: &str,
            options: &Options,
            locale: &str,
        ) -> Result<String, ParseException> {
            get_description(DescriptionTypeEnum::FULL, expression, options, locale)
        }

        pub fn get_description_cron_type_expr(
            desc_type: DescriptionTypeEnum,
            expression: &str,
        ) -> Result<String, ParseException> {
            get_description(
                desc_type,
                expression,
                &Options::options(),
                &rust_i18n::locale(),
            )
        }

        pub fn get_description_cron_type_expr_locale(
            desc_type: DescriptionTypeEnum,
            expression: &str,
            locale: &str,
        ) -> Result<String, ParseException> {
            get_description(desc_type, expression, &Options::options(), locale)
        }

        pub fn get_description_cron_type_expr_opts(
            desc_type: DescriptionTypeEnum,
            expression: &str,
            options: &Options,
        ) -> Result<String, ParseException> {
            get_description(desc_type, expression, options, &rust_i18n::locale())
        }
    }
}
