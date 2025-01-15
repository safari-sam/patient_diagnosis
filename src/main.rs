#[derive(Debug)]

#[derive(PartialEq)]
enum Symptom {
    Fever,
    Headache,
    SoreThroat,
    Cough,
    Fatigue,
    Chills,
    LooseStool,
    Vomiting,
    AbdominalPain,
}

#[derive(Debug)]
enum Illness {
    Malaria,
    CommonCold,
    Migraine,
    Pharyngitis,
    //GastroEnteritis,
    FoodPoisoning,
}


fn diagnose(symptoms: Vec<Symptom>) -> Vec<Illness> {
    let mut illnesses = Vec::new();

    if symptoms.contains(&Symptom::Fever) && symptoms.contains(&Symptom::Headache)
        && symptoms.contains(&Symptom::Chills)
    {
        illnesses.push(Illness::Malaria);
    }
    if symptoms.contains(&Symptom::Fever) && symptoms.contains(&Symptom::Cough) {
        illnesses.push(Illness::CommonCold);
    }
    if symptoms.contains(&Symptom::AbdominalPain) && symptoms.contains(&Symptom::LooseStool)
        && symptoms.contains(&Symptom::Vomiting)
    {
        illnesses.push(Illness::FoodPoisoning);
    }
    if symptoms.contains(&Symptom::Headache) {
        illnesses.push(Illness::Migraine);
    }
    if symptoms.contains(&Symptom::Fever) && symptoms.contains(&Symptom::SoreThroat) {
        illnesses.push(Illness::Pharyngitis);
    }

    illnesses
}

fn parse_symptoms(input: &str) -> Vec<Symptom> {
    let mut symptoms = Vec::new();

    for symptom in input.split(',') {
        match symptom.trim().to_lowercase().as_str() {
            "fever" => symptoms.push(Symptom::Fever),
            "headache" => symptoms.push(Symptom::Headache),
            "sore throat" => symptoms.push(Symptom::SoreThroat),
            "cough" => symptoms.push(Symptom::Cough),
            "fatigue" => symptoms.push(Symptom::Fatigue),
            "chills" => symptoms.push(Symptom::Chills),
            "loose stool" => symptoms.push(Symptom::LooseStool),
            "vomiting" => symptoms.push(Symptom::Vomiting),
            "abdominal pain" => symptoms.push(Symptom::AbdominalPain),
            _ => println!("Unknown symptom: {}", symptom.trim()),
        }
    }

    symptoms
}

use std::io;

fn main() {
    println!("Welcome to the Symptom Checker!");
    println!("Enter your symptoms as a comma-separated list (e.g., fever, headache, cough):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let symptoms = parse_symptoms(&input);
    if symptoms.is_empty() {
        println!("No valid symptoms entered. Please try again.");
        return;
    }

    let diagnoses = diagnose(symptoms);
    if diagnoses.is_empty() {
        println!("No matching illnesses found. Please consult a doctor.");
    } else {
        println!("Possible Diagnoses:");
        for illness in diagnoses {
            println!("- {:?}", illness);
        }
    }
}
