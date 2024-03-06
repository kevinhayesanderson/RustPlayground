pub fn create_new_vectors() {
    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];
}

pub fn updating_vectors() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}

pub fn reading_vectors() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    //let does_not_exist = &v[100]; //panics
    //println!("The element at index 100 is {does_not_exist}");
    let does_not_exist = v.get(100); //it returns None without panicking.

    match does_not_exist {
        Some(element) => println!("The element at index 100 is {element}"),
        None => println!("There is no  element."),
    }

    // Attempting to add an element to a vector while holding a reference to an item
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    //v.push(6);

    //println!("The first element is: {first}");
    //This error is due to the way vectors work:
    //because vectors put the values next to each other in memory,
    //adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space,
    //if there isn’t enough room to put all the elements next to each other where the vector is currently stored. In that case, the reference to the first element would be pointing to deallocated memory.
}

pub fn iterating_values() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        //To change the value that the mutable reference refers to, we have to use the * dereference operator to get to the value in i before we can use the += operator.
    }

    //Using an Enum to Store Multiple Types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for item in &row {
        match item {
            SpreadsheetCell::Int(int_value) => println!("{int_value}"),
            SpreadsheetCell::Float(float_value) => println!("{float_value}"),
            SpreadsheetCell::Text(string) => println!("{string}"),
            _ => println!("Unknown type"),
        }
    }
}
