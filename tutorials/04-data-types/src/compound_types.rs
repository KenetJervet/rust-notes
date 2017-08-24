pub fn compound_types() {
    println!("Compound types");
    tuples();
    arrays();
}

fn tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple: {:?}", tup);
    let (x, y, z) = tup;
    println!("Y is: {}", y);
}

fn arrays() {
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December"
    ];
    let feb = months[1];
    println!("Feb: {}", feb);
    // The following statement will cause panic (index out of bounds)
    // println!("Feb: Thirteenth month: {}", months[x + 1]);
}
