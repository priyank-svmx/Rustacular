use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    for (name, works) in table {
        println!("work of the person {}", name);
        for work in works {
            println!("{}", work);
        }

        println!("************");
    }
}

fn main() {
    println!("shared and mutable refrences.");
    println!("##################");

    let mut table = Table::new();
    table.insert(
        "michael".to_string(),
        vec!["cmd".to_string(), "cli".to_string()],
    );
    table.insert(
        "schaff".to_string(),
        vec!["dsl".to_string(), "packer".to_string()],
    );

    let _copy = &table;

    show(&table);
    println!("{}", table["michael"][0]);
    table.insert("meta boy".to_string(), vec!["rool Up".to_string()]);
//    _copy.insert("magnificient".to_string(), vec!["tool kit".to_string()]);
    println!("{}", _copy["meta boy"][0]);
}
