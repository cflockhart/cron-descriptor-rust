use cron_descriptor;
extern crate strfmt;

use cron_descriptor::cronparser::Options;
mod test_utils;
use crate::test_utils::unwrapped_description_options;
use crate::test_utils::unwrapped_description;
use ctor;


#[ctor::ctor]
fn init() {
    rust_i18n::set_locale("es");
}

#[test]
fn test_every_second() {
    assert_eq!(
        "Cada segundo",
        unwrapped_description("* * * * * *")
    );
    assert_eq!(
        "Cada segundo",
        unwrapped_description_options(
            "* * * * * *",
            &Options::twenty_four_hour()
        )
    );
}

#[test]
fn test_every45seconds() {
    assert_eq!(
        "Cada 45 segundos",
        unwrapped_description("*/45 * * * * *")
    );
    assert_eq!(
        "Cada 45 segundos",
        unwrapped_description_options(
            "*/45 * * * * *",
            &Options::twenty_four_hour()
        )
    );
}

#[test]
fn test_minute_span() {
    assert_eq!(
        "Cada minuto entre 11:00 AM y 11:10 AM",
        unwrapped_description("0-10 11 * * *")
    );
    assert_eq!(
        "Cada minuto entre 11:00 y 11:10",
        unwrapped_description_options(
            "0-10 11 * * *",
            &Options::twenty_four_hour()
        )
    );
    assert_eq!(
        "Cada minuto, en 1:00 AM",
        unwrapped_description("* 1 * * *")
    );
    assert_eq!(
        "Cada minuto, en 12:00 AM",
        unwrapped_description("* 0 * * *")
    );
}

#[test]
fn test_every_minute() {
    assert_eq!(
        "Cada minuto",
        unwrapped_description("* * * * *")
    );
    assert_eq!(
        "Cada minuto",
        unwrapped_description("*/1 * * * *")
    );
    assert_eq!(
        "Cada minuto",
        unwrapped_description("0 0/1 * * * ?")
    );
}

#[test]
fn test_every_hour() {
    assert_eq!(
        "Cada hora",
        unwrapped_description("0 0 * * * ?")
    );
    assert_eq!(
        "Cada hora",
        unwrapped_description("0 0 0/1 * * ?")
    );
    assert_eq!(
        "Cada hora",
        unwrapped_description("0 * * * *")
    );
}

#[test]
fn test_every_xminutes() {
    assert_eq!(
        "Cada 5 minutos",
        unwrapped_description("*/5 * * * *")
    );
    assert_eq!(
        "Cada 5 minutos",
        unwrapped_description("0 */5 * * * *")
    );
    assert_eq!(
        "Cada 10 minutos",
        unwrapped_description("0 0/10 * * * ?")
    );
}

#[test]
fn test_daily_at_time() {
    assert_eq!(
        "En 11:30 AM",
        unwrapped_description("30 11 * * *")
    );
    assert_eq!(
        "En 11:30",
        unwrapped_description_options(
            "30 11 * * *",
            &Options::twenty_four_hour()
        )
    );
    assert_eq!(
        "En 11:00 AM",
        unwrapped_description("0 11 * * *")
    );
}

#[test]
fn test_time_of_day_certain_days_of_week() {
    assert_eq!(
        "En 11:00 PM, lunes hasta viernes",
        unwrapped_description("0 23 ? * MON-FRI")
    );
    assert_eq!(
        "En 23:00, lunes hasta viernes",
        unwrapped_description_options(
            "0 23 ? * MON-FRI",
            &Options::twenty_four_hour()
        )
    );
    assert_eq!(
        "En 11:30 AM, lunes hasta viernes",
        unwrapped_description("30 11 * * 1-5")
    );
}

#[test]
fn test_one_month_only() {
    assert_eq!(
        "Cada minuto, sólo en marzo",
        unwrapped_description("* * * 3 *")
    );
}

#[test]
fn test_two_months_only() {
    assert_eq!(
        "Cada minuto, sólo en marzo y junio",
        unwrapped_description("* * * 3,6 *")
    );
}

#[test]
fn test_two_times_each_afternoon() {
    assert_eq!(
        "En 2:30 PM y 4:30 PM",
        unwrapped_description("30 14,16 * * *")
    );
    assert_eq!(
        "En 14:30 y 16:30",
        unwrapped_description_options(
            "30 14,16 * * *",
            &Options::twenty_four_hour()
        )
    );
}

#[test]
fn test_three_times_daily() {
    assert_eq!(
        "En 6:30 AM, 2:30 PM y 4:30 PM",
        unwrapped_description("30 6,14,16 * * *")
    );
    assert_eq!(
        "En 06:30, 14:30 y 16:30",
        unwrapped_description_options(
            "30 6,14,16 * * *",
            &Options::twenty_four_hour()
        )
    );
}

