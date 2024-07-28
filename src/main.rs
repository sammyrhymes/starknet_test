use std::io;

pub mod mystructs;
pub mod mark_test;
pub mod do_test;

use do_test::do_test;

fn main() -> io::Result<()> {
    println!("\nDoing Test for Smart Contracts and Frontend Topics\n");
    let _ = do_test("src/topic4_5_questions.json", "src/topic_4_5_test_answers.json");

    println!("\nDoing Comprehensive test for all the topics covered\n");
    let _ = do_test("src/comprehensive_questions.json", "src/comprehensive_test_answers.json");
    Ok(())
}
