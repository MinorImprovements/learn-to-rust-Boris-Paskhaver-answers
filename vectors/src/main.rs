#[derive(Debug)]
struct File {
    name: String,
}

#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>,
}

impl Folder {
    fn new(name: String) -> Folder {
        Self {
            name,
            contents: Vec::new(),
        }
    }

    fn create_file(&mut self, name: String) {
        self.contents.push(File { name });
    }

    fn delete_file(&mut self, index: usize) -> File {
        self.contents.remove(index)
    }

    fn get_file(&self, index: usize) -> Option<&File> {
        self.contents.get(index)
    }
}

fn main() {
    let mut game_folder = Folder::new(String::from("Games"));
    game_folder.create_file(String::from("Minecraft"));
    game_folder.create_file(String::from("Deadlock"));
    println!("{:#?}", game_folder);
    game_folder.delete_file(0);
    println!("{:#?}", game_folder);
    let file = game_folder.get_file(0);

    match file {
        Some(game) => println!("The game is {game:?}"),
        None => println!("There is nothing in this folder!"),
    }
}
