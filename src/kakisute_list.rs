use crate::kakisute_file::KakisuteFile;
use std::{
    fs::ReadDir,
    io::{self, Write},
};

pub struct KakisuteList {
    files: Vec<KakisuteFile>,
}

impl KakisuteList {
    pub fn new() -> Self {
        KakisuteList { files: vec![] }
    }

    pub fn from_dir(read_dir: ReadDir) -> Self {
        let mut kakisute_list = Self::new();

        for file in read_dir {
            let kakisute_file = KakisuteFile::from_path(&file.unwrap().path());
            if let Some(kakisute_file) = kakisute_file {
                kakisute_list.add(kakisute_file);
            }
        }

        kakisute_list.sort();

        kakisute_list
    }

    pub fn print_list(&self) {
        let stdout = io::stdout();
        let mut handle = io::BufWriter::new(stdout);
        for file in &self.files {
            writeln!(handle, "{}", file.file_name()).unwrap();
        }
    }

    pub fn get_latest(&self) -> &KakisuteFile {
        self.files.iter().max().unwrap()
    }

    pub fn get_by_file_name(&self, file_name: &str) -> Option<&KakisuteFile> {
        self.files.iter().find(|file| file.file_name() == file_name)
    }

    fn sort(&mut self) {
        self.files.sort();
    }

    fn add(&mut self, file: KakisuteFile) {
        self.files.push(file);
    }
}
