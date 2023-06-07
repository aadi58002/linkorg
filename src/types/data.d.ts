export type FileData = {
  file_name: string;
  file_meta_data?: FileMetaData,
  file_title: string;
  file_description: string;
  file_creation_date: string;
  file_tags: string[];
  heading: HeadingData[];
  links: LinkItem[];
};

export type FileMetaData = {
   file_title?: string,
   file_description?: string,
   file_creation_date?: string,
   file_tags: string[],
}

export type HeadingData = {
  title: string;
  level: number;
  heading: HeadingData[];
  links: LinkData[];
};

export type LinkData = {
  name: string;
  link: string;
  description: string;
  likeability: string;
  read_till: number;
  line_number: number;
};