use std::collections::HashMap;
use crate::cronparser::Options;
use crate::string_utils;
use crate::date_time_utils;
use substring::Substring;

use strfmt::FmtError;
use strfmt::{strfmt, strfmt_builder};
use crate::date_time_utils::format_time;

i18n!("locales");

const SPECIAL_CHARACTERS_MINUS_STAR: [char; 3] = ['/', '-', ','];

trait DescriptionBuilder {
    fn get_segment_description(&self, expression: &String, all_description: String) -> String {
         if expression.is_empty() {
             "".to_string()
         } else if expression == "*" {
             all_description
         } else if string_utils::not_contains_any(expression, &SPECIAL_CHARACTERS_MINUS_STAR)  {
             let gdf = self.get_description_format(expression);
             let sid = self.get_single_item_description(expression);
             let mut vars = HashMap::new();
             vars.insert("0".to_string(), sid);
             strfmt(&gdf, &vars).unwrap()
         } else {
             "".to_string()
         }
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

    fn plural_num<'a>(num: i8, singular: &'a String, plural: &'a String) -> &'a String {
        Self::plural(&num.to_string(), singular, plural)
    }

    fn plural<'a>(expression: &String, singular: &'a String, plural: &'a String) -> &'a String {
        let parsed_expr = expression.parse::<i8>();
        if parsed_expr.is_ok() && parsed_expr.unwrap() > 1 {
            plural
        } else if expression.contains(",") {
            plural
        } else {
            singular
        }
    }
}

struct DayOfMonthDescriptionBuilder {
    options: Options
}

struct DayOfWeekDescriptionBuilder {
    options: Options
}

struct HoursDescriptionBuilder {
    options: Options
}

struct MinutesDescriptionBuilder {
    options: Options
}

struct MonthDescriptionBuilder {
    options: Options
}

struct SecondsDescriptionBuilder {
    options: Options
}

struct YearDescriptionBuilder {
    options: Options
}

impl DescriptionBuilder for DayOfMonthDescriptionBuilder {
    fn get_between_description_format(&self, expression: &String, omit_separator: bool) -> String {
        let format = t!("between_days_of_the_month");
        if omit_separator {
            format
        } else {
            String::from(", ") + &format
        }
    }

    fn get_interval_description_format(self: &Self , expression: &String) -> String {
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

impl DescriptionBuilder for DayOfWeekDescriptionBuilder {
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

impl DescriptionBuilder for HoursDescriptionBuilder {
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

impl DescriptionBuilder for MinutesDescriptionBuilder {
    fn get_between_description_format(&self, expression: &String, omit_separator: bool) -> String {
        todo!()
    }

    fn get_interval_description_format(&self, expression: &String) -> String {
        todo!()
    }

    fn get_single_item_description(&self, expression: &String) -> String {
        todo!()
    }

    fn get_description_format(&self, expression: &String) -> String {
        todo!()
    }

    fn need_space_between_words(&self) -> bool {
        todo!()
    }

    fn get_space(&self) -> String {
        Self::get_space_opt(&self.options)
    }
}

impl DescriptionBuilder for MonthDescriptionBuilder {
    fn get_between_description_format(&self, expression: &String, omit_separator: bool) -> String {
        todo!()
    }

    fn get_interval_description_format(&self, expression: &String) -> String {
        todo!()
    }

    fn get_single_item_description(&self, expression: &String) -> String {
        todo!()
    }

    fn get_description_format(&self, expression: &String) -> String {
        todo!()
    }

    fn need_space_between_words(&self) -> bool {
        todo!()
    }

    fn get_space(&self) -> String {
        Self::get_space_opt(&self.options)
    }
}

impl DescriptionBuilder for SecondsDescriptionBuilder {
    fn get_between_description_format(&self, expression: &String, omit_separator: bool) -> String {
        todo!()
    }

    fn get_interval_description_format(&self, expression: &String) -> String {
        todo!()
    }

    fn get_single_item_description(&self, expression: &String) -> String {
        todo!()
    }

    fn get_description_format(&self, expression: &String) -> String {
        todo!()
    }

    fn need_space_between_words(&self) -> bool {
        todo!()
    }

    fn get_space(&self) -> String {
        Self::get_space_opt(&self.options)
    }
}

impl DescriptionBuilder for YearDescriptionBuilder {
    fn get_between_description_format(&self, expression: &String, omit_separator: bool) -> String {
        todo!()
    }

    fn get_interval_description_format(&self, expression: &String) -> String {
        todo!()
    }

    fn get_single_item_description(&self, expression: &String) -> String {
        todo!()
    }

    fn get_description_format(&self, expression: &String) -> String {
        todo!()
    }

    fn need_space_between_words(&self) -> bool {
        todo!()
    }

    fn get_space(&self) -> String {
        Self::get_space_opt(&self.options)
    }
}
