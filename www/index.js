import * as wasm from "wasm-amogus";
import _ from "./bg-v2.png";
import { saveAs } from 'file-saver';

//wasm.greet();
var canvas = document.createElement("canvas");
var input_ctx = canvas.getContext("2d");
var output = document.getElementById("imageOutput");
var output_ctx = output.getContext("2d");
var imageData;
document.getElementById("imageInput").onchange = function (evt) {
  var tgt = evt.target || window.event.srcElement,
    files = tgt.files;

  // FileReader support
  if (FileReader && files && files.length) {
    var fr = new FileReader();
    fr.onload = () => showImage(fr);
    fr.readAsDataURL(files[0]);
  }
};
/*

          <input type="range" min="1" max="4" id="size" value="1">Size</input>
          <input type="range" min="0" max="10000" id="count" value="2000">Count</input>
          <input type="range" min="0" max="100" id="contrast" value="5">Contrast</input>
          <input type="range" min="0" max="100" id="random" value="2">Random</input>
*/

let settings = {
  size: { min: 1, max: 16, value: 1, text: "Size" },
  count: { min: 0, max: 10000, value: 500, text: "Count" },
  contrast: { min: 0, max: 100, value: 5, text: "Contrast" },
  random: { min: 0, max: 100, value: 2, text: "Contrast variation" },
};

let form = document.getElementById("settings");

for (let id in settings) {
  let parameters = settings[id];
  let inputLabel = document.createElement("label");
  inputLabel.setAttribute("for", id);
  inputLabel.innerHTML = parameters.text + ": " + parameters.value;
  let inputElement = document.createElement("input");
  inputElement.setAttribute("id", id);
  inputElement.setAttribute("type", "range");
  inputElement.setAttribute("min", parameters.min);
  inputElement.setAttribute("max", parameters.max);
  inputElement.setAttribute("value", parameters.value);
  inputElement.setAttribute("style", "width: 100%");

  form.appendChild(document.createElement("br"));
  form.appendChild(inputLabel);
  form.appendChild(document.createElement("br"));
  form.appendChild(inputElement);
  form.appendChild(document.createElement("br"));

  inputElement.onmousemove = function (evt) {
    settings[id].value = evt.target.value;
    inputLabel.innerHTML = parameters.text + ": " + settings[id].value;
  };

  inputElement.onchange = function (evt) {
    settings[id].value = evt.target.value;
    inputLabel.innerHTML = parameters.text + ": " + settings[id].value;
    updateRender();
  };
}

function showImage(fileReader) {
  var img = document.getElementById("imageElement");
  img.onload = () => getImageData(img);
  img.src = fileReader.result;
}

function getImageData(img) {
  canvas.width = img.width;
  canvas.height = img.height;
  input_ctx.drawImage(img, 0, 0);
  //imageData = ctx.getImageData(0, 0, img.width, img.height).data;
  //console.log("image data:", imageData);
  output.width = img.width;
  output.height = img.height;
  updateRender();
}

function updateRender() {
  wasm.apply(
    output.width,
    output.height,
    input_ctx,
    output_ctx,
    settings.size.value,
    settings.count.value,
    settings.contrast.value,
    settings.random.value
  );
}

function save() {
  output.toBlob(function(blob) {
    saveAs(blob, "image.png");
  });
}

document.getElementById("save").onclick = save