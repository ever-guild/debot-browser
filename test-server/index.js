import * as wasm from "debot-browser";

// Test key for test multisig (devnet)
let keypair = {
    public: "9f7fd3df9d72b133fe155c087928c4f9da423076cc20c9f5386614b462e49811",
    secret: "607a90aedb5df02a0f712572f0b5aa5d9342e5f3f2c0794df43f4a2a9688aef3"
  }

function toHexString(byteArray) {
    return Array.from(byteArray, function(byte) {
        return ('0' + (byte & 0xFF).toString(16)).slice(-2);
    }).join('')
}

let user_sbox = {};
user_sbox.get_public_key = async () => {
    return keypair.public
}

user_sbox.sign = async (unsigned) => {
    const res = wasm.sign(keypair, unsigned)
    return res.signature
}

let network = "net.ton.dev";
let wallet = "0:2f9f742cd3ed63c39a31c077d5faada4e52ea365a4b4a9e1d6709e6cb0e9d927"
let pubkey = `0x${keypair.public}`;
let testDebotAddress = "0:2d2696edfe3d7c0d74e8900b2a43ac362de5a45db6fe6147177e2fcd2abfd3e2";
let sendDebotAddress = "0:af5acb55481ebd9c923c11462ea6ab068508a0e05aacd8a9c33bb5b0cabcc33c";

let manifest = `{
    "version": 0,
    "debotAddress": "${testDebotAddress}",
    "initMethod": "invokeTest",
    "initArgs": {
      "arg1": "1500000000",
      "arg2": "68656c6c6f20776f726c6421",
      "arg3": true,
      "arg4": 3,
      "arg5": "0:e859a5858fc99c8f6044aa179af68140c2fb9b07b3f52b70bef51e0c799fd2df",
      "arg6": "${pubkey}",
      "arg7": {
        "1": {
          "data": "10"
        },
        "2": {
          "data": "2020"
        }
      }
    },
    "abi": {
      "ABI version": 2,
      "header": [],
      "functions": [
        {
          "name": "OnInvokeCompleted",
          "inputs": [
            {
              "name": "status",
              "type": "uint8"
            },
            {
              "components": [
                {
                  "name": "data",
                  "type": "bytes"
                }
              ],
              "name": "ret1",
              "type": "map(uint32,tuple)"
            }
          ],
          "outputs": []
        }
      ],
      "data": [],
      "events": []
    },
    "quiet": true,
    "chain": [
      {
        "type": "Input",
        "interface": "a1d347099e29c1624c8890619daf207bde18e92df5220a54bcc6d858309ece84",
        "method": "get",
        "params": {
          "value": "1500000000"
        }
      },
      {
        "type": "Input",
        "interface": "8796536366ee21852db56dccb60bc564598b618c865fc50c8b1ab740bba128e3",
        "method": "input",
        "params": {
          "value": "68656c6c6f20776f726c6421"
        }
      },
      {
        "type": "Input",
        "interface": "16653eaf34c921467120f2685d425ff963db5cbb5aa676a62a2e33bfc3f6828a",
        "method": "get",
        "params": {
          "value": true
        }
      },
      {
        "type": "Input",
        "interface": "ac1a4d3ecea232e49783df4a23a81823cdca3205dc58cd20c4db259c25605b48",
        "method": "select",
        "params": {
          "index": 3
        }
      },
      {
        "type": "Input",
        "interface": "d7ed1bd8e6230871116f4522e58df0a93c5520c56f4ade23ef3d8919a984653b",
        "method": "get",
        "params": {
          "value": "0:e859a5858fc99c8f6044aa179af68140c2fb9b07b3f52b70bef51e0c799fd2df"
        }
      }
    ]
}`;

const sendManifest = `{
    "version": 0,
    "debotAddress": "${sendDebotAddress}",
    "initMethod": "invokeSend",
    "initArgs": {
      "dest": "${sendDebotAddress}",
      "amount": 500000000,
      "bounce": false
    },
    "quiet": true,
    "autoApprove": ["ApproveOnChainCall"],
    "abi": {
        "ABI version": 2,
        "version": "2.2",
        "header": ["time"],
        "functions": [
            {
                "name": "onSend",
                "inputs": [
                    {"name":"succeed","type":"bool"},
                    {"name":"sdkError","type":"uint32"},
                    {"name":"exitCode","type":"uint32"}
                ],
                "outputs": [
                ]
            }
        ],
        "data": [],
        "events": [],
        "fields": []
    },
    "chain": []
}
`;

(async () => {
    let logger = document.getElementById("log");
    logger.textContent += `NETWORK=${network}\n`;
    logger.textContent += `DEBOT=${testDebotAddress}\n`;
    logger.textContent += "Test 1. run_debot_browser\n";
    let result;
    //wasm.init_log();
    console.time("Test 1. Run browser (single call)");
    for (let i = 0; i < 10; i++) {
        result = await wasm.run_debot_browser(network, wallet, pubkey, null, manifest);
    }
    console.timeEnd("Test 1");
    logger.textContent += "Result:\n"
    logger.textContent += result;
    logger.textContent += "\nTest 1. Completed\n"

    logger.textContent += "Test 2. Create, run, destroy browser (3 calls)\n";
    const manifestObj = JSON.parse(manifest);
    const handle = await wasm.create_browser(network, testDebotAddress, wallet, pubkey);
    console.time("Test 2");
    for (let i = 0; i < 10; i++) {
        result = await wasm.run_browser(handle, manifestObj);
    }
    console.timeEnd("Test 2");
    console.log("run_browser result", result);
    logger.textContent += JSON.stringify(result);
    await wasm.destroy_browser(handle);
    logger.textContent += "\nTest 2. Completed\n";

    logger.textContent += "\nTest 3. Create 3 browsers in parallel.\n";
    console.time("Test 3");
    const handle1 = wasm.create_browser(network, testDebotAddress, wallet, pubkey);
    const handle2 = wasm.create_browser(network, testDebotAddress, wallet, pubkey);
    const handle3 = wasm.create_browser(network, testDebotAddress, wallet, pubkey);

    const handles = await Promise.all([handle1, handle2, handle3]);
    console.log(`handle1 = ${handles[0].toString(16)} handle2 = ${handles[1].toString(16)} handle3 = ${handles[2].toString(16)}`)
    console.timeEnd("Test 3");
    logger.textContent += "Test 3. Completed.\n";

    logger.textContent += "\nTest 4. Create, register box, update settings, run, destroy.\n";
    console.time("Test 4");
    const handle4 = await wasm.create_browser(network, sendDebotAddress, null, null);
    let sbox_handle = await wasm.register_signing_box(handle4, user_sbox)
    await wasm.update_user_settings(handle4, { 
        wallet: wallet,
        pubkey: pubkey,
        signing_box: sbox_handle
    })

    const res = await wasm.run_browser(handle4, JSON.parse(sendManifest));
    console.log(res)
    await wasm.close_signing_box(handle4, sbox_handle)
    await wasm.destroy_browser(handle4);
    console.timeEnd("Test 5");
    logger.textContent += `Result: ${JSON.stringify(res)}\n`;
    logger.textContent += "Test 5. Completed\n";
})();