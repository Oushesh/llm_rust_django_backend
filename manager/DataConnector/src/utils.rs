//rust function to loop over directory
//if file is a directory get the files
//else get the images.
use std::{
    fs,
    path::{Path, PathBuf},
};
use vector::VectorDB;
use std::error::Error;

pub struct File {
    pub path: String,
    pub contents: String,
    pub sentences: Vec<String>,
}

struct AppState {
    files: Vec<File>,
    vector_db: VectorDB,
}

enum FileState {
    None,
    CodeBlock,
    Sentence,
    Comments,
}

impl File {
    pub fn new(path: String, contents: String) -> Self {
        Self {
            path,
            contents,
            sentences: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        let mut contents = Vec::new();
        let mut state = FileState::None;
        let mut sentence = String::new();

        for line in self.contents.lines() {
            match state {
                FileState::None => {
                    if line.starts_with("```") {
                        state = FileState::CodeBlock;
                        sentence = String::new();
                        sentence.push_str(line);
                        sentence.push('\n');
                    } else if line.starts_with("---") {
                        state = FileState::Comments;
                    } else if !line.starts_with('#') && !line.is_empty() {
                        state = FileState::Sentence;
                        sentence = String::new();
                        sentence.push_str(line);
                        sentence.push('\n');
                    }
                }
                FileState::CodeBlock => {
                    sentence.push_str(line);
                    if line.starts_with("```") {
                        contents.push(sentence);
                        sentence = String::new();
                        state = FileState::None;
                    }
                }
                FileState::Comments => {
                    if line.starts_with("---") {
                        state = FileState::None;
                    }
                }
                FileState::Sentence => {
                    if line.is_empty() {
                        state = FileState::None;
                        contents.push(sentence);
                        sentence = String::new();
                    } else {
                        sentence.push_str(line);
                        sentence.push('\n');
                    }
                }
            }
        }
        self.sentences = contents;
    }
}

pub fn load_files_from_dir(dir: PathBuf, ending: &str, prefix: &PathBuf) -> Result<Vec<File>, Box<dyn Error>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            let mut sub_files = load_files_from_dir(path, ending, prefix)?;
            files.append(&mut sub_files);
        } else if path.is_file() && path.extension().map_or(false, |ext| ext == ending) {
            println!("Path: {:?}", path);
            let contents = fs::read_to_string(&path)?;
            let path = Path::new(&path).strip_prefix(prefix)?.to_owned();
            let key = path.to_str().ok_or("NotAvailableError")?;
            let mut file = File::new(key.to_string(), contents);
            file.parse();
            files.push(file);
        }
    }
    Ok(files)
}

pub async fn embed_documentation(vector_db: &mut VectorDB, files: &Vec<File>) -> anyhow::Result<()> {
    for file in files {
        let embeddings = open_ai::embed_file(file).await?;
        println!("Embedding: {:?}", file.path);
        for embedding in embeddings.data {
            vector_db.upsert_embedding(embedding, file).await?;
        }
    }

    Ok(())
}