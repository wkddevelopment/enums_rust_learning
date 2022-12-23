// Link: https://rust-lang-de.github.io/rustbook-de/ch06-01-defining-an-enum.html

#[derive(Debug)]
enum Auto {
    Skoda(String, u32),
    Honda(String, u32),
}

fn main() { 
    
    let mein_auto = Auto::Honda(String::from("Civic CRX"), 2); // Auto::Honda() ist ein Funktionsaufruf der einen String sowie einen u32 entgegen nimmt und eine Instanz von Auto konstruiert

    let dein_auto = Auto::Skoda(String::from("200 SL"), 5);

    println!("Meine Automarke ist: {:?}", mein_auto);
    println!("Deine Automarke ist: {:?}", dein_auto);

}


// ? Version mit struct
/*
#[derive(Debug)]
struct Auto {
    hersteller: Marke,
    modell: String,
}
#[derive(Debug)]
enum Marke {
    Skoda,
    Honda
}

fn main() { 
    
    let mein_auto = Auto {
        hersteller: Marke::Honda,
        modell: String::from("Civic CRX"),
    };

    let dein_auto = Auto {
        hersteller: Marke::Skoda,
        modell: String::from("200 SX"),
    };

    println!("Meine Automarke ist: {:?}. Das Modell ist: {:?}", mein_auto.hersteller, mein_auto.modell);
    println!("Deine Automarke ist: {:?}. Das Modell ist: {:?}", dein_auto.hersteller, dein_auto.modell);

}


*/