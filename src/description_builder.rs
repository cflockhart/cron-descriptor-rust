use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::cronparser::Options;
use crate::{format_minutes, string_utils};
use crate::date_time_utils;
use substring::Substring;


use strfmt::{strfmt, strfmt_builder};
use string_builder::Builder;
use crate::date_time_utils::{format_time, MONTHS_ARR};

i18n!("locales");

const SPECIAL_CHARACTERS_MINUS_STAR: [char; 3] = ['/', '-', ','];

pub trait DescriptionBuilder<'a> {
    fn get_segment_description(&self, expression: &String, all_description: String) -> String {
        let description = if expression.is_empty() {
            "".to_string()
        } else if expression == "*" {
            all_description
        } else if string_utils::not_contains_any(expression, &SPECIAL_CHARACTERS_MINUS_STAR) {
            let gdf = self.get_description_format(expression);
            let sid = self.get_single_item_description(expression);
            let mut vars = HashMap::new();
            vars.insert("0".to_string(), sid);
            strfmt(&gdf, &vars).unwrap()
        } else if expression.contains("/") {
            let segments = expression.split("/").collect::<Vec<_>>();
            let gidf = self.get_interval_description_format(&segments[1].to_string());
            let gsid = self.get_single_item_description(&segments[1].to_string());
            let gsid2 = gsid.clone();
            let mut vars = HashMap::new();
            vars.insert("0".to_string(), gsid);
            let tmpstr = strfmt(&gidf, &vars).unwrap();
            if segments[0].contains("-") {
                let between_segments_of_interval = segments[0].to_string();
                let between_segments = between_segments_of_interval.split("-").collect::<Vec<_>>();
                let gbdf = self.get_between_description_format(&between_segments_of_interval, false);
                let sid0 = self.get_single_item_description(&between_segments[0].to_string());
                let sid1 = self.get_single_item_description(&between_segments[1].to_string());
                let mut vars = HashMap::new();
                vars.insert("0".to_string(), sid0);
                vars.insert("1".to_string(), sid1);
                format!("{}, {}", tmpstr, strfmt(&gbdf, &vars).unwrap())
            } else {
                println!("gidf: {}, gsid: {}", gidf, gsid2);
                tmpstr
            }
        } else if expression.contains(",") {
            let segments = expression.split(",").collect::<Vec<_>>();
            let mut description_content = Builder::default();
            for i in 0..segments.len() {
                if i > 0 && segments.len() > 2 {
                    if i < segments.len() - 1 {
                        description_content.append(", ");
                    }
                }
                if i > 0 && segments.len() > 1 && (i == segments.len() - 1 || segments.len() == 2) {
                    if self.need_space_between_words() {
                        description_content.append(" ");
                    }
                    description_content.append(t!("and"));
                    if self.need_space_between_words() {
                        description_content.append(" ");
                    }
                    if segments[i].contains("-") {
                        let between_segments = segments[i].split("-").collect::<Vec<_>>();
                        let gbdf = self.get_between_description_format(expression, true);
                        let sid0 = self.get_single_item_description(&between_segments[0].to_string());
                        let sid1 = self.get_single_item_description(&between_segments[1].to_string());
                        let mut vars = HashMap::new();
                        vars.insert("0".to_string(), sid0);
                        vars.insert("1".to_string(), sid1);
                        description_content.append(format!(", {}", strfmt(&gbdf, &vars).unwrap()));
                    } else {
                       description_content.append(self.get_single_item_description(&segments[i].to_string()));
                    }
                }
            }
            let mut vars = HashMap::new();
            vars.insert("0".to_string(), description_content.string().unwrap());
            strfmt(&self.get_description_format(expression), &vars).unwrap()
        } else if expression.contains("-") {
            let segments= expression.split("-").collect::<Vec<_>>();
            let gbdf = self.get_between_description_format(expression, false);
            let sid0 = self.get_single_item_description(&segments[0].to_string());
            let sid1 = self.get_single_item_description(&segments[1].to_string());
            let mut vars = HashMap::new();
            vars.insert("0".to_string(), sid0);
            vars.insert("1".to_string(), sid1);
            format!(", {}", strfmt(&gbdf, &vars).unwrap())
        } else {
            "".to_string()
        };
        description
    }

    fn get_between_description_format(&self, expression: &String, omit_separator: bool) -> String;
    fn get_interval_description_format(&self, expression: &String) -> String;
    fn get_single_item_description(&self, expression: &String) -> String;
    fn get_description_format(&self, expression: &String) -> String;
    fn need_space_between_words(&self) -> bool;

    fn get_space_opt(options: &Options) -> String {
        if options.need_space_between_words {
            " ".to_string()
        } else {
            "".to_string()
        }
    }

    fn get_space(&self) -> String;

    fn plural_num(num: i8, singular: &'a String, plural: &'a String) -> &'a String {
        Self::plural(&num.to_string(), singular, plural)
    }


    fn plural(expression: &String, singular: &'a String, plural: &'a String) -> &'a String {
        let parsed_expr = expression.parse::<i8>();
        if parsed_expr.is_ok() && parsed_expr.unwrap() > 1 {
            plural
        } else if expression.contains(",") {
            plural
        } else {
            singular
        }
    }

    fn min_plural(expression: &String) -> String {
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
    fn get_between_description_format(&self, expression: &String, omit_separator: bool) -> String {
        let format = t!("between_days_of_the_month");
        if omit_separator {
            format
        } else {
            String::from(", ") + &format
        }
    }

    fn get_interval_description_format(self: &Self, expression: &String) -> String {
        // return ", "+I18nMessages.get("every_x")+ getSpace(options) + plural(expression, I18nMessages.get("day"), I18nMessages.get("days"));
        ", ".to_string() + &t!("every_x") + &self.get_space() + &Self::plural(expression, &t!("day"), &t!("days"))
    }

    fn get_single_item_description(&self, expression: &String) -> String { "".to_string() }

    fn get_description_format(&self, expression: &String) -> String {
        ", ".to_string() + &t!("on_day_of_month")
    }

    fn need_space_between_words(&self) -> bool {
        self.options.need_space_between_words
    }

    fn get_space(self: &Self) -> String {
        Self::get_space_opt(&self.options)
    }
}

