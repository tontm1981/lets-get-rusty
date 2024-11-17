fn main() {
    let students = vec![
        "Elton 4.8",
        "Wallace 2.3",
        "Lidiya 2.8",
        "Kyke 3.5",
        "Anatoly 4.0",
    ];

/**
    let mut good_students = vec![];
    for s in students {
        let mut s = s.split(' ');
        let name = s.next();
        let gpa = s.next();

        if let (Some(name), Some(gpa)) = (name, gpa) {
            let name = name.to_owned();
            let gpa = gpa.parse::<f32>();

            if let Ok(gpa) = gpa {
                if gpa >= 3.5 {
                    good_students.push(Student {name, gpa});
                }
            }
        }
    }
*/

/*
    let good_student: Vec<Student> = students
        .iter()
        .map(|s| {
            let mut s = s.split(' ');
            let name = s.next()?.to_owned();
            let gpa = s.next()?.parse::<f32>().ok()?;

            Some(Student { name, gpa })
        })
        .flatten()
        .filter(|s| s.gpa > 3.5)
        .collect();
*/

    let good_students: Vec<Student> = students
        .iter()
        .filter_map(|s| {
            let mut s = s.split(' ');
            let name = s.next()?.to_owned();
            let gpa = s.next()?.parse::<f32>().ok()?;

            if gpa < 3.5 {
                return None;
            }

            Some(Student { name, gpa })
        })
        .collect();

    for s in good_students {
        println!("{s:?}");
    }
}

#[derive(Debug)]
struct Student {
    name: String,
    gpa: f32,
}