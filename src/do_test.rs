use std::{
    fs::File,
    io::{self, BufReader, BufWriter},
};

use crate::mystructs::{Answer, Question, QuestionType};

pub fn do_test(questions_file_path: &str, answer_file: &str) -> io::Result<()> {
    let questions_file = File::open(questions_file_path)?;
    let reader = BufReader::new(questions_file.try_clone().unwrap());
    let questions: Vec<Question> = serde_json::from_reader(reader)?;

    let mut answers = Vec::new();

    for (i, question) in questions.iter().enumerate() {
        println!(
            "Question {}: {}",
            i + 1,
            question.question.replace("<p>", "").replace("</p>", "")
        );

        match question.question_type {
            QuestionType::Oe => {
                println!("Type: Open-ended ({} Points) \n\nWrite Answer:\n", question.points);
                let mut answer = String::new();
                io::stdin().read_line(&mut answer)?;
                answers.push(Answer {
                    question_type: QuestionType::Oe,
                    answer: Some(answer.trim().to_string()),
                    options: None,
                });
            }
            QuestionType::Sc => {
                println!("Type: Open-ended ({} Points)", question.points);
                for (index, option) in question.options.iter().enumerate() {
                    println!("{}: {}", index + 1, option.answer.replace("<p>", "").replace("</p>", ""));
                }
                println!("\n\nSelect Option ie 1:\n");
                let mut answer = String::new();
                loop {
                    io::stdin().read_line(&mut answer)?;
                    let index = answer.trim().parse::<u8>();
                    if let Ok(index) = index {
                        if index > 0 && (index as usize) <= question.options.len() {
                            answers.push(Answer {
                                question_type: QuestionType::Sc,
                                answer: None,
                                options: Some(vec![index - 1]),
                            });
                            break;
                        }
                    }
                    answer.clear();
                    println!("Invalid input. Please enter a valid option number.");
                }
            }
            QuestionType::Mc => {
                println!("Type: Multiple Choice (Enter all applicable option numbers as a single string, e.g., 13 for options 1 and 3)");
                for (index, option) in question.options.iter().enumerate() {
                    println!("{}: {}", index + 1, option.answer.replace("<p>", "").replace("</p>", ""));
                }
                println!("\n\nSelect Option ie 13:\n");
                let mut answer = String::new();
                loop {
                    io::stdin().read_line(&mut answer)?;
                    let indexes: Vec<u8> = answer
                        .trim()
                        .chars()
                        .filter_map(|c| c.to_digit(10).map(|n| n as u8 - 1))
                        .collect();
                    if indexes
                        .iter()
                        .all(|&i| (i as usize) < question.options.len())
                    {
                        answers.push(Answer {
                            question_type: QuestionType::Mc,
                            answer: None,
                            options: Some(indexes),
                        });
                        break;
                    }
                    answer.clear();
                    println!("Invalid input. Please enter valid option numbers.");
                }
            }
        }

        println!(); // Print a blank line for better readability between questions
    }

    let output_file = File::create(answer_file)?;
    let writer = BufWriter::new(output_file);

    serde_json::to_writer_pretty(writer, &answers)?;

    Ok(())
}
