import source from "../source.json" assert { type: "json" };

export type Category =
  | "tutorial"
  | "podcasts"
  | "books"
  | "videos"
  | "more"
  | "even_more";

export interface Material {
  name: string;
  desc: string;
  link: string;
}

export const categories = (): string[] => {
  return Object.keys(source);
};

export const indexer = (category: Category, element: Material) => {
  return source[category].indexOf(element);
};

export const material = (category: Category, index: number): Material => {
  return source[category][index];
};

export const pager = (
  category: Category,
  page_number: number,
  page_size = 5,
): Material[] => {
  return source[category].slice(
    (page_number - 1) * page_size,
    page_number * page_size,
  );
};
