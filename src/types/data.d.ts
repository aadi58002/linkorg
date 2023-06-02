export type LinkData = { name: string; link: string; read_till: number };

export type FileData = { heading: string; HeadingOrLinks: FileData } | LinkData[];