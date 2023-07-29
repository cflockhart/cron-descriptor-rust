use cron_descriptor;
extern crate strfmt;

use cron_descriptor::cronparser::cron_expression_descriptor;
use cron_descriptor::cronparser::cron_expression_descriptor::ParseException;
use cron_descriptor::cronparser::Options;
mod test_utils;
use crate::test_utils::unwrapped_description;
use crate::test_utils::unwrapped_description_options;
use ctor;

#[ctor::ctor]
fn init() {
    rust_i18n::set_locale("pt");
}

#[test]
fn test_parse_exception() {
    match cron_expression_descriptor::get_description_cron("******") {
        Ok(_) => panic!("Got OK, it's not OK"),
        Err(parse_err) => assert_eq!(
            ParseException {
                s: "******".to_string(),
                error_offset: 0
            },
            parse_err
        ),
    }
}

#[test]
fn test_parse_exception2() {
    match cron_expression_descriptor::get_description_cron("* * * * * * * *") {
        Ok(_) => panic!("Got OK, it's not OK"),
        Err(parse_err) => assert_eq!(
            ParseException {
                s: "* * * * * * * *".to_string(),
                error_offset: 7
            },
            parse_err
        ),
    }
}

#[test]
fn test_every_second() {
    assert_eq!(
        "A cada segundo",
        cron_expression_descriptor::get_description_cron("* * * * * *").unwrap()
    );
    assert_eq!(
        "A cada segundo",
        unwrapped_description_options("* * * * * *", &Options::twenty_four_hour())
    );
}

#[test]
fn test_every45seconds() {
    assert_eq!(
        "A cada 45 segundos",
        cron_expression_descriptor::get_description_cron("*/45 * * * * *").unwrap()
    );
    assert_eq!(
        "A cada 45 segundos",
        unwrapped_description_options("*/45 * * * * *", &Options::twenty_four_hour())
    );
}

#[test]
fn test_minute_span() {
    assert_eq!(
        "A cada minuto entre 11:00 AM e 11:10 AM",
        cron_expression_descriptor::get_description_cron("0-10 11 * * *").unwrap()
    );
    assert_eq!(
        "A cada minuto entre 11:00 e 11:10",
        unwrapped_description_options("0-10 11 * * *", &Options::twenty_four_hour())
    );
    assert_eq!(
        "A cada minuto, à(s) 1:00 AM",
        cron_expression_descriptor::get_description_cron("* 1 * * *").unwrap()
    );
    assert_eq!(
        "A cada minuto, à(s) 12:00 AM",
        cron_expression_descriptor::get_description_cron("* 0 * * *").unwrap()
    );
}

#[test]
fn test_every_minute() {
    assert_eq!(
        "A cada minuto",
        cron_expression_descriptor::get_description_cron("* * * * *").unwrap()
    );
    assert_eq!(
        "A cada minuto",
        cron_expression_descriptor::get_description_cron("*/1 * * * *").unwrap()
    );
    assert_eq!(
        "A cada minuto",
        cron_expression_descriptor::get_description_cron("0 0/1 * * * ?").unwrap()
    );
}

#[test]
fn test_every_hour() {
    assert_eq!(
        "A cada hora",
        cron_expression_descriptor::get_description_cron("0 0 * * * ?").unwrap()
    );
    assert_eq!(
        "A cada hora",
        cron_expression_descriptor::get_description_cron("0 0 0/1 * * ?").unwrap()
    );
    assert_eq!(
        "A cada hora",
        cron_expression_descriptor::get_description_cron("0 * * * *").unwrap()
    );
}

#[test]
fn test_every_xminutes() {
    assert_eq!("A cada 5 minutos", unwrapped_description("*/5 * * * *"));
    assert_eq!("A cada 5 minutos", unwrapped_description("0 */5 * * * *"));
    assert_eq!("A cada 10 minutos", unwrapped_description("0 0/10 * * * ?"));
}

