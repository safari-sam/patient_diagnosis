use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Diagnosis {
    diagnosis: String,
    symptoms: Vec<String>,
    recommended_action: String,
}



use std::fs;

fn load_data(file_path: &str) -> Vec<Diagnosis> {
    let file_content = fs::read_to_string(file_path)
        .expect("Failed to read JSON file");
    serde_json::from_str(&file_content)
        .expect("Failed to parse JSON data")
}

fn diagnose(symptoms: Vec<String>, data: &[Diagnosis]) -> Vec<(String, f64, String)> {
    let mut matching_diagnoses = Vec::new();

    for diagnosis in data {
        let matched_symptoms = diagnosis
            .symptoms
            .iter()
            .filter(|symptom| symptoms.contains(&symptom.to_lowercase()))
            .count();

        if matched_symptoms > 0 {
            let total_symptoms = diagnosis.symptoms.len();
            let match_percentage = (matched_symptoms as f64 / total_symptoms as f64) * 100.0;

            matching_diagnoses.push((
                diagnosis.diagnosis.clone(),
                match_percentage,
                diagnosis.recommended_action.clone(),
            ));
        }
    }


    matching_diagnoses.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    
    matching_diagnoses.into_iter().take(2).collect()
}



fn main() {
    let data = load_data("data.json"); 

    println!("Welcome to the Symptom Checker!");
    println!("The program helps you to diagnose your symptoms and recommends action to take.");
    println!("The program diagnose basic symptoms and recommends actions that can be taken at home, it is important to seek further medical advice if symptoms persists");
    println!("please enter your symptoms as a comma-separated list (e.g., fever, headache):");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");

    let symptoms: Vec<String> = input
        .trim()
        .split(',')
        .map(|s| s.trim().to_lowercase())
        .collect();

    if symptoms.is_empty() {
        println!("No valid symptoms entered. Please try again.");
        return;
    }

    let diagnoses = diagnose(symptoms, &data);

    if diagnoses.is_empty() {
        println!("your symptoms did not match a diagnosis in our data base, please consult a doctor for further evaluation and treatment.");
    } else {
        println!("your symptoms matched:");
        for (diagnosis, percentage, action) in diagnoses {
            println!(
                "- {}: {:.2}% match. Recommended action: {}",
                diagnosis, percentage, action
            );
        }
    }
}
