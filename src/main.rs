use alfred;
use std::env;
use std::fs;
use std::io;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
struct IoArg {
    query: String,
    paths: Vec<String>
}

fn parse_args() -> IoArg {
    let _args: Vec<String> = env::args().collect();

    let mut paths:Vec<String> = Vec::new();
    let mut query: String = String::from("");
    for (index, arg) in _args.iter().enumerate() {
        if let "-p" = arg.as_str() {
            let path_index = index + 1;
            paths.push(_args[path_index].clone());
        } else if let "-q" = arg.as_str() {
            if query == "" {
                let query_index = index + 1;
                query = _args[query_index].to_string();
            }
        }
    }

    IoArg { query, paths }
}

fn main() {
    let io_arg = parse_args();
    let query = io_arg.clone().query;
    // println!("{:?}", io_arg);
    let dir_path = io_arg.clone().paths[0].to_string();

    let mut items: Vec<Markdown> = Vec::new();

    let dirs = WalkDir::new(dir_path).max_depth(5);
    for entry in dirs {
        if items.len() > 10 {
            break;
        }

        let entry = entry.unwrap();
        let path = entry.path().to_string_lossy().to_string();

        // 忽略node_modules
        let has_node_modules: Vec<_> = path.match_indices("node_modules").collect();
        if has_node_modules.len() > 0 {
            continue;
        }

        if path.ends_with(".md") {
            // 只处理markdown
            // markdown
            let md = Markdown::new(&path);
            let new_md = md.clone();
            let content = md.get_content();
            let finds: Vec<_> = content.match_indices(query.as_str()).collect();
            if finds.len() > 0 {
                let md = new_md.set_part(finds[0].0);
                // println!("{}", md.file_name);
                items.push(md);
            }
        }
    }

    workflow_output(items, true);
}

fn workflow_output(markdowns: Vec<Markdown>, json: bool) {
    let items: Vec<alfred::Item> = markdowns
        .into_iter()
        .map(|item| {
            alfred::ItemBuilder::new(item.file_name)
                .arg(item.file_path.clone())
                .quicklook_url(item.file_path.clone())
                .icon_filetype("fileicon")
                .subtitle(item.part)
                .into_item()
        })
        .collect();
    if json {
        alfred::json::Builder::with_items(&items)
            .write(io::stdout())
            .expect("Couldn't write items to Alfred");
        // alfred::json::write_items(io::stdout(), &items).expect("Couldn't write items to Alfred");
    } else {
        alfred::xml::write_items(io::stdout(), &items).expect("Couldn't write items to Alfred");
    }
}

#[derive(Clone, Debug)]
struct Markdown {
    file_name: String,
    file_path: String,
    file_content: String,
    part: String,
}

impl Markdown {
    pub fn new(file_path: &String) -> Markdown {
        let file_content =
            fs::read_to_string(file_path).expect("Something went wrong reading the file");
        let split_path = file_path.split_inclusive("/");
        let file_name = split_path.last();

        if let Some(file_name) = file_name {
            let file_name = file_name.to_string();
            Markdown {
                file_name,
                file_path: file_path.to_string(),
                file_content,
                part: String::from(""),
            }
        } else {
            panic!("找不到文件: {}", file_path);
        }
    }

    pub fn get_content(self) -> String {
        self.file_content
    }

    pub fn set_part(mut self, _index: usize) -> Markdown {
        // let index = self.index;
        let s1 = String::from(&self.file_content);
        // let mut left: usize = 0;
        // let mut right: usize = self.file_content.len();

        // if index - 50 > 0 {
        //     left = index - 50;
        // }

        // if index + 50 < right {
        //     right = index + 50
        // }
        let finds: Vec<_> = s1.match_indices("\n").collect();
        let result = &s1[0..s1.len()].trim().replacen("\n", "", finds.len());
        self.part = result.to_string();
        self
    }
}
