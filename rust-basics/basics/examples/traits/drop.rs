use std::fs::File;
use std::path::{Path, PathBuf};

struct Droppable {
    name: &'static str,
}

enum DroppableEnum {
    D,
    E,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}
impl Drop for DroppableEnum {
    fn drop(&mut self) {
        match self {
            DroppableEnum::D => println!("> Dropping d"),
            DroppableEnum::E => println!("> Dropping e"),
        }
    }
}

fn drop_scope_demo() {
    let _a = Droppable { name: "a" };
    // block 1
    {
        let _b = Droppable { name: "b" };
        let d = DroppableEnum::D;
        // block 2
        {
            let _c = Droppable { name: "c" };
            let e = DroppableEnum::E;
            println!("Exiting block 2");
        }
        println!("Just exited block 2");
        println!("Exiting block 1");
    }
    println!("Just exited block 1");
    drop(_a);
    println!("end of the drop_scope_demo function");
}

struct TempFile {
    file: File,
    path: PathBuf,
}

impl TempFile {
    fn new(mut path: PathBuf) -> std::io::Result<Self> {

         if path.is_relative() {
            let source_dir = Path::new(file!()).parent().unwrap();
            path = source_dir.join(path);
        }
        let file = File::create(&path)?;
        
        Ok(Self { file, path })
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        if let Err(e) = std::fs::remove_file(&self.path) {
        eprintln!("Failed to remove temporary file: {}", e);
        }
        println!("> Dropped temporary file: {:?}", self.path);
    }
}

fn drop_tempfile_demo() -> std::io::Result<()> {
    {
        let temp = TempFile::new("test.txt".into())?;
        println!("Temporary file created");
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
    println!("End of scope - file should be cleaned up");

    let temp2 = TempFile::new("another_test.txt".into())?;
    std::thread::sleep(std::time::Duration::from_secs(2));
    drop(temp2); 
    println!("Manually dropped file");
    
    Ok(())
}

pub fn drop_examples() {
    drop_scope_demo();
    println!();
    drop_tempfile_demo().unwrap();
}