#[test]
fn test_daily_at_time() {
    assert_eq!("Às 11:30 AM", unwrapped_description("30 11 * * *"));
    assert_eq!(
        "Às 11:30",
        unwrapped_description_options("30 11 * * *", &Options::twenty_four_hour())
    );
    assert_eq!("Às 11:00 AM", unwrapped_description("0 11 * * *"));
}

#[test]
fn test_time_of_day_certain_days_of_week() {
    assert_eq!(
        "Às 11:00 PM, Segunda a Sexta",
        unwrapped_description("0 23 ? * MON-FRI")
    );
    assert_eq!(
        "Às 23:00, Segunda a Sexta",
        unwrapped_description_options("0 23 ? * MON-FRI", &Options::twenty_four_hour())
    );
    assert_eq!(
        "Às 11:30 AM, Segunda a Sexta",
        unwrapped_description("30 11 * * 1-5")
    );
}

#[test]
fn test_one_month_only() {
    assert_eq!(
        "A cada minuto, somente em Março",
        unwrapped_description("* * * 3 *")
    );
}

#[test]
fn test_two_months_only() {
    assert_eq!(
        "A cada minuto, somente em Março e Junho",
        unwrapped_description("* * * 3,6 *")
    );
}

#[test]
fn test_two_times_each_afternoon() {
    assert_eq!(
        "Às 2:30 PM e 4:30 PM",
        unwrapped_description("30 14,16 * * *")
    );
    assert_eq!(
        "Às 14:30 e 16:30",
        unwrapped_description_options("30 14,16 * * *", &Options::twenty_four_hour())
    );
}

#[test]
fn test_three_times_daily() {
    assert_eq!(
        "Às 6:30 AM, 2:30 PM e 4:30 PM",
        unwrapped_description("30 6,14,16 * * *")
    );
    assert_eq!(
        "Às 06:30, 14:30 e 16:30",
        unwrapped_description_options("30 6,14,16 * * *", &Options::twenty_four_hour())
    );
}

#[test]
fn test_once_aweek() {
    assert_eq!(
        "Às 9:46 AM, somente Domingo",
        unwrapped_description("46 9 * * 0")
    );
    assert_eq!(
        "Às 9:46 AM, somente Domingo",
        unwrapped_description("46 9 * * 7")
    );
    assert_eq!(
        "Às 9:46 AM, somente Segunda",
        unwrapped_description("46 9 * * 1")
    );
    assert_eq!(
        "Às 9:46 AM, somente Sábado",
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
        "Às 9:46 AM, somente Domingo",
        unwrapped_description_options("46 9 * * 1", &options)
    );
    assert_eq!(
        "Às 9:46 AM, somente Segunda",
        unwrapped_description_options("46 9 * * 2", &options)
    );
    assert_eq!(
        "Às 9:46 AM, somente Sábado",
        unwrapped_description_options("46 9 * * 7", &options)
    );
}

#[test]
fn test_twice_aweek() {
    assert_eq!(
        "Às 9:46 AM, somente Segunda e Terça",
        unwrapped_description("46 9 * * 1,2")
    );
    assert_eq!(
        "Às 9:46 AM, somente Domingo e Sábado",
        unwrapped_description("46 9 * * 0,6")
    );
    assert_eq!(
        "Às 9:46 AM, somente Sábado e Domingo",
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
        "Às 9:46 AM, somente Domingo e Segunda",
        unwrapped_description_options("46 9 * * 1,2", &options)
    );
    assert_eq!(
        "Às 9:46 AM, somente Sexta e Sábado",
        unwrapped_description_options("46 9 * * 6,7", &options)
    );
}

#[test]
fn test_day_of_month() {
    assert_eq!(
        "Às 12:23 PM, no dia 15 do mês",
        unwrapped_description("23 12 15 * *")
    );
    assert_eq!(
        "Às 12:23, no dia 15 do mês",
        unwrapped_description_options("23 12 15 * *", &Options::twenty_four_hour())
    );
}

#[test]
fn test_month_name() {
    assert_eq!(
        "Às 12:23 PM, somente em Janeiro",
        unwrapped_description("23 12 * JAN *")
    );
}

