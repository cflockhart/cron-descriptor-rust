use crate::cronparser::Options;
use crate::date_time_utils;
use crate::date_time_utils::{format_time, MONTHS_ARR};
use crate::{format_minutes, string_utils};
use lazy_static::lazy_static;
use std::collections::HashMap;
use strfmt::strfmt;

i18n!("locales");

const SPECIAL_CHARACTERS_MINUS_STAR: [char; 3] = ['/', '-', ','];

pub trait DescriptionBuilder<'a> {
    fn get_segment_description(&self, expression: &str, all_description: String) -> String {
        if expression.is_empty() {
            String::new()
        } else if expression == "*" {
            all_description
        } else if string_utils::not_contains_any(expression, &SPECIAL_CHARACTERS_MINUS_STAR) {
            let gdf = self.get_description_format(expression);
            let sid = self.get_single_item_description(expression);
            let mut vars = HashMap::new();
            vars.insert("0".to_string(), sid);
            strfmt(&gdf, &vars).unwrap_or_default()
        } else if expression.contains('/') {
            let segments: Vec<&str> = expression.split('/').collect();
            let step = segments.get(1).unwrap_or(&"");
            let gidf = self.get_interval_description_format(step);
            let gsid = self.get_single_item_description(step);
            let mut vars = HashMap::new();
            vars.insert("0".to_string(), gsid);
            let tmpstr = strfmt(&gidf, &vars).unwrap_or_default();
            if let Some(range_part) = segments.first() {
                if range_part.contains('-') {
                    let between_segments: Vec<&str> = range_part.split('-').collect();
                    let gbdf = self.get_between_description_format(false);
                    let sid0 =
                        self.get_single_item_description(between_segments.first().unwrap_or(&""));
                    let sid1 =
                        self.get_single_item_description(between_segments.get(1).unwrap_or(&""));
                    let mut vars = HashMap::new();
                    vars.insert("0".to_string(), sid0);
                    vars.insert("1".to_string(), sid1);
                    format!("{}, {}", tmpstr, strfmt(&gbdf, &vars).unwrap_or_default())
                } else {
                    tmpstr
                }
            } else {
                tmpstr
            }
        } else if expression.contains(',') {
            let segments: Vec<&str> = expression.split(',').collect();
            let mut description_content = String::new();
            for (i, segment) in segments.iter().enumerate() {
                if i > 0 && segments.len() > 2 && i < segments.len() - 1 {
                    description_content.push_str(", ");
                }
                if i > 0 && segments.len() > 1 && (i == segments.len() - 1 || segments.len() == 2) {
                    if self.need_space_between_words() {
                        description_content.push(' ');
                    }
                    description_content.push_str(&t!("and"));
                    if self.need_space_between_words() {
                        description_content.push(' ');
                    }
                }
                if segment.contains('-') {
                    let between_segments: Vec<&str> = segment.split('-').collect();
                    let gbdf = self.get_between_description_format(true);
                    let sid0 =
                        self.get_single_item_description(between_segments.first().unwrap_or(&""));
                    let sid1 =
                        self.get_single_item_description(between_segments.get(1).unwrap_or(&""));
                    let mut vars = HashMap::new();
                    vars.insert("0".to_string(), sid0);
                    vars.insert("1".to_string(), sid1);
                    description_content.push_str(&strfmt(&gbdf, &vars).unwrap_or_default());
                } else {
                    description_content.push_str(&self.get_single_item_description(segment));
                }
            }
            let mut vars = HashMap::new();
            vars.insert("0".to_string(), description_content);
            strfmt(&self.get_description_format(expression), &vars).unwrap_or_default()
        } else if expression.contains('-') {
            let segments: Vec<&str> = expression.split('-').collect();
            let gbdf = self.get_between_description_format(false);
            let sid0 = self.get_single_item_description(segments.first().unwrap_or(&""));
            let sid1 = self.get_single_item_description(segments.get(1).unwrap_or(&""));
            let mut vars = HashMap::new();
            vars.insert("0".to_string(), sid0);
            vars.insert("1".to_string(), sid1);
            strfmt(&gbdf, &vars).unwrap_or_default()
        } else {
            String::new()
        }
    }

    fn get_between_description_format(&self, omit_separator: bool) -> String;
    fn get_interval_description_format(&self, expression: &str) -> String;
    fn get_single_item_description(&self, expression: &str) -> String;
    fn get_description_format(&self, expression: &str) -> String;
    fn need_space_between_words(&self) -> bool;

    fn get_space_opt(options: &Options) -> String {
        if options.need_space_between_words {
            " ".to_string()
        } else {
            String::new()
        }
    }

    fn get_space(&self) -> String;

    fn plural<'b>(expression: &str, singular: &'b str, plural: &'b str) -> &'b str {
        let parsed_expr = expression.parse::<i8>();
        if parsed_expr.is_ok_and(|val| val > 1) || expression.contains(',') {
            plural
        } else {
            singular
        }
    }

    fn min_plural(expression: &str) -> String {
        lazy_static! {
            static ref MINUTE: String = t!("minute");
            static ref MINUTES: String = t!("minutes");
        }
        Self::plural(expression, &MINUTE, &MINUTES).to_string()
    }
}

