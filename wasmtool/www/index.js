import * as wasm from "wasmtool";
import {
  fileOpen,
  directoryOpen,
  fileSave,
  supported,
} from "browser-fs-access";

// async function getFile() {
//   // Open file picker and destructure the result the first handle
//   const [fileHandle] = await window.showOpenFilePicker();
//   const file = await fileHandle.getFile();
//   return file;
// }

// const pickerOpts = {
//   types: [
//     {
//       description: "Images",
//       accept: {
//         "image/*": [".png", ".gif", ".jpeg", ".jpg"],
//       },
//     },
//   ],
//   excludeAcceptAllOption: true,
//   multiple: false,
// };

// async function getTheFile() {
//   // Open file picker and destructure the result the first handle
//   const [fileHandle] = await window.showOpenFilePicker(pickerOpts);

//   // get file contents
//   const fileData = await fileHandle.getFile();
// }

// getFile();

if (supported) {
  console.log("Using the File System Access API.");
} else {
  console.log("Using the fallback implementation.");
}

async function getFile() {
  const blob = await fileOpen([
    {
      description: "DMM files",
      mimeTypes: [],
      extensions: [".dmm"],
      multiple: false,
    },
  ]);
  // console.log(blob.text());
  return await blob.text();
}

async function onClick() {
  // wasm.greet();
  let x = await getFile();

  console.log("autopiping...");
  let y = wasm.autopipe(x);

  console.log("autopiped!");
  console.log(y);
  console.log("autopiped, holy hell!");

  let out_blob = new Blob([y], {
    type: "text/plain",
  });

  await fileSave(out_blob, {
    fileName: "autopiped.dmm",
    extensions: [".dmm"],
  });
}

const uploadButton = document.getElementById("uploadButton");
uploadButton.addEventListener("click", (event) => {
  onClick();
});
