# BLESS-NETWORK-BOT
This is a high-performance robot developed with Rust, supporting multi-node operations and session management to earn rewards for the Bless Network.

#### Table of contents

* [Init Account & Nodes](#init-account-&-nodes)
* [Usage](#usage)
* [Download](#download)
* [Build](#build)
* [License](#license)

## Init Account & Nodes
1. Open your browser and sign up for an account at [Bless Network](https://bless.network/dashboard?ref=0F4X45).
2. After signing up, open the Developer Tools in your browser (press F12) and go to **Application > Local Storage**.
3. Look for the key `B7S_AUTH_TOKEN` and copy the corresponding `accessToken`, which will begin with `eyJ...`.
4. Install Node.js (v20.11.1 or your preferred version).
5. Edit the `crypto.js` file by replacing the `main()` function call with your copied `accessToken`, like this:

```javascript
main("Your AccessToken");
// Uncomment the following line to test hash generation:
// WTFTestHash(); // Test to create a hash
```

## Usage

1. Download and extract the files from [this link](#download).
2. Configure the `.json` file like the example below:

    ```json
    [
        {
            "token": "accessToken 1",
            "proxy": null
        },
        {
            "token": "accessToken 2",
            "proxy": "https://test:1234@example.com:8080"
        }
    ]
    ```

    - `proxy = null` disables proxy usage.

3. Run the program with the following command:

    ```bash
    bless-network.exe --init config.json
    ```

You can also add custom tags in the JSON for easier identification of your accounts, like so:

```json
[
    {
        "name": "Account 1",
        "token": "accessToken 2",
        "proxy": "https://test:1234@example.com:8080"
    }
]
```

- The `name` tag helps you identify accounts but does not affect the program’s functionality.

## Download

- **Windows**: [Download here](https://github.com/mantvmass/bless-network/releases)
- **Other Platforms**: [Manual Build Instructions](#build)

## Build
To build the project locally, follow these steps:
1. Install [Rust](https://www.rust-lang.org/tools/install).
2. Run `cargo build --release` to build the project.

## License
This project is licensed under the MIT License. See the [LICENSE](https://github.com/mantvmass/bless-network/blob/main/LICENSE) file for more details.