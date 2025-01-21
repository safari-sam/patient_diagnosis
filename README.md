#Symptom_Check


A "Rust-based CLI tool" designed to match patient symptoms with potential diagnoses dynamically. The program uses JSON-based data and ranks diagnoses by likelihood, ensuring accurate results by requiring key symptoms for certain diagnoses.
the tool is still in its basic form but the main goal is to create a program that helps users to quickly find a solution to the symptoms they might have by recommending medications to take, seek medical attention or basic first aid before professional intervention. 
the program will also feature a tele-medicine system approach to users by adding a feature of quickly accesing a healthcare profession for further consultations, and also an online pharmacy where users can quickly order the recommended drugs. all these will be done remotely at the comfort of the user's home. 
with all said, this program can quickly be adopted by an online retail pharmacy, research institutions and hospitals. 

---

Features
- Matches symptoms to diagnoses dynamically using a JSON database.
- Ranks diagnoses by the percentage of symptoms matched.
- Ensures key symptoms are present for accurate results.
- Displays recommended actions for each diagnosis.
- Flexible and extensible—easily add or modify diagnoses and symptoms in the JSON file.

Features to Implement: 
Remote Doctor Consultation.

    Enables users to:
        Initiate a consultation with a doctor via video or chat.
        Schedule appointments if doctors are unavailable in real time.
    Use cases:
        Discuss matched diagnoses.
        Seek advice for severe or complex symptoms.
    Tech Stack Options:
        Use APIs for video calls (e.g., Zoom API.)
        Implement real-time chat.

Online Pharmacy

    Match symptoms to over-the-counter or prescribed medications.
    Allow patients to:
        Browse medication recommendations for their symptoms.
        Order medications through an integrated delivery system.
        Track medication orders.
    Tech Stack Options:
        Integrate with third-party pharmacy APIs or build a custom inventory and order management system.
        Use secure payment gateways for online transactions.

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
