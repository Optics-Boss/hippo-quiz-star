function goToQuiz(quiz) {
  saveDataToStorage("quiz", quiz)
  goToUrl('quiz.html')
}
