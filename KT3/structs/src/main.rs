struct Animal 
{
    name: String,
    sientific_name: String,
    time_period: String,
    diet: String,
    region: String,
    extinct: bool,
}

fn main() 
{
    let animals =[
        Animal {
            name: String::from("T-Rex"),
            sientific_name: String::from("Tyrannosaurus"),
            time_period: String::from("Jurassic Period"),
            diet: String::from("Carnivore"),
            region: String::from("North America"),
            extinct: true,
        },

        Animal {
            name: String::from("Stegosaurus"),
            sientific_name: String::from("Stegosaurioni"),
            time_period: String::from("Clasc Period"),
            diet: String::from("Herbivore"),
            region: String::from("Asia"),
            extinct: true,
        },

        Animal {
            name: String::from("gray wolf"),
            sientific_name: String::from("Doggo"),
            time_period: String::from("ice age"),
            diet: String::from("Carnivore"),
            region: String::from("Europe"),
            extinct: false,
        },
    ];

    for animal in animals
    {
        println!("Name: {}", animal.name);
        println!("Sientific_name: {}", animal.sientific_name);
        println!("Time period: {}", animal.time_period);
        println!("Diet: {}", animal.diet);
        println!("Region: {}", animal.region);
        println!("Extinct: {}", animal.extinct);

        println!("");
    }
}
