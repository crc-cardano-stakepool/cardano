use crate::set_env;

pub fn check_confirm(confirm: bool) {
    if confirm {
        set_env("CONFIRM", "true")
    } else {
        set_env("CONFIRM", "false")
    }
}
