export type ParseResult = BookClippings[];

export type BookClippings = {
  book: {
    title: string;
    author: string;
  };
  clippings: ClippingContent[];
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

declare function parse_clippings(data: string): string;

export function parse(data: string): ParseResult {
  return JSON.parse(parse_clippings(data));
}
