struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    fn calculate_total_cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    let hp = Laptop {
        brand: String::from("HP"),
        price: 650000,
    };

    let ibm = Laptop {
        brand: String::from("IBM"),
        price: 755000,
    };

    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        price: 550000,
    };

    let dell = Laptop {
        brand: String::from("Dell"),
        price: 850000,
    };

    let total_cost_hp = hp.calculate_total_cost(3);
    let total_cost_ibm = ibm.calculate_total_cost(3);
    let total_cost_toshiba = toshiba.calculate_total_cost(3);
    let total_cost_dell = dell.calculate_total_cost(3);

    let overall_total_cost = total_cost_hp + total_cost_ibm + total_cost_toshiba + total_cost_dell;

    println!("Total cost for 3 laptops from each brand: {}", overall_total_cost);
}
