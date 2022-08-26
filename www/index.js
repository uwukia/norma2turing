import * as wasm from "norma2turing";

var codeBlock = document.getElementById("norma");
var infoTitle = document.getElementById("infoTitle");
var infoCode = document.getElementById("infoCode");
var infoText = document.getElementById("infoText");

var saveButton = document.getElementById("save");
saveButton.disabled = true;

var fileData = "// empty";
var fileName = "maqturing.mt"

codeBlock.addEventListener('input', () => {
    let codeText = codeBlock.value;
    var compileResult = wasm.compile_code(codeText);

    saveButton.disabled = !compileResult.successful;
    fileData = compileResult.filedata;
    fileName = compileResult.filename;

    let hash = compileResult.info.indexOf('#');
    setTitle(compileResult.info.substring(0, hash));
    let infoInner = compileResult.info.substring(hash + 1);

    let sep = infoInner.lastIndexOf('=');
    infoCode.innerHTML = infoInner.substring(0, sep);
    infoText.innerHTML = infoInner.substring(sep + 1);
}, false);

saveButton.onclick = () => {
    let download = document.createElement('a');
    download.style.display = 'none';
    download.setAttribute('href', 'data:text/plain;charset=utf-8,' + encodeURIComponent(fileData));
    download.setAttribute('download', fileName);
    download.click();
}

var setTitle = (titleString) => {
    infoTitle.innerHTML = titleString;
    switch (titleString) {
        case 'Ok':
            infoTitle.style.color = '#7085b5';
            break;
        case 'Aviso':
            infoTitle.style.color = '#f5ff3d';
            break;
        case 'Erro':
            infoTitle.style.color = '#ff3d3d';
            break;
    }
}