#[derive(Clone)]
struct File {
    size: u32,
    name: String,
}
impl File {
    fn new(size: u32, name: &str) -> File {
        File {
            size: size,
            name: name.to_string(),
        }
    }
}

#[derive(Clone)]

enum DirectoryObject {
    File(File),
    Directory(Directory),
}

#[derive(Clone)]

struct Directory {
    name: String,
    objects: Vec<DirectoryObject>,
}
impl Directory {
    fn new(name: &str) -> Directory {
        Directory {
            name: name.to_string(),
            objects: Vec::default(),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::Directory;
    use super::*;

    #[test]
    fn work_with_empty_directory() {
        let dir = Directory::new("empty");

        let size = dir.calculate_size();
        assert_eq!(size, 0)
    }

    #[test]
    fn work_with_one_file() {
        let mut dir = Directory::new("single");
        let file = File::new(10, "basic file");
        dir.add_object(DirectoryObject::File(file));
        let size = dir.calculate_size();
        assert_eq!(size, 10)
    }
    #[test]
    fn work_with_multiples_file() {
        let mut dir = Directory::new("multiple");
        let test_file = File::new(10, "basic file");
        dir.add_object(DirectoryObject::File(test_file));
        let test_file = File::new(10, "basic file");
        dir.add_object(DirectoryObject::File(test_file));

        let size = dir.calculate_size();
        assert_eq!(size, 20)
    }
    #[test]
    fn work_with_nested_directory() {
        let mut dir = Directory::new("nested");
        let test_file = File::new(10, "basic file");
        dir.add_object(DirectoryObject::File(test_file));
        let mut test_dir = Directory::new("single");
        let test_file = File::new(50, "basic file");
        test_dir.add_object(DirectoryObject::File(test_file));

        dir.add_object(DirectoryObject::Directory(test_dir.clone()));

        assert_eq!(test_dir.calculate_size(), 50);
        assert_eq!(dir.calculate_size(), test_dir.calculate_size() + 10);
    }
}
