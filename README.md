#Symptom_Check


A "Rust-based CLI tool" designed to match patient symptoms with potential diagnoses dynamically. The program uses JSON-based data and ranks diagnoses by likelihood, ensuring accurate results by requiring key symptoms for certain diagnoses.


Features
- Matches symptoms to diagnoses dynamically using a JSON database.
- Ranks diagnoses by the percentage of symptoms matched.
- Ensures key symptoms are present for accurate results.
- Displays recommended actions for each diagnosis.
- Flexible and extensible—easily add or modify diagnoses and symptoms in the JSON file.

Benefits of the project 
with telemedicine feature, the project will provide: 
    Improved Accessibility: Patients can access healthcare professionals and medications from the comfort of their homes.
    Time-Saving: Reduces the time and effort required to visit a healthcare facility or pharmacy physically.
    Better Continuity of Care: Ensures patients receive timely consultations and prescribed medications without delays.
    Scalability: The project is positioned for broader adoption in various healthcare settings, including rural areas with limited access to physical healthcare facilities.

How It Works
1. Input Symptoms:
   - The user provides a list of symptoms as a comma-separated input (e.g., `fever, headache`).
   
2. Match Diagnoses:
   - The program calculates the percentage of symptoms matched for each diagnosis.
   - Only diagnoses with all required symptoms present are considered.

3. Display Results:
   - The top 2 most likely diagnoses are displayed with their match percentages and recommended actions.



Installation
Follow these steps to set up and run the project locally:

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/patient_diagnosis.git

Run program:
cargo run

Contributing:

contributions are welcomed. To contribute:

    Fork this repository.
    Create a new branch (git checkout -b feature-branch).
    Commit your changes (git commit -m "Add some feature").
    Push to the branch (git push origin feature-branch).
    Open a pull request.

contact: 
created by Samuel Safari Onyango. 
github: safari-sam
email: safarisamm51@gmail.com
