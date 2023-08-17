use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, near_bindgen, env};
use near_sdk::serde::Serialize;
use near_sdk::collections::{UnorderedMap};

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Answer {
     content: String,
     responder_id: AccountId
    
}
 
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Question {
     content: String,
     asker_id: AccountId,
     id: u128,
     answers: Vec<Answer> 
}


#[derive(BorshDeserialize, BorshSerialize)]
#[near_bindgen]
 pub struct ChatRoom {
     questions_asked: UnorderedMap<u128, Question>,
     number_of_questions: u128
}

impl Default for ChatRoom{
     fn default() -> Self {
         Self {
             questions_asked: UnorderedMap::new(b'm'),
             number_of_questions: 0000
         }
     }
}


#[near_bindgen]
impl ChatRoom {
     pub fn add_question(&mut self, content: String) -> Question {
         let question = Question {
            content: content,
            asker_id: env::signer_account_id(),
            id: self.number_of_questions,
            answers: Vec::<Answer>::new()
             
         };

         self.questions_asked.insert(&self.number_of_questions, &question);
         self.number_of_questions += 1;
         return question;
     }

    pub fn add_answer(&mut self, question_id: u128, answer_content: String) -> Question {

        let mut question;
        if let None = self.questions_asked.get (&question_id){
            question = Question {
                    content: "Theres no question here!".to_string(),
                    asker_id: env::signer_account_id(), //whoever signed the transaction 
                    id: question_id,
                    answers: Vec::<Answer>::new()
            };
            return question; //if there is no question, returns out of the question 
        }else{
                question = self.questions_asked.get(&question_id).unwrap(); //if question is found continue further
            }

            let answer = Answer {
                content: answer_content,
                responder_id: env::signer_account_id()

            };
            question.answers.push(answer);
            self.questions_asked.insert(&question_id, &question);
            return question;
     }

     pub fn get_question_by_id (&self, question_id: u128) -> Question {
         return self.questions_asked.get(&question_id).unwrap();
     }

     //experimental 
     pub fn get_questions_by_keyword(&self, keyword: String) -> Vec<Question> {
        let mut filtered_questions = Vec::new();

        for question in self.questions_asked.values() {
            if question.content.contains(&keyword) {
                filtered_questions.push(question);
            }
        }

        return filtered_questions;
    }


 }