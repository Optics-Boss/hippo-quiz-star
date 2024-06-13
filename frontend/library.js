let quizes;

function getQuizes() {
  this.getJson("http://localhost:3030/quizes")
  .then((quizes) => {
    this.quizes = quizes;
  }).then(() => {
    for (let i = 0; i < this.quizes[0].length; i++) {
      this.appendElement("quiz_list", 
        `<li 
        onclick="goToQuiz('${this.quizes[0][i].title}')" 
        class="normal__list-item rounded_corners" 
        >
        ${this.quizes[0][i].title}
        </li>`
      );
    }

  });
}

function getJson(from) {
  return fetch(from)
  .then((response) => response.json())
  .then((json) => { 
    return json;
  })
}

function setElement(elementId, elementText) {
    const element = document.getElementById(elementId);

    if (element){
      element.innerHTML = elementText;
    } else {
      window.onload = () => {
        const element = document.getElementById(elementId);
        if(element) {
          element.innerHTML = elementText;
        }
      }
  }
}

function appendElement(elementId, elementText) {
    const element = document.getElementById(elementId);

    if (element){
      element.innerHTML += elementText;
    } else {
      window.onload = () => {
        const element = document.getElementById(elementId);
        if(element) {
          element.innerHTML += elementText;
        }
      }
  }
}

function changeBackgroundOfElement(elementId, color) {
     const element = document.getElementById(elementId);
     element.style.background = color;
}


function goToUrl(path) {
    window.location.assign(path);
}

function saveDataToStorage(key, value) {
  sessionStorage.setItem(key, value);
}

function getDataFromStorage(key) {
  return sessionStorage.getItem(key);
}

function removeDataFromStorage(key) {
  sessionStorage.removeItem(key);
}

function clearStorage() {
  sessionStorage.clear();
}

function shuffleArray(list) {
  return list.map(value => ({value, sort: Math.random()}))
  .sort((a, b) => a.sort - b.sort)
  .map(({ value }) => value)
}

this.getQuizes();