#[test]
fn test_once_aweek() {
    assert_eq!(
        "En 9:46 AM, sólo en domingo",
        unwrapped_description("46 9 * * 0")
    );
    assert_eq!(
        "En 9:46 AM, sólo en domingo",
        unwrapped_description("46 9 * * 7")
    );
    assert_eq!(
        "En 9:46 AM, sólo en lunes",
        unwrapped_description("46 9 * * 1")
    );
    assert_eq!(
        "En 9:46 AM, sólo en sábado",
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
        "En 9:46 AM, sólo en domingo",
        unwrapped_description_options(
            "46 9 * * 1",
            &options
        )
    );
    assert_eq!(
        "En 9:46 AM, sólo en lunes",
        unwrapped_description_options(
            "46 9 * * 2",
            &options
        )
    );
    assert_eq!(
        "En 9:46 AM, sólo en sábado",
        unwrapped_description_options(
            "46 9 * * 7",
            &options
        )
    );
}

#[test]
fn test_twice_aweek() {
    assert_eq!(
        "En 9:46 AM, sólo en lunes y martes",
        unwrapped_description("46 9 * * 1,2")
    );
    assert_eq!(
        "En 9:46 AM, sólo en domingo y sábado",
        unwrapped_description("46 9 * * 0,6")
    );
    assert_eq!(
        "En 9:46 AM, sólo en sábado y domingo",
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
        "En 9:46 AM, sólo en domingo y lunes",
        unwrapped_description_options(
            "46 9 * * 1,2",
            &options
        )
    );
    assert_eq!(
        "En 9:46 AM, sólo en viernes y sábado",
        unwrapped_description_options(
            "46 9 * * 6,7",
            &options
        )
    );
}

#[test]
fn test_day_of_month() {
    assert_eq!(
        "En 12:23 PM, en el 15 día del mes",
        unwrapped_description("23 12 15 * *")
    );
    assert_eq!(
        "En 12:23, en el 15 día del mes",
        unwrapped_description_options(
            "23 12 15 * *",
            &Options::twenty_four_hour()
        )
    );
}

#[test]
fn test_month_name() {
    assert_eq!(
        "En 12:23 PM, sólo en enero",
        unwrapped_description("23 12 * JAN *")
    );
}

#[test]
fn test_day_of_month_with_question_mark() {
    assert_eq!(
        "En 12:23 PM, sólo en enero",
        unwrapped_description("23 12 ? JAN *")
    );
}

#[test]
fn test_month_name_range2() {
    assert_eq!(
        "En 12:23 PM, enero hasta febrero",
        unwrapped_description("23 12 * JAN-FEB *")
    );
}

#[test]
fn test_month_name_range3() {
    assert_eq!(
        "En 12:23 PM, enero hasta marzo",
        unwrapped_description("23 12 * JAN-MAR *")
    );
}

#[test]
fn test_month_name_ranges() {
    assert_eq!(
        "En 3:00 AM, sólo en enero hasta marzo y mayo hasta junio",
        unwrapped_description("0 0 3 * 1-3,5-6 *")
    );
}

#[test]
fn test_day_of_week_name() {
    assert_eq!(
        "En 12:23 PM, sólo en domingo",
        unwrapped_description("23 12 * * SUN")
    );
}

#[test]
fn test_day_of_week_range() {
    assert_eq!(
        "Cada 5 minutos, en 3:00 PM, lunes hasta viernes",
        unwrapped_description("*/5 15 * * MON-FRI")
    );
    assert_eq!(
        "Cada 5 minutos, en 3:00 PM, domingo hasta sábado",
        unwrapped_description("*/5 15 * * 0-6")
    );
    assert_eq!(
        "Cada 5 minutos, en 3:00 PM, sábado hasta domingo",
        unwrapped_description("*/5 15 * * 6-7")
    );
}

#[test]
fn test_day_of_week_ranges() {
    assert_eq!(
        "En 3:00 AM, sólo en domingo, martes hasta jueves y sábado",
        unwrapped_description("0 0 3 * * 0,2-4,6")
    );
}

#[test]
fn test_day_of_week_once_in_month() {
    assert_eq!(
        "Cada minuto, en el terzero lunes del mes",
        unwrapped_description("* * * * MON#3")
    );
    assert_eq!(
        "Cada minuto, en el terzero domingo del mes",
        unwrapped_description("* * * * 0#3")
    );
}

#[test]
fn test_last_day_of_the_week_of_the_month() {
    assert_eq!(
        "Cada minuto, en el último jueves del mes",
        unwrapped_description("* * * * 4L")
    );
    assert_eq!(
        "Cada minuto, en el último domingo del mes",
        unwrapped_description("* * * * 0L")
    );
}

#[test]
fn test_last_day_of_the_month() {
    assert_eq!(
        "Cada 5 minutos, en el último día del mes, sólo en enero",
        unwrapped_description("*/5 * L JAN *")
    );
}

#[test]
fn test_time_of_day_with_seconds() {
    assert_eq!(
        "En 2:02:30 PM",
        unwrapped_description("30 02 14 * * *")
    );
}

