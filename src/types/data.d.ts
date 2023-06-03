export type FileData = {
  file_title: string;
  heading: HeadingItem[];
  links: LinkItem[];
};

export type HeadingData = {
  title: string;
  heading: HeadingItem[];
  links: LinkItem[];
};

export type LinkData = {
  name: string;
  link: string;
  likeability: string;
  read_till: number;
  line_number: number;
};