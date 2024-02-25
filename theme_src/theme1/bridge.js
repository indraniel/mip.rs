document.addEventListener("keydown", keyDownTextField, false);

function keyDownTextField(e) {
  if(e.keyCode==82 && e.ctrlKey) {
    reload("1")
  }
}

function checkSeed(url){
  var request = new XMLHttpRequest();
  request.open('GET',url , true);
  request.send(null);
  request.onreadystatechange = function () {
    if (request.readyState === 4 && request.status === 200) {
      var type = request.getResponseHeader('Content-Type');
      if (type.indexOf("text") !== 1) {
        newSeed = request.responseText;
        if(newSeed !== initialSeed){
          //alert(newSeed)
          //reload("2")
          initialSeed = newSeed;
          location.reload();
        }
      }
    }
  }
}


var sleep = duration => new Promise(resolve => setTimeout(resolve, duration))
var poll = (promiseFn, duration) => promiseFn().then(
  sleep(duration).then(() => poll(promiseFn, duration)))

poll(() => new Promise(() => {
  //reload("3")
  //location.reload();
  checkSeed(seedUrl)
}), 500)
