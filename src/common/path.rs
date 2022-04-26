use crate::inode::inode;
use crate::inode::inode_manager;
use crate::common::directory;

// Copy the next element from path into name
pub fn skip_elem(path: String) -> (String, String) {
    let path = path.as_str();
    let mut index = 0;
    let temp_index;
    let len;
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

// Look up and return the inode for a path name.
pub fn name_x(i_manager: &mut inode_manager::InodeManager, path: String, name: &mut String, name_i_parent: bool) -> Option<inode_manager::InodeLink> {
    let path = &mut path.clone();
    let mut ip;
    let mut next;
    if path.len() == 0 {
        return None;
    }
    if path[0..0] == '/'.to_string() {
        ip = i_manager.i_get(1).unwrap();
    } else {
        return None;
    }
    loop {
        (*path, *name) = skip_elem(path.clone());
        if path == "" {
            break;
        }
        if ip.borrow().file_type != inode::InodeFileType::Directory {
            return None;
        }
        if name_i_parent && path == "" {
            return Some(ip);
        }
        let res = directory::dir_lookup(i_manager, ip.clone(), name.to_string());
        if res.is_some() {
            next = res.unwrap().0;
        } else {
            return None;
        }
        ip = next;
    }
    if name_i_parent {
        i_manager.i_put(ip);
        return None;
    }
    return Some(ip);
}

pub fn name_i(i_manager: &mut inode_manager::InodeManager, path: String) -> inode_manager::InodeLink {
    let mut name = "".to_string();
    name_x(i_manager, path, &mut name, true).unwrap()
}

pub fn name_i_parent(i_manager: &mut inode_manager::InodeManager, path: String, name: &mut String) -> inode_manager::InodeLink {
    name_x(i_manager, path, name, false).unwrap()
}


#[cfg(test)]
mod tests {
    use crate::common::path::skip_elem;

    #[test]
    fn test_skip_elem() {
        let res1 = skip_elem("a/bb/c".to_string());
        let res2 = skip_elem("///a//bb".to_string());
        let res3 = skip_elem("a".to_string());
        let res4 = skip_elem("".to_string());
        let res5 = skip_elem("////".to_string());
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