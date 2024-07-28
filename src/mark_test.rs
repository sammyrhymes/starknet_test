use std::{fs::File, io::{self, BufReader}};

use crate::mystructs::{Answer, Question, QuestionOption, QuestionType};

fn calculate_points(options: &[QuestionOption], selected_answers: &[u8], total_points: f32) -> f32 {
    let correct_options: Vec<&QuestionOption> =
        options.iter().filter(|option| option.is_correct).collect();
    let _incorrect_options: Vec<&QuestionOption> =
        options.iter().filter(|option| !option.is_correct).collect();

    let points_per_correct_option = total_points / correct_options.len() as f32;
    let penalty_per_incorrect_option = total_points / options.len() as f32; // Simple penalty for each incorrect answer

    let mut points_scored = 0.0;

    for &selected_index in selected_answers {
        if let Some(selected_option) = options.get(selected_index as usize) {
            if selected_option.is_correct {
                points_scored += points_per_correct_option;
            } else {
                points_scored -= penalty_per_incorrect_option;
            }
        }
    }

    points_scored.max(0.0)
}

pub fn mark_test(questions_file_path: &str, answers_file_path: &str) -> io::Result<()> {
    let questions_file = File::open(questions_file_path)?;
    let answers_file = File::open(answers_file_path)?;

    let questions_reader = BufReader::new(questions_file);
    let answers_reader = BufReader::new(answers_file);

    let questions: Vec<Question> = serde_json::from_reader(questions_reader)?;
    let answers: Vec<Answer> = serde_json::from_reader(answers_reader)?;

    let mut total_score = 0.0;

    for (i, question) in questions.iter().enumerate() {
        println!("Question {}: {}", i + 1, question.question);

        let answer = &answers[i];
        match question.question_type {
            QuestionType::Oe => {
                if let Some(ref ans) = answer.answer {
                    println!("Answer: {}", ans);
                    println!("Score (out of {}): ", question.points);
                    let mut score = String::new();
                    io::stdin().read_line(&mut score)?;
                    let score: f32 = score.trim().parse().unwrap_or(0.0);
                    total_score += score;
                }
            }
            QuestionType::Sc => {
                if let Some(ref selected_options) = answer.options {
                    let selected_option = selected_options[0];
                    println!(
                        "Selected Answer: {}",
                        question.options[selected_option as usize].answer
                    );
                    if question.options[selected_option as usize].is_correct {
                        total_score += question.points;
                        println!("Score: {}", question.points);
                    } else {
                        println!("Score: 0.0");
                    }
                }
            }
            QuestionType::Mc => {
                if let Some(ref selected_options) = answer.options {
                    println!(
                        "Selected Answers: {:?}",
                        selected_options
                            .iter()
                            .map(|&i| &question.options[i as usize].answer)
                            .collect::<Vec<_>>()
                    );
                    let score =
                        calculate_points(&question.options, selected_options, question.points);
                    total_score += score;
                    println!("Score: {}", score);
                }
            }
        }

        println!(); // Print a blank line for better readability between questions
    }

    println!("Total Score: {}", total_score);

    Ok(())
}
