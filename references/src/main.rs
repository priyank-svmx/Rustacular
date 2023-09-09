
use std::collections::HashMap;
type Table = HashMap<String,Vec<String>>;


fn show(table: Table){
    for(artist, works) in table {
        println!("works by artist {}", artist);
        for work in works {
            println!("{}", work);
        }
        println!("****************");
    }
}


fn main() {
    println!("refrences the first program ....");
   let mut table =  Table::new();
    table.insert(
                    "Gesualdo".to_string(),
                    vec!["many madrigals".to_string(),"Tenebria responsoria".to_string()]
                );
    table.insert(
        "Caravaggio".to_string(),
        vec!["the musicians".to_string(),"the calling at mathew".to_string() ]
    );
    table.insert(
        "cellini".to_string(),
        vec!["Persus with the head of medusa".to_string()]
    );
    show(table);
    println!("show table {}", table["Gesualdo"][0]);


}
