use clippings_parser;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type ParseResult = Clipping[];

export type Clipping = {
  title: string;
  author: string;
  content: ClippingContent;
};

export type ClippingContent =
  | { kind: "ClippingHighlight", location: Location, text: string }
  | { kind: "ClippingNote", location: Location, text: string }
  | { kind: "ClippingBookmark", location: Location }
  | { kind: "ClippingArticleClip", location: Location, text: string };

export type Location = {
    from: number;
    to: number | null;
    kind: "Page" | "Location";
};
"#;

#[wasm_bindgen]
pub fn parse_clippings(text: &str) -> String {
    let clippings = clippings_parser::parse(text).unwrap().1;

    serde_json::to_string(&clippings).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    use insta::*;

    const SINGLE_HIGHLIGHT: &str = "Flow (Mihaly Csikszentmihalyi)\r
- Your Highlight at location 1213-1214 | Added on Sunday, 12 July 2015 17:36:17\r
\r
The reason it is possible to achieve such complete involvement in a flow experience is that goals are usually clear, and feedback immediate.\r
==========\r
";

    fn prettify_json(json: String) -> String {
        serde_json::to_string_pretty(&serde_json::from_str::<serde_json::Value>(&json).unwrap())
            .unwrap()
    }

    #[test]
    fn parse_single_clipping() {
        let res: String = prettify_json(parse_clippings(SINGLE_HIGHLIGHT));

        assert_snapshot!(res);
    }
}
