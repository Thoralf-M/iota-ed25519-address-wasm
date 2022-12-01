let lib
import('./pkg').then(r => {
    lib = r
})

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

async function get_node_info() {
    try {
        let node_url = document.getElementById('node_url').value.trim()
        let nodeinfo = await lib.get_node_info(node_url)
        console.log(nodeinfo);
        addElement(JSON.stringify(nodeinfo))
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

async function generate_addresses() {
    try {
        for (let i = 0; i < document.getElementById('addresses_amount').value.trim(); i++) {
            // for large amounts the loop should be done on the wasm side
            let mnemonic = document.getElementById('mnemonic').value.trim()
            let coin_type = document.getElementById('coin_type').value.trim()
            let account_index = document.getElementById('account_index').value.trim()
            let internal = document.getElementById('internal').value.trim()
            let address_index = document.getElementById('address_index').value.trim()
            let bech32_hrp = document.getElementById('bech32_hrp').value.trim()
            let result = await lib.generate_address(mnemonic, parseInt(coin_type), parseInt(account_index) + i, JSON.parse(internal.toLowerCase()), parseInt(address_index), bech32_hrp)
            console.log(result);
            document.getElementById('address').value = result.slice(-2, -1)
            document.getElementById('bech32_address').value = result.slice(-1)
            addElement(JSON.stringify(result, null, 1))
        }
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
        let result = await lib.generate_address_with_logs(mnemonic, parseInt(coin_type), parseInt(account_index), JSON.parse(internal.toLowerCase()), parseInt(address_index), bech32_hrp)
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

async function hash_public_key() {
    try {
        let public_key = document.getElementById('public_key').value.trim()
        let result = await lib.hash_public_key(public_key)
        console.log(result);
        document.getElementById('address').value = result
        // Update also the bech32 address
        to_bech32_address()
        addElement(JSON.stringify(result, null, 1))
    } catch (e) {
        addElement(e)
    }
}

function addElement(address) {
    let addressElement = document.getElementById("result");
    addressElement.innerHTML += "<pre>" + address + '<br>'
}

function clear_output() {
    let addressElement = document.getElementById("result");
    addressElement.innerHTML = ''
}

function bytes_to_hex() {
    let bytes_strings = document.getElementById("bytes").value.trim().split(',');
    let bytes = []

    for (let byte_string of bytes_strings) {
        if (Number.isInteger(parseInt(byte_string))) {
            bytes.push(parseInt(byte_string))
        }
    }

    let bytes_hex = bytesToHex(bytes)
    document.getElementById('bytes_hex').value = '0x' + bytes_hex
}

function bytesToHex(bytes) {
    for (var hex = [], i = 0; i < bytes.length; i++) {
        var current = bytes[i] < 0 ? bytes[i] + 256 : bytes[i];
        hex.push((current >>> 4).toString(16));
        hex.push((current & 0xF).toString(16));
    }
    return hex.join("");
}

function hex_to_bytes() {
    let hex_string = document.getElementById("bytes_hex").value.trim();
    if (hex_string.startsWith("0x")) {
        hex_string = hex_string.slice(2, hex_string.length)
    }
    let bytes = hexToBytes(hex_string)
    document.getElementById('bytes').value = bytes
}

function hexToBytes(hex) {
    for (var bytes = [], c = 0; c < hex.length; c += 2)
        bytes.push(parseInt(hex.substr(c, 2), 16));
    return bytes;
}

function bytes_to_utf8() {
    let bytes_strings = document.getElementById("bytes").value.trim().split(',');
    let bytes = []
    for (let byte_string of bytes_strings) {
        if (Number.isInteger(parseInt(byte_string))) {
            bytes.push(parseInt(byte_string, 10))
        }
    }
    let bytes_utf8 = new TextDecoder().decode(new Uint8Array(bytes));
    document.getElementById('bytes_utf8').value = bytes_utf8
}

function utf8_to_bytes() {
    let bytes_utf8 = document.getElementById("bytes_utf8").value.trim();
    let bytes = new TextEncoder("utf-8").encode(bytes_utf8);
    document.getElementById('bytes').value = bytes
}

export { bytes_to_utf8, utf8_to_bytes, bytes_to_hex, hex_to_bytes, clear_output, generate_mnemonic, get_node_info, generate_address, generate_addresses, generate_address_with_logs, change_bech32_hrp, to_bech32_address, hash_public_key };
