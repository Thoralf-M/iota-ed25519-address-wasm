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
    let coin_type = document.getElementById('coin_type').value.trim()
    let account_index = document.getElementById('account_index').value.trim()
    let internal = document.getElementById('internal').value.trim()
    let address_index = document.getElementById('address_index').value.trim()
    let bech32_hrp = document.getElementById('bech32_hrp').value.trim()
    let result = await lib.generate_address(mnemonic, parseInt(coin_type), parseInt(account_index), JSON.parse(internal.toLowerCase()), parseInt(address_index), bech32_hrp)
    console.log(result);
    document.getElementById('address').value = result.slice(-2, -1)
    document.getElementById('bech32_address').value = result.slice(-1)
    addElement(JSON.stringify(result, null, 1))
  } catch (e) {
    addElement(e)
  }
}

async function generate_address_with_logs() {
  try {
    let mnemonic = document.getElementById('mnemonic').value.trim()
    let coin_type = document.getElementById('coin_type').value.trim()
    let account_index = document.getElementById('account_index').value.trim()
    let internal = document.getElementById('internal').value.trim()
    let address_index = document.getElementById('address_index').value.trim()
    let bech32_hrp = document.getElementById('bech32_hrp').value.trim()
    let result = await lib.generate_address_with_logs(mnemonic, coin_type, account_index, JSON.parse(internal.toLowerCase()), address_index, bech32_hrp)
    console.log(result);
    document.getElementById('address').value = result.slice(-2, -1)
    document.getElementById('bech32_address').value = result.slice(-1)
    addElement(JSON.stringify(result, null, 1))
  } catch (e) {
    addElement(e)
  }
}

async function change_bech32_hrp() {
  try {
    let address = document.getElementById('bech32_address').value.trim()
    let bech32_hrp = document.getElementById('bech32_hrp').value.trim()
    let result = await lib.change_bech32_hrp(address, bech32_hrp)
    console.log(result);
    document.getElementById('address').value = JSON.parse(result.slice(-2, -1)).data
    document.getElementById('bech32_address').value = result.slice(-1)
    addElement(JSON.stringify(result, null, 1))
  } catch (e) {
    addElement(e)
  }
}

async function to_bech32_address() {
  try {
    let address = document.getElementById('address').value.trim()
    let bech32_hrp = document.getElementById('bech32_hrp').value.trim()
    let address_type = document.getElementById('address_type').value.trim()
    let result = await lib.to_bech32_address(address, bech32_hrp, parseInt(address_type))
    console.log(result);
    document.getElementById('bech32_address').value = result
    addElement(JSON.stringify(result, null, 1))
  } catch (e) {
    addElement(e)
  }
}

function addElement(address) {
  let addressElement = document.getElementById("result");
  addressElement.innerHTML = "<pre>" + address + '<br>'
}

