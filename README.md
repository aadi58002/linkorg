# Link Org

# Brief Description
This project aims to create a cross platform gui frontend for the `.org` for emacs and '.md' with have links in the format of `[[link][description]]` and `.md` with links `[description](link)`

There can be heading in org files which start with `*` with the no of `*` represting the level or depth of the heading and `#` in markdown.

# Example
## Md
- Support for normal links  
`  [Book name](Book link)         (Personal thoughts)   (Must read)   -- after 34`
- Support for links inside a table  
`| [Book name](Book link)       | (Personal thoughts) | (Interesting read) | -- after 34 |`
## Org
- Support for normal links  
`  [[Book link][Book name]]         (Personal thoughts)   (Must read)   -- after 34`
- Support for links inside a table  
`| [[Book link][Book name]]       | (Personal thoughts) | (Must read) | -- after 34 |`

Book link             - Link to open the book in browser  
Book name             - Name of the book  
Personal Thoughts     - Personal throughs on the book  (Optional)  
Must read             - How much do you want to read the book (Optional)  
after 34              - Read the book after page 34  

# How to run
## Setup
After that just run the app with below command
```bash
pnpm i
pnpm tauri dev
```

By default linkorg looks for any files in your system's Documents folder (Only top level files).
You can change it by changing the `notes_dir` variable in config.toml in you Config folder under linkorg.
# Working
## Backend
The rust backend uses the glob crate to search through all the org files given under the path from the config file.

After that using the regular expression in the parse.rs file it outputs a FileData struct which represents the data inside the file.

## Frontend
This then coverts the FileData struct into a graphical html and ts website which is then provided with data through the tauri interface from rust to js.

# Future Goals
- [x] Support general mark up formats like markdown
- [x] Support more of the org file syntax like tags,File Description
- [ ] Remove unsafe code from the parser
- [ ] Possible to update the file from the svelte frontend