pub struct DayOfMonthDescriptionBuilder<'a> {
    pub options: &'a Options,
}

pub struct DayOfWeekDescriptionBuilder<'a> {
    pub options: &'a Options,
}

pub struct HoursDescriptionBuilder<'a> {
    pub options: &'a Options,
}

pub struct MinutesDescriptionBuilder<'a> {
    pub options: &'a Options,
}

pub struct MonthDescriptionBuilder<'a> {
    pub options: &'a Options,
}

pub struct SecondsDescriptionBuilder<'a> {
    pub options: &'a Options,
}

pub struct YearDescriptionBuilder<'a> {
    pub options: &'a Options,
}

impl DescriptionBuilder<'_> for DayOfMonthDescriptionBuilder<'_> {
    fn get_between_description_format(&self, omit_separator: bool) -> String {
        let format = t!("messages.between_days_of_the_month");
        if omit_separator {
            format
        } else {
            format!(", {}", format)
        }
    }

    fn get_interval_description_format(&self, expression: &str) -> String {
        format!(
            ", {}{}{}",
            t!("every_x"),
            self.get_space(),
            Self::plural(expression, &t!("day"), &t!("days"))
        )
    }

    fn get_single_item_description(&self, expression: &str) -> String {
        expression.to_string()
    }

    fn get_description_format(&self, _: &str) -> String {
        format!(", {}", t!("messages.on_day_of_month"))
    }

    fn need_space_between_words(&self) -> bool {
        self.options.need_space_between_words
    }

    fn get_space(&self) -> String {
        Self::get_space_opt(self.options)
    }
}

impl DescriptionBuilder<'_> for DayOfWeekDescriptionBuilder<'_> {
    fn get_between_description_format(&self, omit_separator: bool) -> String {
        let format = t!("messages.between_weekday_description_format");
        if omit_separator {
            format
        } else {
            format!(", {}", format)
        }
    }

    fn get_interval_description_format(&self, expression: &str) -> String {
        format!(
            ", {}",
            t!("messages.interval_description_format", 0 = expression)
        )
    }

    fn get_single_item_description(&self, expression: &str) -> String {
        let exp = match expression.find('#') {
            Some(ind) => expression[..ind].to_string(),
            None => match expression.find('L') {
                Some(_) => expression.replace('L', ""),
                None => expression.to_string(),
            },
        };

        if string_utils::is_numeric(&exp) {
            let mut day_of_week_num = exp.parse::<u8>().unwrap_or(0);
            let is_invalid_day_of_week_for_setting =
                !self.options.zero_based_day_of_week && day_of_week_num <= 1;
            if is_invalid_day_of_week_for_setting
                || (self.options.zero_based_day_of_week && day_of_week_num == 0)
            {
                return date_time_utils::get_day_of_week_name(7);
            } else if !self.options.zero_based_day_of_week && day_of_week_num > 0 {
                day_of_week_num -= 1;
            }
            date_time_utils::get_day_of_week_name(day_of_week_num as usize)
        } else {
            let lowered = exp.to_lowercase();
            let mut chars = lowered.chars();
            let capitalized = match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            };
            t!(&capitalized)
        }
    }

    fn get_description_format(&self, expression: &str) -> String {
        if expression.contains('#') {
            let hash_ind = expression.find('#').map_or(0, |idx| idx + 1);
            let day_of_week_of_month_number = &expression[hash_ind..];
            let day_of_week_month_description = match day_of_week_of_month_number {
                "1" => t!("first"),
                "2" => t!("second"),
                "3" => t!("third"),
                "4" => t!("fourth"),
                "5" => t!("fifth"),
                _ => String::new(),
            };
            let i18_str = t!("messages.on_the_day_of_the_month");
            let msg = strfmt!(&i18_str, nth => day_of_week_month_description,
                           day_of_week => "{0}");
            format!(", {}", msg.unwrap_or_default())
        } else if expression.contains('L') {
            format!(", {}", t!("messages.on_the_last_of_the_month"))
        } else {
            format!(", {}", t!("messages.only_on"))
        }
    }

    fn need_space_between_words(&self) -> bool {
        self.options.need_space_between_words
    }

    fn get_space(&self) -> String {
        Self::get_space_opt(self.options)
    }
}

impl DescriptionBuilder<'_> for HoursDescriptionBuilder<'_> {
    fn get_between_description_format(&self, _: bool) -> String {
        t!("messages.between_x_and_y")
    }

    fn get_interval_description_format(&self, expression: &str) -> String {
        let gdf = format!(
            "{}{}{}",
            t!("messages.every_x"),
            self.get_space(),
            Self::plural(expression, &t!("hour"), &t!("hours"))
        );
        let mut vars = HashMap::new();
        vars.insert("0".to_string(), expression.to_string());
        strfmt(&gdf, &vars).unwrap_or_default()
    }

