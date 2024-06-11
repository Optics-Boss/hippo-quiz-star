let questions;
let rightColor = "#41BE98";
let wrongColor = "#BE4167";

function getQuestions(quiz) {
  this.getJson("http://localhost:3030/quiz/" + quiz)
  .then((questions) => {
    this.questions = questions;
    console.table(this.questions[0][0]);
  }).then(() => {
    this.setElement("quiz_name", quiz);
    this.setElement("statement", this.questions[0][0].statement);
    this.setElement("list1", this.questions[0][0].right_answer);
    this.setElement("list2", this.questions[0][0].wrong_answer_1);
    this.setElement("list3", this.questions[0][0].wrong_answer_2);
    this.setElement("list4", this.questions[0][0].wrong_answer_3);
  });
}

function wrongAnswer(elementId) {
  changeBackgroundOfElement(elementId, wrongColor);
}

function rightAnswer(elementId) {
  changeBackgroundOfElement(elementId, rightColor);
}

this.getQuestions("german-1.hippo");
