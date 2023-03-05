# cron-descriptor-rust
A Rust library that converts cron expressions into human-readable strings.
Translated from the Java version at https://github.com/grahamar/cron-parser
Origial Project in C#, on which the Java version is based: https://github.com/bradymholt/cron-expression-descriptor

# i18n
Only English and Spanisih are available right now, but translating from the Java properties files to YAML in `locales` is straightforward.

# Usage
See `tests.rs`. E.g. 

```
use crate::cronparser::cron_expression_descriptor;
assert_eq!("Every 45 seconds", cron_expression_descriptor::get_description_cron("*/45 * * * * *".to_string()));
```
