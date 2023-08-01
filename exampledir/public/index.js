navigator.storage.getDirectory().then(async (dir) => {

    // Log out the directory
    console.log("Directory:", dir);

    // Read in the chirp file
    const chirpDataText = await readChirpFile(dir);
    // Change the text into an array
    const chirpData = processChirpData(chirpDataText);
    // If data fill in the chirp boxes
    if (chirpData.length > 0) {
        document.getElementById("chirps").innerHTML = "";
        chirpData.forEach((line) => {
            document.getElementById("chirps").innerHTML += '<div class="chirp"><div>Me: ' + line[0] + '</div><div class="date">' + line[1] + '</div></div>';
        });
    }

    document
        .getElementById("createChirp")
        .addEventListener("click", async () => {
            // Save the chirp.
            const chirpTextArea = document.getElementById("chirpTextArea")
            const newChirpValue = chirpTextArea.value;

            console.log("Saving Chirp:", newChirpValue);
            await createChirpFile(dir, newChirpValue);

            // Refresh the page
            location.reload();
        }); 
});

function processChirpData(data) {
    const lines = data.split('\n');
    return lines.map((line) => {
        return line.split(',');
    })
}

async function readChirpFile(dir) {
   // Get handle to draft file in OPFS
   const draftHandle = await dir.getFileHandle("chirp.txt", { create: true });

   // Get the file and read the text
   const accessHandle = await draftHandle.getFile();
   const text = await accessHandle.text();

   return text;
}

async function createChirpFile(dir, message) {

    const currentData = await readChirpFile(dir);
    console.log(currentData);
  
    // Get handle to draft file in OPFS
    const draftHandle = await dir.getFileHandle("chirp.txt", { create: true });
    const writable = await draftHandle.createWritable();

    // Write the contents of the file to the stream.
    await writable.write(message + "," + (new Date()).toString() + '\n' + currentData);
  
    // Close the file and write the contents to disk.
    await writable.close();
  };
  