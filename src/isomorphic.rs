use std::collections::HashMap;

#[allow(unused_variables)]
fn main() {
    let (s, t) = ("egg".to_string(), "add".to_string());
    let (s, t) = ("foo".to_string(), "bar".to_string());
    let (s, t) = ("paper".to_string(), "title".to_string());
    // let (s, t) = ("badc".to_string(), "baba".to_string());

    println!("Result -> {}", is_isomorphic(s, t));
}

fn is_isomorphic(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut s_map: HashMap<char, char> = HashMap::new();
    let mut t_map: HashMap<char, char> = HashMap::new();

    for (sc, tc) in s.chars().zip(t.chars()) {
        println!("{s_map:?} - {t_map:?}");
        match (s_map.get(&sc), t_map.get(&tc)) {
            (None, None) => {
                s_map.insert(sc, tc);
                t_map.insert(tc, sc);
            }
            (Some(&rsc), Some(&rtc)) if sc == rtc && tc == rsc => (),
            _ => return false,
        };
    }

    true
}
