//! JUST A SNIPPET
//! FIND IT'S PLACE IN MAIN.RS

let mut currmap: Vec<Map> = vec![];
for r in ranges {
    let mut found = false;
    for m in &lastmap {
        if m.destination <= r.source && m.destination + m.range > r.source {
            found = true;
            println!("moved: {} {} {}", m.destination, m.destination + m.range, r.source);
            currmap.push(Map { destination: r.source + m.destination, source: m.destination + r.source, range: r.range });
        }
    }
    if !found {
        println!("stayed: {} {}", r.source, r.source + r.range);
        currmap.push(r);
    }
}
println!("{:?}", currmap);
lastmap = currmap.clone();