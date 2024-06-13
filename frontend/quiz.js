let questions;
let rightColor = "#41BE98";
let wrongColor = "#BE4167";

function getQuestions(quiz) {
  this.getJson("http://localhost:3030/quiz/" + quiz)
  .then((questions) => {
    this.questions = questions;
  }).then(() => {
    this.setElement("quiz_name", quiz);

    for (let i = 0; i < this.questions[0].length; i++) {
        this.appendElement("questions_list", 
          `<h2 id="statement">${this.questions[0][i].statement}</h2>
            <ul class="normal__list">
              <li 
                onclick="wrongAnswer('list1-${i}')" 
                class="normal__list-item rounded_corners" 
                id="list1-${i}">
                ${this.questions[0][i].wrong_answer_1}
              </li>
              <li 
                onclick="wrongAnswer('list2-${i}')" 
                class="normal__list-item rounded_corners" 
                id="list2-${i}">
                ${this.questions[0][i].wrong_answer_2}
              </li>
              <li 
                onclick="wrongAnswer('list3-${i}')" 
                class="normal__list-item rounded_corners" 
                id="list3-${i}">
                ${this.questions[0][i].wrong_answer_3}
              </li>
              <li 
                onclick="rightAnswer('list4-${i}')" 
                class="normal__list-item rounded_corners" 
                id="list4-${i}">
                ${this.questions[0][i].right_answer}
              </li>
            </ul>
      `
      );
    }
  });
}

function wrongAnswer(elementId) {
  changeBackgroundOfElement(elementId, wrongColor);
}

function rightAnswer(elementId) {
  changeBackgroundOfElement(elementId, rightColor);
}

this.getQuestions(this.getDataFromStorage("quiz"));
