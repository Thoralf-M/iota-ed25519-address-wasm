let lib
wasm.wasmmodule.then(r => lib = r)

async function generate_mnemonic() {
  try {
    let mnemonic = await lib.generate_mnemonic()
    document.getElementById('mnemonic').value = mnemonic
    console.log(mnemonic);
    addElement(mnemonic)
  } catch (e) {
    addElement(e)
  }
}

async function generate_address() {
  try {
    let mnemonic = document.getElementById('mnemonic').value.trim()
    let result = await lib.generate_address(mnemonic)
    console.log(result);
    addElement(JSON.stringify(result, null, 1))
  } catch (e) {
    addElement(e)
  }
}

function addElement(address) {
  let addressElement = document.getElementById("result");
  addressElement.innerHTML = "<pre>" + address + '<br>'
}