#[test]
fn test_day_of_month_with_question_mark() {
    assert_eq!(
        "Às 12:23 PM, somente em Janeiro",
        unwrapped_description("23 12 ? JAN *")
    );
}

#[test]
fn test_month_name_range2() {
    assert_eq!(
        "Às 12:23 PM, Janeiro a Fevereiro",
        unwrapped_description("23 12 * JAN-FEB *")
    );
}

#[test]
fn test_month_name_range3() {
    assert_eq!(
        "Às 12:23 PM, Janeiro a Março",
        unwrapped_description("23 12 * JAN-MAR *")
    );
}

#[test]
fn test_month_name_ranges() {
    assert_eq!(
        "Às 3:00 AM, somente em Janeiro a Março e Maio a Junho",
        unwrapped_description("0 0 3 * 1-3,5-6 *")
    );
}

#[test]
fn test_day_of_week_name() {
    assert_eq!(
        "Às 12:23 PM, somente Domingo",
        unwrapped_description("23 12 * * SUN")
    );
}

#[test]
fn test_day_of_week_range() {
    assert_eq!(
        "A cada 5 minutos, à(s) 3:00 PM, Segunda a Sexta",
        unwrapped_description("*/5 15 * * MON-FRI")
    );
    assert_eq!(
        "A cada 5 minutos, à(s) 3:00 PM, Domingo a Sábado",
        unwrapped_description("*/5 15 * * 0-6")
    );
    assert_eq!(
        "A cada 5 minutos, à(s) 3:00 PM, Sábado a Domingo",
        unwrapped_description("*/5 15 * * 6-7")
    );
}

#[test]
fn test_day_of_week_ranges() {
    assert_eq!(
        "Às 3:00 AM, somente Domingo, Terça a Quinta e Sábado",
        unwrapped_description("0 0 3 * * 0,2-4,6")
    );
}

#[test]
fn test_day_of_week_once_in_month() {
    assert_eq!(
        "A cada minuto, no(a) terceiro(a) Segunda do mês",
        unwrapped_description("* * * * MON#3")
    );
    assert_eq!(
        "A cada minuto, no(a) terceiro(a) Domingo do mês",
        unwrapped_description("* * * * 0#3")
    );
}

#[test]
fn test_last_day_of_the_week_of_the_month() {
    assert_eq!(
        "A cada minuto, no(a) último(a) Quinta do mês",
        unwrapped_description("* * * * 4L")
    );
    assert_eq!(
        "A cada minuto, no(a) último(a) Domingo do mês",
        unwrapped_description("* * * * 0L")
    );
}

#[test]
fn test_last_day_of_the_month() {
    assert_eq!(
        "A cada 5 minutos, no último dia do mês, somente em Janeiro",
        unwrapped_description("*/5 * L JAN *")
    );
}

#[test]
fn test_time_of_day_with_seconds() {
    assert_eq!("Às 2:02:30 PM", unwrapped_description("30 02 14 * * *"));
}

#[test]
fn test_second_intervals() {
    assert_eq!(
        "Segundos 5 a 10 após o minuto",
        unwrapped_description("5-10 * * * * *")
    );
}

#[test]
fn test_second_minutes_hours_intervals() {
    assert_eq!(
        "Segundos 5 a 10 após o minuto, de 30 a 35 minutos após a hora, entre 10:00 AM e 12:00 PM",
        unwrapped_description("5-10 30-35 10-12 * * *")
    );
}

#[test]
fn test_every5minutes_at30seconds() {
    assert_eq!(
        "Aos 30 segundos após o minuto, a cada 5 minutos",
        unwrapped_description("30 */5 * * * *")
    );
}

#[test]
fn test_minutes_past_the_hour_range() {
    assert_eq!(
        "À(s) 30 minutos após a hora, entre 10:00 AM e 1:00 PM, somente Quarta e Sexta",
        unwrapped_description("0 30 10-13 ? * WED,FRI")
    );
}

