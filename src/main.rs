// mod pokemon;

// use pokemon::{fetch_pokemon_data, Pokemon};
use rustemon::Follow;
use rustemon::client::RustemonClient;
use rustemon::model::evolution::EvolutionDetail;
use rustemon::model::pokemon::Pokemon;
use std::env;
use tokio;
use clap::Parser;
use titlecase::titlecase;


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

// Format evolution details into a string that's pretty on the eyes.
fn evolution_detail_to_str(evolution_name:String, detail:EvolutionDetail) -> String {
    let mut output = "Method: ".to_owned() + &titlecase(&&detail.trigger.name) + "\n";

    // Unique evolution methods
    output += match evolution_name.as_str() {
        "pawmot" => "Walk 1000 steps using Let's Go! and level up outside Pokeball\n",
        "maushold" => "Level up in a battle, 1/100 chance of Family of Three form\n",
        "brambleghast" => "Walk 1000 steps using Let's Go! and level up outside Pokeball\n",
        "rabsca" => "Walk 1000 steps using Let's Go! and level up outside Pokeball\n",
        "palafin" => "Evolves at level 38 when connected to another player via the Union Circle\n",
        "kingambit" => "Level up after defeating 3 Bisharp that hold a Leader's Crest\n",
        "gholdengo" => "Level up while the player has 999 Gimmighoul Coins\n",
        &_ => ""
    };
    match detail.trigger.name.as_str(){
        "use-item" => output += &("Use ".to_string() + &titlecase(&&detail.item.unwrap().name) + "\n"),
        &_ => {
            if detail.gender.is_some(){ 
                let gd = match detail.gender {
                    Some(0) => "Female",
                    Some(1) => "Male",
                    Some(2) => "Unspecified",
                    _ => "N/A"
                };
                output += &("Gender: ".to_owned() + gd + "\n");
            };

            if detail.min_level.is_some(){
                output += &("At level ".to_owned() + &detail.min_level.unwrap().to_string() + "\n");

            };
            if detail.held_item.is_some(){
                output += &("Must be holding ".to_owned() + &titlecase(&&detail.held_item.unwrap().name) + "\n");
            };
            if detail.known_move.is_some(){
                output += &("Must know ".to_owned() + &titlecase(&detail.known_move.unwrap().name) + "\n");
            };
            if detail.known_move_type.is_some(){
                output += &("Must know a ".to_owned() + &titlecase(&&detail.known_move_type.unwrap().name) + " type move\n");

            };
            if detail.location.is_some(){
                output += &("At location: ".to_owned() + &titlecase(&&detail.location.unwrap().name) + "\n");

            };
            if detail.min_happiness.is_some(){
                output += &("With happiness at least ".to_owned() + &detail.min_happiness.unwrap().to_string() + "\n");

            };
            if detail.min_beauty.is_some(){
                output += &("With beauty at least ".to_owned() + &detail.min_beauty.unwrap().to_string() + "\n");

            };
            if detail.min_affection.is_some(){
                output += &("With affection at least ".to_owned() + &detail.min_affection.unwrap().to_string() + "\n");

            };
            if detail.party_species.is_some(){
                output += &("With a ".to_owned() + &titlecase(&&detail.party_species.unwrap().name) + " in your party\n");

            };
            if detail.party_type.is_some(){
                output += &("With a ".to_owned() + &titlecase(&&detail.party_type.unwrap().name) + " type pokemon in your party\n");

            };
            if detail.relative_physical_stats.is_some(){
                let stats = match detail.relative_physical_stats {
                    Some(0) => "Attack = Defense",
                    Some(1) => "Attack > Defense",
                    Some(2) => "Attack < Defense",
                    _ => "N/A"
                };

                output += &("While ".to_owned() + stats + "\n");

            };
            if detail.trade_species.is_some(){
                output += &("If traded for a ".to_owned() + &titlecase(&&detail.trade_species.unwrap().name) + "\n");

            };
            if detail.needs_overworld_rain == true {
                output += "Needs overworld rain\n";
            };
            if detail.turn_upside_down == true {
                output += "With the console turned upside down\n";
            };

            if detail.time_of_day.len() > 0 {
                output += &("While it's ".to_owned() + &detail.time_of_day + "time\n");
            }


        }
    }
    output

}


// Consider using '?' syntax instead of unwrap for more flexibility
async fn get_evolution_data(pokemon: &Pokemon, rustemon_client: &RustemonClient) -> String {

    // Follow the resource link to get the pokemon's species
    let species_resource = &pokemon.species;
    let species = species_resource.follow(rustemon_client).await;

    // Get the evolution chain from the species
    let evolution_chain = species
    .unwrap()
    .evolution_chain
    .unwrap()
    .follow(&rustemon_client)
    .await;

    // dbg!(&evolution_chain);
    let mut data = "".to_string();
    let mut current_link = &evolution_chain.unwrap().chain;

    // If chain species and species are not the same,
    // set current link to the link in evolves_to and repeat the process
    // until we have the pokemon we want

    // let mut evolves_to = &current_link.evolves_to;
    
    while current_link.species != *species_resource {
        if current_link.evolves_to.is_empty() {
            break;
        }
        // evolves_to.iter().for_each(|x| {
        //     if x.species == *species_resource {current_link = x};
        // });
        current_link = &current_link.evolves_to[0];

    }
    
    // If chain-species and species are the same,
    // Return evolves-to species and non-blank evolution triggers including item
    if current_link.species == *species_resource {
        // dbg!(&current_link.species);
        let evolves_to = &current_link.evolves_to;

        if evolves_to.is_empty() {
            // Pokemon Doesn't evolve
            data = format!("{} does not evolve.", pokemon.name);
        } else {
            // Thanks Rhea from recurse for the tip here!!
            // Flat mapping the evolution details to 
            let evolutions = evolves_to.iter().flat_map(|x| {
            let identifier = x.species.name.clone();
            x.evolution_details.iter().map(move|y| {
                (identifier.clone(), y.clone())
            })});

            evolutions.for_each(|(identifier, detail)|{
                data.push_str(&format!("{} evolves into {} \n{}\n", &titlecase(&pokemon.name), &titlecase(&identifier), evolution_detail_to_str(identifier.clone(), detail)));
            });
        }
    }
    data
}
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let pokemon_name = &args[1];
        let rustemon_client = rustemon::client::RustemonClient::default();
        let pokemon = rustemon::pokemon::pokemon::get_by_name(&pokemon_name, &rustemon_client).await;

        println!("{}", get_evolution_data(&pokemon.unwrap(), &rustemon_client).await);


    } else {
        eprintln!("Usage: pokeapi_rust <pokemon_name>");
    }
}
