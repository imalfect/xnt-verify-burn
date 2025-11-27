# Neptune Burn Verifier for XNT

This repository provides a simple script that verifies the integrity of the 1,526,640 NPT burn transaction done by Neptune Privacy.

You can read more details about the burn in the posts below

[@neptuneprivacy on X](https://x.com/neptuneprivacy/status/1994141617423569123?s=46&t=btYLuRpt86cZlXZpFxe3Lw)

[@safetradeex on X](https://x.com/safetradeex/status/1994130845368766950)

## How to build and run locally

Before you run this application, you need to have Rust installed on your device. [rustup](https://rustup.rs) is an easy way to do so.

### Step 1: Download or clone this repository
If you have git installed, you can clone the repository with this command:
```bash
$ git clone https://github.com/imalfect/neptune-burn-verifier-xnt.git
```
Alternatively, you can download the repository as a ZIP file and extract it to your desired location, use the green button on the homepage of this repository

### Step 2: Run!
Navigate to the directory where you cloned or extracted the repository, then run:
```bash
$ cargo run --release
```
This command will compile and execute the program in release mode, the expected output can be found below (note that the chain height might be higher)
```
--- XNT Burn Prover ---
Connected to Neptune Cash RPC, current chain height: 17020
Fetching block 16999 announcements to find burn transaction
Block 16999 has 6 announcements
Fetched the transaction, has 2 outputs, checking
Found output with 1526640 coins, this IS a burn output!
Found output with 2242 coins, this IS NOT a burn output!
```

#### NOTE: BYON (Bring Your Own Node)
By default, the application uses [comradecobweb's](https://github.com/comradecobweb) node from the [neptune-cash-js readme](https://github.com/comradecobweb/neptune-cash-js)

If you'd like to use your own node, it needs to run the latest release and have the RPC enabled, you can find a basic tutorial [here](https://github.com/comradecobweb/neptune-cash-js?tab=readme-ov-file#configuring-rpc-server)
You can then set the `NEPTUNE_RPC_URL` environment variable to point to your node, for example
```bash
$ NEPTUNE_RPC_URL=https://127.0.0.1:9797 cargo run --release
```

## License
This project is licensed under The Unlicense. See the [LICENSE](LICENSE.md) file for
more details.