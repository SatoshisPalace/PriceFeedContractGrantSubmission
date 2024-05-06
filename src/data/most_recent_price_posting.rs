use secret_toolkit::storage::Item;
use sp_secret_toolkit::macros::identifiable::Identifiable;

use super::price_posting::PricePosting;

pub static MOST_RECENT_PRICE_POSTING: Item<<PricePosting as Identifiable>::ID> =
    Item::new(b"most_recent_price_posting");
