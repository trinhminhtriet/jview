# jview

```text
   _         _
  (_)__   __(_)  ___ __      __
  | |\ \ / /| | / _ \\ \ /\ / /
  | | \ V / | ||  __/ \ V  V /
 _/ |  \_/  |_| \___|  \_/\_/
|__/

```

jview: A CLI app to print HTTP responses as JSON.

## ✨ Features

- **Prints HTTP responses**: Converts HTTP responses into JSON format for easy readability and processing.
- **Supports HTTP/1.1 and HTTP/3**: Can make requests using both HTTP/1.1 and HTTP/3 protocols.
- **Customizable**: Allows setting custom headers, user-agent, and basic authentication.

## 🚀 Installation

To install **jview**, simply clone the repository and follow the instructions below:

```bash
git clone git@github.com:trinhminhtriet/jview.git
cd jview

cargo build --release
cp ./target/release/jview /usr/local/bin/
```

Running the below command will globally install the `jview` binary.

```bash
cargo install jview
```

Optionally, you can add `~/.cargo/bin` to your PATH if it's not already there

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## 💡 Usage

Run **jview** with the following command:

```sh
./jview [options] [path]

```

### Available Options

- `-A, --agent string`: Set the User-Agent name (default is `jview/{{ Version }}`).
- `-u, --basic string`: Set Basic Auth username:password.
- `-H, --header stringArray`: Set HTTP request headers.
- `--http1.1`: Use HTTP/1.1.
- `--http3`: Use HTTP/3.
- `-X, --method string`: Set the HTTP request method (default is `GET`).
- `-v, --version`: Display the version of `jview`

## 🗑️ Uninstallation

Running the below command will globally uninstall the `jview` binary.

```bash
cargo uninstall jview
```

Remove the project repo

```bash
rm -rf /path/to/git/clone/jview
```

## 🤝 How to contribute

We welcome contributions!

- Fork this repository;
- Create a branch with your feature: `git checkout -b my-feature`;
- Commit your changes: `git commit -m "feat: my new feature"`;
- Push to your branch: `git push origin my-feature`.

Once your pull request has been merged, you can delete your branch.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
