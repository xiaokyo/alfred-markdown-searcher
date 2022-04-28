use std::fs;

#[derive(Clone, Debug)]
pub struct Markdown {
    pub file_name: String,
    pub file_path: String,
    pub file_content: String,
    pub part: String,
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
