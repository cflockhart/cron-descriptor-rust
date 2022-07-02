use std::collections::HashMap;
use crate::cronparser::Options;
use crate::string_utils;
use crate::date_time_utils;
use substring::Substring;

extern crate strfmt;
use strfmt::strfmt;

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

    fn get_between_description_format(&self, expression: &String, all_description: &String) -> String;
    fn get_interval_description_format(&self, expression: &String) -> String;
    fn get_single_item_description(&self, expression: &String) -> String;
    fn get_description_format(&self, expression: &String) -> String;
    fn need_space_between_words() -> bool;

    fn get_space_opt(options: &Options) -> String {
        if options.need_space_between_words {
            " ".to_string()
        } else {
            "".to_string()
        }
    }

    fn get_space(self: Self) -> String;

    fn plural_num(num: i8, singular: String, plural: String) -> String {
        Self::plural(num.to_string(), singular, plural)
    }

    fn plural(expression: String, singular: String, plural: String) -> String {
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
    fn get_between_description_format(&self, expression: &String, all_description: &String) -> String {
        todo!()
    }

    fn get_interval_description_format(&self, expression: &String) -> String {
        todo!()
    }

    fn get_single_item_description(&self, expression: &String) -> String { "".to_string() }

    fn get_description_format(&self, expression: &String) -> String {
        todo!()
    }

    fn need_space_between_words() -> bool{
        todo!()
    }

    fn get_space(self: Self) -> String {
        Self::get_space_opt(&self.options)
    }
}

impl DescriptionBuilder for DayOfWeekDescriptionBuilder {
    fn get_between_description_format(&self, expression: &String, all_description: &String) -> String {
        todo!()
    }

    fn get_interval_description_format(&self, expression: &String) -> String {
        todo!()
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
                day_of_week_num = day_of_week_num - 1;
            }
            return date_time_utils::get_day_of_week_name(day_of_week_num as usize);
        } else {
            return "".to_string();
        }

    }

    fn get_description_format(&self, expression: &String) -> String {
        todo!()
    }

    fn need_space_between_words() -> bool {
        todo!()
    }

    fn get_space(self: Self) -> String {
        Self::get_space_opt(&self.options)
    }
}

impl DescriptionBuilder for HoursDescriptionBuilder {
    fn get_between_description_format(&self, expression: &String, all_description: &String) -> String {
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

    fn need_space_between_words() -> bool {
        todo!()
    }

    fn get_space(self: Self) -> String {
        Self::get_space_opt(&self.options)
    }
}

impl DescriptionBuilder for MinutesDescriptionBuilder {
    fn get_between_description_format(&self, expression: &String, all_description: &String) -> String {
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

    fn need_space_between_words() -> bool {
        todo!()
    }

    fn get_space(self: Self) -> String {
        Self::get_space_opt(&self.options)
    }
}

impl DescriptionBuilder for MonthDescriptionBuilder {
    fn get_between_description_format(&self, expression: &String, all_description: &String) -> String {
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

    fn need_space_between_words() -> bool {
        todo!()
    }

    fn get_space(self: Self) -> String {
        Self::get_space_opt(&self.options)
    }
}

impl DescriptionBuilder for SecondsDescriptionBuilder {
    fn get_between_description_format(&self, expression: &String, all_description: &String) -> String {
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

    fn need_space_between_words() -> bool {
        todo!()
    }

    fn get_space(self: Self) -> String {
        Self::get_space_opt(&self.options)
    }
}

impl DescriptionBuilder for YearDescriptionBuilder {
    fn get_between_description_format(&self, expression: &String, all_description: &String) -> String {
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

    fn need_space_between_words() -> bool {
        todo!()
    }

    fn get_space(self: Self) -> String {
        Self::get_space_opt(&self.options)
    }
}
