use crate::border_box::BoxStyle;
use crate::colour;

pub const BALANCES: BoxStyle = BoxStyle::new (colour::BLACK, colour::BALANCES, colour::WHITE);
pub const BUY: BoxStyle = BoxStyle::new (colour::BLACK, colour::BUY, colour::WHITE);
pub const ERROR: BoxStyle = BoxStyle::new (colour::BLACK, colour::ERROR, colour::WHITE);
pub const FETCHING: BoxStyle = BoxStyle::new (colour::BLACK, colour::FETCHING, colour::WHITE);
pub const MOVE: BoxStyle = BoxStyle::new (colour::BLACK, colour::MOVE, colour::WHITE);
pub const SELL: BoxStyle = BoxStyle::new (colour::BLACK, colour::SELL, colour::WHITE);
pub const TAB: BoxStyle = BoxStyle::new (colour::BLACK, colour::TAB, colour::WHITE);
pub const TAB_ERROR: BoxStyle = BoxStyle::new (colour::BLACK, colour::TAB_ERROR, colour::WHITE);
pub const TAB_NOTICE: BoxStyle = BoxStyle::new (colour::BLACK, colour::TAB_NOTICE, colour::WHITE);
pub const TITLE: BoxStyle = BoxStyle::new (colour::BLACK, colour::TITLE, colour::WHITE);