#[test]
fn test_second_intervals() {
    assert_eq!(
        "5 segundos 10 después el minuto",
        unwrapped_description("5-10 * * * * *")
    );
}

#[test]
fn test_second_minutes_hours_intervals() {
    assert_eq!("5 segundos 10 después el minuto, desde 30 hasta el 35 minuto después de la hora, entre 10:00 AM y 12:00 PM",
                   unwrapped_description("5-10 30-35 10-12 * * *"));
}

#[test]
fn test_every5minutes_at30seconds() {
    assert_eq!(
        "30 segundos después el minuto, cada 5 minutos",
        unwrapped_description("30 */5 * * * *")
    );
}

#[test]
fn test_minutes_past_the_hour_range() {
    assert_eq!(
        "En 30 minutos pasada la hora, entre 10:00 AM y 1:00 PM, sólo en miércoles y viernes",
        unwrapped_description("0 30 10-13 ? * WED,FRI")
    );
}

#[test]
fn test_seconds_past_the_minute_interval() {
    assert_eq!(
        "10 segundos después el minuto, cada 5 minutos",
        unwrapped_description("10 0/5 * * * ?")
    );
}

#[test]
fn test_between_with_interval() {
    assert_eq!("Cada 3 minutos, desde 02 hasta el 59 minuto después de la hora, en 1:00 AM, 9:00 AM y 10:00 PM, entre el 11 y el 26 del mes, enero hasta junio",
                   unwrapped_description("2-59/3 1,9,22 11-26 1-6 ?"));
}

#[test]
fn test_recurring_first_of_month() {
    assert_eq!(
        "En 6:00 AM",
        unwrapped_description("0 0 6 1/1 * ?")
    );
}

#[test]
fn test_minutes_past_the_hour() {
    assert_eq!(
        "En 05 minutos pasada la hora",
        unwrapped_description("0 5 0/1 * * ?")
    );
}

/**
 * @since https://github.com/RedHogs/cron-parser/issues/2
 */
#[test]
fn test_every_past_the_hour() {
    assert_eq!(
        "En 00, 05, 10, 15, 20, 25, 30, 35, 40, 45, 50 y 55 minutos pasada la hora",
        unwrapped_description(
            "0 0,5,10,15,20,25,30,35,40,45,50,55 * ? * *"
        )
    );
}

/**
 * @since https://github.com/RedHogs/cron-parser/issues/10
 */
#[test]
fn test_every_xminute_past_the_hour_with_interval() {
    assert_eq!(
        "Cada 2 minutos, desde 00 hasta el 30 minuto después de la hora, en 5:00 PM, lunes hasta viernes",
        unwrapped_description("0 0-30/2 17 ? * MON-FRI")
    );
}

/**
 * @since https://github.com/RedHogs/cron-parser/issues/13
 */
#[test]
fn test_one_year_only_with_seconds() {
    assert_eq!(
        "Cada segundo, sólo en 2013",
        unwrapped_description("* * * * * * 2013")
    );
}

#[test]
fn test_one_year_only_without_seconds() {
    assert_eq!(
        "Cada minuto, sólo en 2013",
        unwrapped_description("* * * * * 2013")
    );
}

#[test]
fn test_two_years_only() {
    assert_eq!(
        "Cada minuto, sólo en 2013 y 2014",
        unwrapped_description("* * * * * 2013,2014")
    );
}

#[test]
fn test_year_range2() {
    assert_eq!(
        "En 12:23 PM, enero hasta febrero, 2013 hasta 2014",
        unwrapped_description("23 12 * JAN-FEB * 2013-2014")
    );
}

#[test]
fn test_year_range3() {
    assert_eq!(
        "En 12:23 PM, enero hasta marzo, 2013 hasta 2015",
        unwrapped_description("23 12 * JAN-MAR * 2013-2015")
    );
}

#[test]
fn test_issue26() {
    assert_eq!(
        "En 05 y 10 minutos pasada la hora",
        unwrapped_description("5,10 * * * *")
    );
    assert_eq!(
        "En 05 y 10 minutos pasada la hora, en 12:00 AM",
        unwrapped_description("5,10 0 * * *")
    );
    assert_eq!(
        "En 05 y 10 minutos pasada la hora, en el 2 día del mes",
        unwrapped_description("5,10 * 2 * *")
    );
    assert_eq!(
        "Cada 10 minutos, en el 2 día del mes",
        unwrapped_description("5/10 * 2 * *")
    );

    assert_eq!(
        "5 y 6 segundos después el minuto",
        unwrapped_description("5,6 0 * * * *")
    );
    assert_eq!(
        "5 y 6 segundos después el minuto, en 1:00 AM",
        unwrapped_description("5,6 0 1 * * *")
    );
    assert_eq!(
        "5 y 6 segundos después el minuto, en el 2 día del mes",
        unwrapped_description("5,6 0 * 2 * *")
    );
}
