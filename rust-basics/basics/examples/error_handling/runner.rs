use super::unrecoverable_errors;
use super::recoverable_errors;

pub fn run() {
    // unrecoverable_errors::trigger_unrecoverable_errors();
    recoverable_errors::trigger_recovarable_errors();
}