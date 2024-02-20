export async function readFile(
  filePath: string,
  json: boolean = false,
  cache: boolean = false
) {
  const response = await fetch(filePath);
  if (response.ok) {
    if (json) {
      return response.json();
    } else {
      return response.text();
    }
  } else {
    throw new Error("Failed to load file");
  }
}
export async function readCSV(filePath: string) {
  let text = await readFile(filePath);
  const arr = [];
  let quote = false;
  for (let row = 0, col = 0, c = 0; c < text.length; c++) {
    const cc = text[c];
    const nc = text[c + 1];
    arr[row] = arr[row] || [];
    arr[row][col] = arr[row][col] || "";
    if (cc === '"' && quote && nc === '"') {
      arr[row][col] += '"';
      c++;
    } else if (cc === '"') {
      quote = !quote;
    } else if (cc === "," && !quote) {
      col++;
    } else if (cc === "\n" && !quote) {
      col = 0;
      row++;
    } else {
      arr[row][col] += cc;
    }
    return arr;
  }
}

export async function loadTags(c) {

}