impl DescriptionBuilder<'_> for DayOfWeekDescriptionBuilder<'_> {
    fn get_between_description_format(&self, expression: &String, omit_separator: bool) -> String {
        // MessageFormat.format(", "+I18nMessages.get("interval_description_format"), expression);
        format!("{} {}", ",", t!("messages.interval_description_format", 0 = expression))
    }

    fn get_interval_description_format(&self, expression: &String) -> String {
        String::from(", ") + &t!("messages.interval_description_format", 0 = expression)
    }

    fn get_single_item_description(&self, expression: &String) -> String {
        let exp = match expression.find("#") {
            Some(ind) =>
                expression.substring(0, ind).to_string(),
            None =>
                match expression.find("L") {
                    Some(_) =>
                        expression.replace("L", ""),
                    None => expression.to_string()
                }
        };

        if string_utils::is_numeric(&exp) {
            let mut day_of_week_num = exp.parse::<u8>().unwrap();
            let is_invalid_day_of_week_for_setting = !self.options.zero_based_day_of_week
                && day_of_week_num <= 1;
            if is_invalid_day_of_week_for_setting ||
                (self.options.zero_based_day_of_week && day_of_week_num == 0) {
                return date_time_utils::get_day_of_week_name(7);
            } else if !self.options.zero_based_day_of_week {
                day_of_week_num -= 1;
            }
            return date_time_utils::get_day_of_week_name(day_of_week_num as usize);
        } else {
            // Get localized day of week name
            let lowered = exp.to_lowercase();
            let capitalized = lowered[0..1].to_uppercase() + &lowered[1..];
            return t!(&capitalized);
        }
    }

