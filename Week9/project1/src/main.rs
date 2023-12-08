use std::io::Write;
fn main() {

    let lager =["33 Export".to_string(), "Desperados".to_string(), "Goldberg".to_string(), "Gulder".to_string(), "Heineken".to_string() , "Star".to_string()];
    let stout = ["Legend".to_string(), "Turbo King".to_string(), "Williams".to_string()];
    let non_Alchoholic = ["Maltina".to_string(), "Amstel Malt".to_string(), "Malta Gold".to_string(), "Fayrouz".to_string()];

    
    let drinks = [&lager[..],&stout[..],&non_Alchoholic[..]];
    let drinkname = ["lager", "stout" , "non_Alchoholic"];

    for x in 0..3{
        let mut file = std::fs::File::create(format!("{}.txt",drinkname[x])).expect("create failed");
        file.write_all(format!("{}:\n", drinkname[x]).as_bytes()).expect("write failed");

        for item in drinks[x]{
            file.write_all(format!("{},\n", item).as_bytes()).expect("Failed to write");
        }
    }

    println!("Task completed succesfully");

}
