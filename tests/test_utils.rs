
use cron_descriptor::cronparser::cron_expression_descriptor;
use cron_descriptor::cronparser::Options;

pub fn unwrapped_description(cron_expr: &str) -> String {
    cron_expression_descriptor::get_description_cron(cron_expr).unwrap()
}

pub fn unwrapped_description_options(cron_expr: &str, opts: &Options) -> String {
    cron_expression_descriptor::get_description_cron_options(cron_expr, opts).unwrap()
}
