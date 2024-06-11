let quizes;

function getQuizes() {
  this.getJson("http://localhost:3030/quizes")
  .then((quizes) => {
    this.quizes = quizes;
  }).then(() => {
    console.table(this.quizes);
    this.setElement("list1", this.quizes[0].title);
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

function changeBackgroundOfElement(elementId, color) {
     const element = document.getElementById(elementId);
     element.style.background = color;
}


function goToUrl(path) {
    window.location.assign(path);
}

this.getQuizes();
