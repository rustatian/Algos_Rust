use std::rc::Rc;
use std::borrow::Borrow;

struct Three {
    name: String,
    children: Option<Rc<Three>>,
}

impl Three {
    fn dfs(root: Option<Rc<Three>>) -> Vec<String> {
        let mut v = vec![];

        Three::dfs_recurse(&mut v, &root);

        v
    }

    fn dfs_recurse(array: &mut Vec<String>, root: &Option<Rc<Three>>) {
        if let Some(n) = root {
            array.push(n.name.clone());
            Three::dfs_recurse(array, &n.children);
        }
    }
}

#[test]
fn test() {
    let mut b = Three {
        name: "B".to_string(),
        children: None,
    };

    let mut n = Three {
        name: "A".to_string(),
        children: Option::from(Rc::new(b)),
    };
    assert_eq!(Three::dfs(Option::from(Rc::new(n))), vec!["A".to_string(), "B".to_string()]);
}
