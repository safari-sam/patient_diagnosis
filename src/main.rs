use std::collections::HashMap;
use serde::Deserialize;
use std::fs;
use std::io;

#[derive(Debug, Deserialize)]
struct SubQuestion {
    question: String,
    options: HashMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct Diagnosis {
    diagnosis: String,
    symptoms: Vec<String>,
    required_symptoms: Vec<String>,
    sub_questions: Option<HashMap<String, SubQuestion>>,
    recommended_action: String,
}

fn load_data(file_path: &str) -> Vec<Diagnosis> {
    let file_content = fs::read_to_string(file_path)
        .expect("Failed to read JSON file");
    serde_json::from_str(&file_content)
        .expect("Failed to parse JSON data")
}

fn main() {
    let data = load_data("data.json");

    println!("Welcome to the Symptom Checker!");
    println!("Enter your symptoms as a comma-separated list (e.g., fever, cough, chest pain):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let symptoms: Vec<String> = input
        .trim()
        .split(',')
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();

    if symptoms.is_empty() {
        println!("No valid symptoms entered. Please try again.");
        return;
    }

    let mut follow_ups: HashMap<String, String> = HashMap::new();
    let mut results: Vec<(String, f64, String)> = Vec::new();

    for diagnosis in &data {
        // Ask follow-up questions for each symptom if available
        if let Some(sub_questions) = &diagnosis.sub_questions {
            for symptom in &symptoms {
                if let Some(sub_question) = sub_questions.get(symptom) {
                    println!("\nFollow-up for '{}':", symptom);
                    println!("{}", sub_question.question);
                    let options: Vec<_> = sub_question.options.keys().collect();

                    for (i, option) in options.iter().enumerate() {
                        println!("{}. {}", i + 1, option);
                    }

                    let mut answer_input = String::new();
                    io::stdin().read_line(&mut answer_input).expect("Failed to read answer");

                    if let Ok(choice) = answer_input.trim().parse::<usize>() {
                        if let Some(selected_option) = options.get(choice - 1) {
                            follow_ups.insert(symptom.clone(), selected_option.to_string());
                            // Add the sub-diagnoses too
                            if let Some(sub_diagnoses) = sub_question.options.get(*selected_option) {
                                for diag in sub_diagnoses {
                                    if !diag.is_empty() {
                                        results.push((
                                            diag.clone(),
                                            100.0,
                                            "Further evaluation may be required.".to_string(),
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Regular diagnosis scoring
    for diagnosis in &data {
        let total_symptoms = diagnosis.symptoms.len();
        if total_symptoms == 0 {
            continue;
        }

        let matched_symptoms = diagnosis
            .symptoms
            .iter()
            .filter(|s| symptoms.contains(&s.to_lowercase()))
            .count();

        if matched_symptoms == 0 {
            continue;
        }

        let mut score = (matched_symptoms as f64 / total_symptoms as f64) * 100.0;

        let required_present = diagnosis
            .required_symptoms
            .iter()
            .all(|req| symptoms.contains(&req.to_lowercase()));

        if required_present && !diagnosis.required_symptoms.is_empty() {
            score += 10.0;
        }

        results.push((
            diagnosis.diagnosis.clone(),
            score,
            diagnosis.recommended_action.clone(),
        ));
    }

    if results.is_empty() {
        println!("\nNo matching diagnosis found. Please consult a healthcare provider.");
    } else {
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        println!("\nPossible diagnosis suggestions:");
        for (diagnosis, score, action) in results {
            println!(
                "- Diagnosis: {}\n  Match Confidence: {:.2}%\n  Recommended Action: {}\n",
                diagnosis, score, action
            );
        }
    }
}
