use fehler::{throw, throws};
use snafu::ensure;

use crate::fifth::{
    errors::{self, Error},
    order::{Order, OrderType},
};

#[throws]
fn parse_id(token: &str) -> usize {
    const ID_MARKER: &str = ":";

    ensure!(
        token.len() > 1 && &token[token.len() - 1..] == ID_MARKER,
        errors::InvalidToken {
            token: token.to_string()
        }
    );

    let res = token[..token.len() - 1].parse().map_err(|_| {
        errors::InvalidToken {
            token: token.to_string(),
        }
        .build()
    })?;

    res
}

#[throws]
fn parse_quantity(token: &str) -> u32 {
    let quantity: u32 = token.parse().map_err(|_| {
        errors::InvalidToken {
            token: token.to_string(),
        }
        .build()
    })?;

    ensure!(
        quantity >= 1 as u32 && quantity <= 999 as u32,
        errors::InvalidValue { value: quantity }
    );

    quantity
}

#[throws]
fn parse_price(token: &str) -> u32 {
    let price: u32 = token.parse().map_err(|_| {
        errors::InvalidToken {
            token: token.to_string(),
        }
        .build()
    })?;

    ensure!(
        price >= 1 as u32 && price <= 99999 as u32,
        errors::InvalidValue { value: price }
    );

    price
}

#[throws]
pub fn parse(s: String) -> Order {
    const TOKEN_NUM: usize = 7;
    const CRYPTO_CURRENCY_MARKER: &str = "BTC";
    const CURRENCY_MARKER: &str = "USD";
    const MARKER: &str = "@";

    let tokens = s.split(" ").collect::<Vec<&str>>();
    ensure!(
        tokens.len() == TOKEN_NUM,
        errors::WrongNumberOfArguments {
            args_num: tokens.len()
        }
    );

    let id: usize = parse_id(&tokens[0])?;

    let order_type = match tokens[1] {
        "Sell" => OrderType::Sell,
        "Buy" => OrderType::Buy,
        _ => throw!(errors::InvalidInput.build()),
    };

    let quantity: u32 = parse_quantity(&tokens[2])?;

    if !(tokens[3] == CRYPTO_CURRENCY_MARKER && tokens[4] == MARKER && tokens[6] == CURRENCY_MARKER)
    {
        throw!(errors::InvalidInput.build());
    }

    let price = parse_price(&tokens[5])?;

    Order {
        id,
        order_type,
        quantity,
        price,
    }
}
