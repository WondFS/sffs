use crate::inode::inode;
use crate::inode::inode_manager;
use crate::common::directory;

// Copy the next element from path into name
pub fn skip_elem(path: &str) -> (String, String) {
    let mut index = 0;
    let mut temp_index = 0;
    let mut len = 0;
    for c in path.chars() {
        if c != '/' {
            break;
        }
        index += 1;
    }
    if index == path.len() {
        return ("".to_string(), "".to_string())
    }
    temp_index = index;
    while path[index..index+1] != '/'.to_string() {
        index += 1;
        if index == path.len() {
            break;
        }
    }
    len = index - temp_index;
    for c in path[index..].chars() {
        if c != '/' {
            break;
        }
        index += 1;
    }
    (path[index..].to_string(), path[temp_index..temp_index+len].to_string())
}

pub fn name_x(i_manager: &mut inode_manager::InodeManager, path: &str, name_i_parent: bool) -> Option<inode_manager::InodeLink> {
    let mut ip;
    if path.len() == 0 {
        return None;
    }
    if path[0..0] == '/'.to_string() {
        ip = i_manager.i_get(1).unwrap();
    } else {
        return None;
    }

    let mut temp = (path.to_string(), "".to_string());
    // let mut path = path;
    // let mut name = "";
    loop {
        temp = skip_elem(&temp.1);

        ip.borrow_mut().lock.lock();
        if ip.borrow().file_type != inode::InodeFileType::Directory {
            return None;
        }
        if name_i_parent && path == "" {
            return None;
        }
        let res = directory::dir_lookup(i_manager, ip.clone(), temp.1.clone());
        if res.is_some() {
            ip = res.unwrap().0;
        }

        if path == "" {
            break;
        }
    }
    todo!()
}

pub fn name_i(i_manager: &mut inode_manager::InodeManager, path: &str) -> inode_manager::InodeLink {
    name_x(i_manager, "", false).unwrap()
}

pub fn name_i_parent(i_manager: &mut inode_manager::InodeManager, path: &str) -> inode_manager::InodeLink {
    name_x(i_manager, "", false).unwrap()
}


#[cfg(test)]
mod tests {
    use crate::common::path::skip_elem;

    #[test]
    fn test_skip_elem() {
        let res1 = skip_elem("a/bb/c");
        let res2 = skip_elem("///a//bb");
        let res3 = skip_elem("a");
        let res4 = skip_elem("");
        let res5 = skip_elem("////");
        assert_eq!("bb/c", res1.0);
        assert_eq!("a", res1.1);
        assert_eq!("bb", res2.0);
        assert_eq!("a", res2.1);
        assert_eq!("", res3.0);
        assert_eq!("a", res3.1);
        assert_eq!("", res4.0);
        assert_eq!("", res4.1);
        assert_eq!("", res5.0);
        assert_eq!("", res5.1);
    }

}