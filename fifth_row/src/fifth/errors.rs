use crate::fifth::order::OrderType;
use snafu::{Backtrace, Snafu};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("Invalid input"))]
    InvalidInput { backtrace: Backtrace },

    #[snafu(display("Invalid token: {:#?}", token))]
    InvalidToken { token: String, backtrace: Backtrace },

    #[snafu(display("Invalid value: {}", value))]
    InvalidValue { value: u32, backtrace: Backtrace },

    #[snafu(display("Wrong number of arguments: {}", args_num))]
    WrongNumberOfArguments {
        args_num: usize,
        backtrace: Backtrace,
    },

    #[snafu(display("Unexpected order type: {:#?}", order_type))]
    UnexpectedOrderType {
        order_type: OrderType,
        backtrace: Backtrace,
    },
}
pub type Result<T, E = Error> = std::result::Result<T, E>;
