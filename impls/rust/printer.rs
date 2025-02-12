use crate::types::MalType;

fn pr_str(tree: MalType) -> String {
    match tree {
        MalType::MalList(list) => {
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
        MalType::MalInteger(number) => return number.to_string(),
        MalType::MalSymbol(sym) => return sym,
    }
}
