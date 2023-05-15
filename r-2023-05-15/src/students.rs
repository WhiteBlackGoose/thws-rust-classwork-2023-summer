pub fn read_students(path: &str) -> Vec<Vec<String>> {
    let mut rdr = csv::Reader::from_path(path).unwrap();
    let mut res = vec![];
    for r in rdr.records() {
        res.push(r.unwrap().iter().map(|i| String::from(i)).collect());
    }
    res
}

fn print_ids(students: &mut Vec<Vec<String>>) {
    let mut ids: Vec<String> = students.iter().map(|s| s[0].clone()).collect();
    ids.sort();
    for id in ids {
        println!("{}", id);
    }
}

fn youngest(students: &mut Vec<Vec<String>>) {
    let youngest_found = students.iter().min_by_key(|x| x[3].clone());
    let mut iter = youngest_found.iter();
    let youngest = iter.next().unwrap();
    assert!(iter.next().is_none());
    println!("{}", youngest[2]);
}
