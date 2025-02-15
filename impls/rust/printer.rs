use crate::types::Mal;

pub fn pr_str(tree: Mal) -> String {
    match tree {
        Mal::List(list) => {
            let mut res = String::from("(");
            let joined = list
                .iter()
                .map(|item| pr_str(item.clone()))
                .collect::<Vec<String>>()
                .join(" ");
            res.push_str(&joined);
            res.push_str(")");
            return res;
        }
        Mal::Int(number) => return number.to_string(),
        Mal::Sym(sym) => return sym,
    }
}