    fn get_description_format(&self, expression: &String) -> String {
        let format = if expression.contains("#") {
            let hash_ind = expression.find('#').unwrap() + 1;
            let day_of_week_of_month_number = &expression[hash_ind..];
            let day_of_week_month_description = match day_of_week_of_month_number {
                "1" => t!("first"),
                "2" => t!("second"),
                "3" => t!("third"),
                "4" => t!("fourth"),
                "5" => t!("fifth"),
                _ => "".to_string()
            };
            let i18_str = t!("on_the_day_of_the_month");
            let msg = strfmt!(&i18_str, nth => day_of_week_month_description);
            String::from(", ") + msg.unwrap().as_str()
        } else if expression.contains("L") {
            format!("{} {}", ",", t!("on_the_last_of_the_month"))
        } else {
            format!("{} {}", ",", t!("only_on"))
        };
        format
    }

    fn need_space_between_words(self: &Self) -> bool {
        self.options.need_space_between_words
    }

    fn get_space(self: &Self) -> String {
        Self::get_space_opt(&self.options)
    }
}

impl DescriptionBuilder<'_> for HoursDescriptionBuilder<'_> {
    fn get_between_description_format(&self, expression: &String, omit_separator: bool) -> String {
        t!("messages.between_x_and_y")
    }

    fn get_interval_description_format(&self, expression: &String) -> String {
        //  return MessageFormat.format(I18nMessages.get("every_x")+ getSpace(options) +
        //                 plural(expression, I18nMessages.get("hour"), I18nMessages.get("hours")), expression

        let gdf = t!("messages.every_x") + &self.get_space() + &Self::plural(expression, &t!("hour"), &t!("hours"));
        let mut vars = HashMap::new();
        vars.insert("0".to_string(), expression.to_string());
        strfmt(&gdf, &vars).unwrap()
    }

    fn get_single_item_description(&self, expression: &String) -> String {
        format_time(expression, &String::from("0"), &self.options)
    }

    fn get_description_format(&self, expression: &String) -> String {
        t!("messages.at_x")
    }

    fn need_space_between_words(&self) -> bool {
        self.options.need_space_between_words
    }

    fn get_space(&self) -> String {
        Self::get_space_opt(&self.options)
    }
}

impl DescriptionBuilder<'_> for MinutesDescriptionBuilder<'_> {
    fn get_between_description_format(&self, expression: &String, omit_separator: bool) -> String {
        t!("messages.minutes_through_past_the_hour")
    }

    fn get_interval_description_format(&self, expression: &String) -> String {
        // return MessageFormat.format(I18nMessages.get("every_x") + getSpace(options) + minPlural(expression), expression);
        let gdf = t!("messages.every_x") + &self.get_space() + &Self::min_plural(expression);
        let mut vars = HashMap::new();
        vars.insert("0".to_string(), expression.to_string());
        strfmt(&gdf, &vars).unwrap()
    }

    fn get_single_item_description(&self, expression: &String) -> String {
        format_minutes(expression)
    }

    fn get_description_format(&self, expression: &String) -> String {
        if expression == "0" {
            t!("messages.at_x") + &self.get_space() + &Self::min_plural(expression)
        } else {
            self.get_space() + &t!("messages.past_the_hour")
        }
    }

    fn need_space_between_words(&self) -> bool {
        self.options.need_space_between_words
    }

    fn get_space(&self) -> String {
        Self::get_space_opt(&self.options)
    }
}

impl DescriptionBuilder<'_> for MonthDescriptionBuilder<'_> {
    fn get_between_description_format(&self, expression: &String, omit_separator: bool) -> String {
        let format = t!("messages.between_description_format");
        if omit_separator {
            format
        } else {
            format!(", {}", format)
        }
    }

