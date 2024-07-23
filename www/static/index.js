function goToQuiz(quiz) {
  saveDataToStorage("quiz", quiz)
  goToUrl('/static/quiz.html')
}
