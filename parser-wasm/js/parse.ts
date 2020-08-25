export type ParseResult = BookClippings[];

export type BookClippings = {
  book: {
    title: string;
    author: string;
  };
  clippings: ClippingContent[];
};

export type ClippingContent =
  | { kind: "Highlight", location: Location, text: string }
  | { kind: "Note", location: Location, text: string }
  | { kind: "Bookmark", location: Location }
  | { kind: "ArticleClip", location: Location, text: string };

export type Location = {
  from: number;
  to: number | null;
  kind: "Page" | "Location";
};

declare function parse_clippings(data: string): string;

export function parse(data: string): ParseResult {
  return JSON.parse(parse_clippings(data));
}