    fn get_interval_description_format(&self, expression: &String) -> String {
        // MessageFormat.format(", "+I18nMessages.get("every_x")+ getSpace(options) +
        //   plural(expression, I18nMessages.get("month"), I18nMessages.get("months")), expression
        // let gdf = t!("messages.every_x") + &self.get_space() + &Self::plural(expression, &t!("hour"), &t!("hours"));
        let month_str = t!("month");
        let months_str = t!("months");
        let plural_str = Self::plural(expression, &month_str, &months_str);
        let gdf = format!(", {}{}{}", t!("messages.every_x"), self.get_space(), plural_str);

        let mut vars = HashMap::new();
        vars.insert("0".to_string(), expression.to_string());
        strfmt(&gdf, &vars).unwrap()
    }

    fn get_single_item_description(&self, expression: &String) -> String {

        // What is this doing?
        // gets the long month name of the year with the integer value of "expression"
        // return new DateTime().withDayOfMonth(1).withMonthOfYear(Integer.parseInt(expression)).
        //     toString("MMMM", I18nMessages.getCurrentLocale());
        let month_num = expression.parse::<usize>().unwrap();
        let month_key = MONTHS_ARR[month_num];
        t!(month_key)
    }

    fn get_description_format(&self, expression: &String) -> String {
        format!(", {}", t!("messages.only_in_month"))
    }

    fn need_space_between_words(&self) -> bool {
        self.options.need_space_between_words
    }

    fn get_space(&self) -> String {
        Self::get_space_opt(&self.options)
    }
}

impl DescriptionBuilder<'_> for SecondsDescriptionBuilder<'_> {
    fn get_between_description_format(&self, expression: &String, omit_separator: bool) -> String {
        t!("messages.seconds_through_past_the_minute")
    }

    fn get_interval_description_format(&self, expression: &String) -> String {
        t!("messages.every_x_seconds")
        // MessageFormat.format(I18nMessages.get("every_x_seconds"), expression);
    }

    fn get_single_item_description(&self, expression: &String) -> String {
        expression.to_string()
    }

    fn get_description_format(&self, expression: &String) -> String {
        t!("messages.at_x_seconds_past_the_minute")
    }

    fn need_space_between_words(&self) -> bool {
        self.options.need_space_between_words
    }

    fn get_space(&self) -> String {
        Self::get_space_opt(&self.options)
    }
}

impl DescriptionBuilder<'_> for YearDescriptionBuilder<'_> {
    fn get_between_description_format(&self, expression: &String, omit_separator: bool) -> String {
        let format = t!("messages.between_description_format");
        if omit_separator {
            format
        } else {
            format!(", {}", format)
        }
    }

    fn get_interval_description_format(&self, expression: &String) -> String {
        // return MessageFormat.format(", " + I18nMessages.get("every_x")+ getSpace(options) +
        //                                 plural(expression, I18nMessages.get("year"), I18nMessages.get("years")), expression);
        // let month_str = t!("month");
        // let months_str = t!("months");
        // let plural_str = Self::plural(expression, &month_str, &months_str);
        // let gdf = format!(", {}{}{}", t!("messages.every_x"), self.get_space(), plural_str);
        //
        // let mut vars = HashMap::new();
        // vars.insert("0".to_string(), expression.to_string());
        // strfmt(&gdf, &vars).unwrap()
        let year_str = t!("year");
        let years_str = t!("years");
        let plural_str = Self::plural(expression, &year_str, &years_str);
        let gdf = format!(", {}{}{}", t!("messages.every_x"), self.get_space(), plural_str);
        let mut vars = HashMap::new();
        vars.insert("0".to_string(), expression.to_string());
        strfmt(&gdf, &vars).unwrap()
    }

    fn get_single_item_description(&self, expression: &String) -> String {
        // return new DateTime().withYear(Integer.parseInt(expression)).toString("yyyy", I18nMessages.getCurrentLocale());
        expression.parse::<u16>().unwrap().to_string()
    }

    fn get_description_format(&self, expression: &String) -> String {
        format!(", {}", t!("messages.only_in_year"))
    }

    fn need_space_between_words(&self) -> bool {
        self.options.need_space_between_words
    }

    fn get_space(&self) -> String {
        Self::get_space_opt(&self.options)
    }
}
