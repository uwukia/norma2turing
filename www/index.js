import display from './display.json';
import * as wasm from "norma2turing";
import rawHelpPT from './help-pt.md'
import rawHelpEN from './help-en.md'

var md = require('markdown-it')();

const helpPT = md.render(rawHelpPT);
const helpEN = md.render(rawHelpEN);

// DIVS

var helpPanel = document.getElementById("sidePanel");
var helpButton = document.getElementById("sideGrabber");
var langToggle = document.getElementById("langToggle");

var codeBlock = document.getElementById("norma");
var infoTitle = document.getElementById("infoTitle");
var infoCode = document.getElementById("infoCode");
var infoText = document.getElementById("infoText");
var footerNote = document.getElementById("footerNote");

var saveButton = document.getElementById("save");
saveButton.disabled = true;

// GLOBAL VARIABLES

var lang = "EN";
var resultTitle = "";
var resultInfoCode = "";
var resultInfoText = "";
var fileData = "// empty";
var fileName = "maqturing.mt"

var timeTakenTimeout = setTimeout(console.log("OK"), 5000);

// EVENT HANDLERS

codeBlock.addEventListener('input', () => {
    let codeText = codeBlock.value;
    var compileResult = wasm.compile_code(codeText);

    // disable save button if compilation failed
    saveButton.disabled = !compileResult.successful;

    // this doesn't matter if saveButton.disabled (they are default values)
    fileData = compileResult.filedata;
    fileName = compileResult.filename;

    // set result values
    resultTitle = compileResult.titlecode;
    resultInfoCode = compileResult.showcode;
    resultInfoText = compileResult.errorcode;

    updateResult();
    // updateTimeTaken(compileResult.timetaken);
}, false);

saveButton.onclick = () => {
    let download = document.createElement('a');
    download.style.display = 'none';
    download.setAttribute('href', 'data:text/plain;charset=utf-8,' + encodeURIComponent(fileData));
    download.setAttribute('download', fileName);
    download.click();
}

helpButton.onclick = () => {
    if (helpPanel.style.marginLeft == "0px") {
        helpPanel.style.marginLeft = "-410px";
    } else {
        console.log(helpPanel.style.marginLeft);
        helpPanel.style.marginLeft = "0px";
    }
}

langToggle.onclick = () => {
    switch (lang) {
        case "EN":
            lang = "PT";
            break;
        case "PT":
            lang = "EN";
            break;
    }

    updateLanguage();
}

// FUNCTIONS

const updateLanguage = () => {
    langToggle.innerHTML = lang;
    saveButton.innerHTML = display[lang].saveButton;
    footerNote.innerHTML = display[lang].footerNote;
    updateResult();
    updateHelpPage();
}

const updateResult = () => {
    updateTitle();
    infoCode.innerHTML = resultInfoCode;
    infoText.innerHTML = getInfoText();
}

const updateTitle = () => {

    switch (resultTitle) {
        case "OK":
            infoTitle.style.color = "#7085b5";
            break;
        case "ERR":
            infoTitle.style.color = "#ff3d3d";
            break;
        case "WARN":
            infoTitle.style.color = "#f5ff3d";
            break;
        default:
            return;
    }

    infoTitle.innerHTML = display[lang].resultTitle[resultTitle];
}

const updateHelpPage = () => {
    switch (lang) {
        case "EN":
            helpPanel.innerHTML = helpEN;
            break;
        case "PT":
            helpPanel.innerHTML = helpPT;
            break;
    }
}

const updateTimeTaken = (timeString) => {
    clearTimeout(timeTakenTimeout);
    timeTakenTimeout = setTimeout(console.log(timeString), 2000);
}

const getInfoText = () => {
    let params = resultInfoText.split(' ');
    let errmsg = display[lang].errorMessage[params[0]];

    if (errmsg == undefined) return "";

    let bound = Math.min(params.length, errmsg.length);
    let errorMessage = errmsg[0];
    for (let i = 1; i < bound; i++) {
        errorMessage += params[i] + errmsg[i];
    }

    return errorMessage
}

updateHelpPage();