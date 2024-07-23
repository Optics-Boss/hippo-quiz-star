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
       let questionsArray = [
              `<li 
                onclick="wrongAnswer('list1-${i}')" 
                class="normal__list-item rounded_corners" 
                id="list1-${i}">
                ${this.questions[0][i].wrong_answer_1}
              </li>`,
              `<li 
                onclick="wrongAnswer('list2-${i}')" 
                class="normal__list-item rounded_corners" 
                id="list2-${i}">
                ${this.questions[0][i].wrong_answer_2}
              </li>`,
              `<li 
                onclick="wrongAnswer('list3-${i}')" 
                class="normal__list-item rounded_corners" 
                id="list3-${i}">
                ${this.questions[0][i].wrong_answer_3}
              </li>`,
              `<li 
                onclick="rightAnswer('list4-${i}')" 
                class="normal__list-item rounded_corners" 
                id="list4-${i}">
                ${this.questions[0][i].right_answer}
              </li>`
        ];
      
        let shuffledQuestionsArray = this.shuffleArray(questionsArray);

        this.appendElement("questions_list", 
          `<h2 class="align__text__center" id="statement">${this.questions[0][i].statement}</h2>
            <div class="center">
              <img class="normal__image" src="http://localhost:3030/static/images/${this.questions[0][i].image}" alt="${this.questions[0][i].statement}">
            </div>
            <ul class="normal__list">
              ${shuffledQuestionsArray[0]}
              ${shuffledQuestionsArray[1]}
              ${shuffledQuestionsArray[2]}
              ${shuffledQuestionsArray[3]}
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
