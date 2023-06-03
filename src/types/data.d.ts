export type FileData = {
  file_title: string;
  heading: HeadingData[];
  links: LinkItem[];
};

export type HeadingData = {
  title: string;
  level: number;
  heading: HeadingData[];
  links: LinkData[];
};

export type LinkData = {
  name: string;
  link: string;
  likeability: string;
  read_till: number;
  line_number: number;
};