async function init() {
  let rustApp = null;

  try {
    // NOTE Importing the compiled WASM module
    rustApp = await import('../pkg');
  } catch (e) {
    console.error(`Failed to initialize WASM module!\n\n--------`, e);
    return;
  }

  console.log(rustApp);

  const input = document.getElementById('upload');
  const fileReader = new FileReader();

  fileReader.onloadend = () => {
    const base64 = fileReader.result.replace(
      /^data:image\/(png|jpeg|jpg);base64,/,
      ''
    );

    const timeInit = new Date().getTime();
    // NOTE Calling the Rust fn from JS
    const imgDataUrl = rustApp.grayscale(base64);
    const durationInSec = (new Date().getTime() - timeInit) / 1000;

    console.log(`Duration: ${durationInSec} sec`); // 3.85 sec
    // ? Writing the result to the DOM
    document.getElementById('new-img').setAttribute('src', imgDataUrl);
  };

  input.addEventListener('change', () => {
    fileReader.readAsDataURL(input.files[0]);
  });
}

init();