#[test]
fn test_seconds_past_the_minute_interval() {
    assert_eq!(
        "Aos 10 segundos após o minuto, a cada 5 minutos",
        unwrapped_description("10 0/5 * * * ?")
    );
}

#[test]
fn test_between_with_interval() {
    assert_eq!("A cada 3 minutos, de 02 a 59 minutos após a hora, à(s) 1:00 AM, 9:00 AM e 10:00 PM, entre os dias 11 e 26 do mês, Janeiro a Junho",
                   unwrapped_description("2-59/3 1,9,22 11-26 1-6 ?"));
}

#[test]
fn test_recurring_first_of_month() {
    assert_eq!("Às 6:00 AM", unwrapped_description("0 0 6 1/1 * ?"));
}

#[test]
fn test_minutes_past_the_hour() {
    assert_eq!(
        "À(s) 05 minutos após a hora",
        unwrapped_description("0 5 0/1 * * ?")
    );
}

/**
 * @since https://github.com/RedHogs/cron-parser/issues/2
 */
#[test]
fn test_every_past_the_hour() {
    assert_eq!(
        "À(s) 00, 05, 10, 15, 20, 25, 30, 35, 40, 45, 50 e 55 minutos após a hora",
        unwrapped_description("0 0,5,10,15,20,25,30,35,40,45,50,55 * ? * *")
    );
}

/**
 * @since https://github.com/RedHogs/cron-parser/issues/10
 */
#[test]
fn test_every_xminute_past_the_hour_with_interval() {
    assert_eq!(
        "A cada 2 minutos, de 00 a 30 minutos após a hora, à(s) 5:00 PM, Segunda a Sexta",
        unwrapped_description("0 0-30/2 17 ? * MON-FRI")
    );
}

/**
 * @since https://github.com/RedHogs/cron-parser/issues/13
 */
#[test]
fn test_one_year_only_with_seconds() {
    assert_eq!(
        "A cada segundo, somente em 2013",
        unwrapped_description("* * * * * * 2013")
    );
}

#[test]
fn test_one_year_only_without_seconds() {
    assert_eq!(
        "A cada minuto, somente em 2013",
        unwrapped_description("* * * * * 2013")
    );
}

#[test]
fn test_two_years_only() {
    assert_eq!(
        "A cada minuto, somente em 2013 e 2014",
        unwrapped_description("* * * * * 2013,2014")
    );
}

#[test]
fn test_year_range2() {
    assert_eq!(
        "Às 12:23 PM, Janeiro a Fevereiro, 2013 a 2014",
        unwrapped_description("23 12 * JAN-FEB * 2013-2014")
    );
}

#[test]
fn test_year_range3() {
    assert_eq!(
        "Às 12:23 PM, Janeiro a Março, 2013 a 2015",
        unwrapped_description("23 12 * JAN-MAR * 2013-2015")
    );
}

#[test]
fn test_issue26() {
    assert_eq!(
        "À(s) 05 e 10 minutos após a hora",
        unwrapped_description("5,10 * * * *")
    );
    assert_eq!(
        "À(s) 05 e 10 minutos após a hora, à(s) 12:00 AM",
        unwrapped_description("5,10 0 * * *")
    );
    assert_eq!(
        "À(s) 05 e 10 minutos após a hora, no dia 2 do mês",
        unwrapped_description("5,10 * 2 * *")
    );
    assert_eq!(
        "A cada 10 minutos, no dia 2 do mês",
        unwrapped_description("5/10 * 2 * *")
    );

    assert_eq!(
        "Aos 5 e 6 segundos após o minuto",
        unwrapped_description("5,6 0 * * * *")
    );
    assert_eq!(
        "Aos 5 e 6 segundos após o minuto, à(s) 1:00 AM",
        unwrapped_description("5,6 0 1 * * *")
    );
    assert_eq!(
        "Aos 5 e 6 segundos após o minuto, no dia 2 do mês",
        unwrapped_description("5,6 0 * 2 * *")
    );
}

// #[macro_use]
// extern crate rust_i18n;

// use rust_i18n::set_locale;
// i18n!("locales");
// rust_i18n::set_locale("es");
