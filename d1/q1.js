const inputTxt = $$("html body pre")[0].childNodes[0].nodeValue;
const masses = inputTxt.trim().split("\n").map(x => Number(x));
const result = masses.map(x => Math.floor(x / 3) - 2).reduce((acc,next) => acc + next,0);