    fn get_single_item_description(&self, expression: &str) -> String {
        format_time(expression, "0", self.options)
    }

    fn get_description_format(&self, _: &str) -> String {
        t!("messages.at_x")
    }

    fn need_space_between_words(&self) -> bool {
        self.options.need_space_between_words
    }

    fn get_space(&self) -> String {
        Self::get_space_opt(self.options)
    }
}

impl DescriptionBuilder<'_> for MinutesDescriptionBuilder<'_> {
    fn get_between_description_format(&self, _: bool) -> String {
        t!("messages.minutes_through_past_the_hour")
    }

    fn get_interval_description_format(&self, expression: &str) -> String {
        let gdf = format!(
            "{}{}{}",
            t!("messages.every_x"),
            self.get_space(),
            Self::min_plural(expression)
        );
        let mut vars = HashMap::new();
        vars.insert("0".to_string(), expression.to_string());
        strfmt(&gdf, &vars).unwrap_or_default()
    }

    fn get_single_item_description(&self, expression: &str) -> String {
        format_minutes(expression)
    }

    fn get_description_format(&self, expression: &str) -> String {
        if expression == "0" {
            String::new()
        } else {
            format!(
                "{}{}{}{}{}",
                t!("messages.at_x"),
                self.get_space(),
                Self::min_plural(expression),
                self.get_space(),
                t!("messages.past_the_hour")
            )
        }
    }

    fn need_space_between_words(&self) -> bool {
        self.options.need_space_between_words
    }

    fn get_space(&self) -> String {
        Self::get_space_opt(self.options)
    }
}

impl DescriptionBuilder<'_> for MonthDescriptionBuilder<'_> {
    fn get_between_description_format(&self, omit_separator: bool) -> String {
        let format = t!("messages.between_description_format");
        if omit_separator {
            format
        } else {
            format!(", {}", format)
        }
    }

    fn get_interval_description_format(&self, expression: &str) -> String {
        let month_str = t!("month");
        let months_str = t!("months");
        let plural_str = Self::plural(expression, &month_str, &months_str);
        let gdf = format!(
            ", {}{}{}",
            t!("messages.every_x"),
            self.get_space(),
            plural_str
        );

        let mut vars = HashMap::new();
        vars.insert("0".to_string(), expression.to_string());
        strfmt(&gdf, &vars).unwrap_or_default()
    }

    fn get_single_item_description(&self, expression: &str) -> String {
        if let Ok(month_num) = expression.parse::<usize>() {
            if (1..=12).contains(&month_num) {
                let month_key = MONTHS_ARR[month_num - 1];
                return t!(month_key);
            }
        }
        expression.to_string()
    }

    fn get_description_format(&self, _: &str) -> String {
        format!(", {}", t!("messages.only_in_month"))
    }

    fn need_space_between_words(&self) -> bool {
        self.options.need_space_between_words
    }

    fn get_space(&self) -> String {
        Self::get_space_opt(self.options)
    }
}

impl DescriptionBuilder<'_> for SecondsDescriptionBuilder<'_> {
    fn get_between_description_format(&self, _: bool) -> String {
        t!("messages.seconds_through_past_the_minute")
    }

    fn get_interval_description_format(&self, _: &str) -> String {
        t!("messages.every_x_seconds")
    }

    fn get_single_item_description(&self, expression: &str) -> String {
        expression.to_string()
    }

    fn get_description_format(&self, _: &str) -> String {
        t!("messages.at_x_seconds_past_the_minute")
    }

    fn need_space_between_words(&self) -> bool {
        self.options.need_space_between_words
    }

    fn get_space(&self) -> String {
        Self::get_space_opt(self.options)
    }
}

impl DescriptionBuilder<'_> for YearDescriptionBuilder<'_> {
    fn get_between_description_format(&self, omit_separator: bool) -> String {
        let format = t!("messages.between_description_format");
        if omit_separator {
            format
        } else {
            format!(", {}", format)
        }
    }

    fn get_interval_description_format(&self, expression: &str) -> String {
        let year_str = t!("year");
        let years_str = t!("years");
        let plural_str = Self::plural(expression, &year_str, &years_str);
        let gdf = format!(
            ", {}{}{}",
            t!("messages.every_x"),
            self.get_space(),
            plural_str
        );
        let mut vars = HashMap::new();
        vars.insert("0".to_string(), expression.to_string());
        strfmt(&gdf, &vars).unwrap_or_default()
    }

    fn get_single_item_description(&self, expression: &str) -> String {
        expression
            .parse::<u16>()
            .map_or_else(|_| expression.to_string(), |y| y.to_string())
    }

    fn get_description_format(&self, _: &str) -> String {
        format!(", {}", t!("messages.only_in_year"))
    }

    fn need_space_between_words(&self) -> bool {
        self.options.need_space_between_words
    }

    fn get_space(&self) -> String {
        Self::get_space_opt(self.options)
    }
}
