struct File {
    size: u32,
    name: String,
}

enum DirectoryObject {
    File(File),
    Directory(Directory),
}
struct Directory {
    name: String,
    objects: Vec<DirectoryObject>,
}
impl Directory {
    fn add_object(&mut self, object: DirectoryObject) {
        match &object {
            DirectoryObject::Directory(dir) => {
                println!("adding dir {} to dir {}", dir.name, self.name);
            }
            DirectoryObject::File(file) => {
                println!("adding file {} to dir {}", file.name, self.name);
            }
        }
        self.objects.push(object);
    }

    fn calculate_size(&self) -> u32 {
        let mut final_size = 0;
        for object in &self.objects {
            match &object {
                DirectoryObject::Directory(dir) => {
                    final_size += dir.calculate_size();
                }
                DirectoryObject::File(file) => final_size += file.size,
            }
        }
        final_size
    }
}
