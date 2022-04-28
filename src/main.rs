use alfred;
use std::env;
use std::io;
use walkdir::WalkDir;

mod markdown;
use self::markdown::Markdown;

#[derive(Debug, Clone)]
struct IoArg {
    query: String,
    paths: Vec<String>,
    ignores: Vec<String>,
}

/**
 * 过滤输入参数
 */
fn parse_args() -> IoArg {
    let _args: Vec<String> = env::args().collect();

    let mut paths: Vec<String> = Vec::new();
    let mut query: String = String::from("");
    let mut ignores: Vec<String> = vec![
        String::from("/node_modules"),
        String::from("/dist"),
        String::from("/build"),
        String::from("/target"),
    ];
    for (index, arg) in _args.iter().enumerate() {
        let next = index + 1;
        match arg.as_str() {
            "-p" => paths.push(_args[next].clone()),
            "-q" => query = _args[next].to_string(),
            "-i" => ignores.push(_args[next].clone()),
            _ => {
                // panic!("有未知参数传入");
            }
        }
    }

    IoArg {
        query,
        paths,
        ignores,
    }
}

fn main() {
    let io_arg = parse_args();
    let query = io_arg.clone().query;
    // println!("{:?}", io_arg);
    let paths = io_arg.clone().paths;

    let mut items: Vec<Markdown> = Vec::new();

    for dir_path in paths {
        let dirs = WalkDir::new(dir_path).max_depth(4);
        for entry in dirs {
            if items.len() > 10 {
                break;
            }

            let entry = entry.unwrap();
            let path = entry.path().to_string_lossy().to_string();

            // 处理忽略数组
            let mut has_ignores = false;

            for ignore_text in io_arg.clone().ignores {
                let is_has = path.contains(ignore_text.as_str());
                if is_has {
                    has_ignores = true;
                    continue;
                }
            }

            if has_ignores {
                // 有忽略的直接跳过这次循环
                continue;
            }

            if path.ends_with(".md") {
                // 只处理markdown
                // markdown
                let md = Markdown::new(&path);
                let content = md.clone().get_content();
                let filename = md.clone().file_name;
                let find_content = content.find(query.as_str());

                if let Some(find_index) = find_content {
                    let md = md.clone().set_part(find_index);
                    items.push(md);
                    continue;
                }

                if filename.contains(query.as_str()) {
                    // 标题内有包含搜索词的
                    let md = md.clone().set_part(0);
                    items.push(md);
                }
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
