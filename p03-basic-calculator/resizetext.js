"use strict";

//Gets the width of the text
function getTextWidth(text, font) {
  // re-use canvas object for better performance
  const canvas = getTextWidth.canvas || (getTextWidth.canvas = document.createElement("canvas"));
  const context = canvas.getContext("2d");
  context.font = font;
  const metrics = context.measureText(text);
  const height = Math.abs(metrics.actualBoundingBoxAscent) + Math.abs(metrics.actualBoundingBoxDescent);
  return [metrics.width, height];
}

// Gets the size and updates the text on the HTML
function fitText(el, maxHeight) {
  let resize = function() {
    let cs = window.getComputedStyle(el);
    let w = getTextWidth(el.innerText, cs.fontWeight + " 20.1px " + cs.fontFamily);
    let r = (el.clientWidth / (w[0]) * 20);

    let m = maxHeight(el);
    if(r > m) el.style.fontSize = m + 'px';
    else el.style.fontSize = r + 'px';
  };
  window.addEventListener("resize", resize);
  resize();
  return {
    stop: function() {
      window.removeEventListener("resize", resize);
    },
    call: resize,
  };
}

var firstRun = true;

let out;
var fit;

// Lets me run the code from rust
export function runFitText() {
  if (firstRun == true) {
    firstRun = false;
    out = document.getElementById("output");
    fit = fitText(out, function(el) {
      return el.clientHeight / 2;
    });
  }
  fit.call();
}