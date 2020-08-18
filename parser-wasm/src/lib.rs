use clippings_parser::{self, sort_clippings_list, ClippingsList};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_clippings(text: &str) -> String {
    let clippings: ClippingsList = clippings_parser::parse(text).unwrap().1.into();

    serde_json::to_string(&clippings).unwrap()
}

#[wasm_bindgen]
pub fn parse_clippings_sorted(text: &str) -> String {
    let mut clippings: ClippingsList = clippings_parser::parse(text).unwrap().1.into();

    sort_clippings_list(&mut clippings);

    serde_json::to_string(&clippings).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    use insta::*;

    const BOOK_1_HIGHLIGHT_1: &str = "Book 1 (Author X)\r
- Your Highlight at location 1-100 | Added on Sunday, 12 July 2015 17:36:17\r
\r
Book 1 highlight 1 content.\r
==========\r
";

    const BOOK_1_HIGHLIGHT_2: &str = "Book 1 (Author X)\r
- Your Highlight at location 2-200 | Added on Sunday, 12 July 2015 17:36:17\r
\r
Book 1 highlight 2 content.\r
==========\r
";

    const BOOK_2_HIGHLIGHT_1: &str = "Book 2 (Author X)\r
- Your Highlight at location 1-100 | Added on Sunday, 12 July 2015 17:36:17\r
\r
Book 2 highlight 1 content.\r
==========\r
";

    fn prettify_json(json: String) -> String {
        serde_json::to_string_pretty(&serde_json::from_str::<serde_json::Value>(&json).unwrap())
            .unwrap()
    }

    #[test]
    fn parse_multiple() {
        let input = &[BOOK_1_HIGHLIGHT_1, BOOK_2_HIGHLIGHT_1, BOOK_1_HIGHLIGHT_2].concat();

        let res: String = prettify_json(parse_clippings_sorted(&input));

        assert_snapshot!(res);
    }
}
