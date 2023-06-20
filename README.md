# Todo Yew 📝✅

This is a simple version of a todo list using Yew 🌳

## Pre-Requisites 📋

Before getting started, make sure you have the following installed:

- Rust 🦀
- Cargo 📦
- Trunk 🚂

## Installing 🛠️

### Installing pre-requisites

To run the repository, we need to install some dependencies. Let's start with:

- **WebAssembly target** 🌐

    ```shell
    rustup target add wasm32-unknown-unknown
    ```

- **Trunk service** 📦

    ```shell
    cargo install --locked trunk
    ```

### Installing the repository 📥

Next, let's clone the repository:

```shell
git clone https://github.com/ChirujanoCodding/todo-yew
```

### Installing the dependencies 📦

We also need to install the project dependencies. Run the following command:

```shell
pnpm install
```

## Running ▶️

To start the project, follow these steps:

1. Open a terminal.

2. Compile the Sass files for styling:

    ```shell
    pnpm start:css
    ```

3. Run the project:

    ```shell
    pnpm start:rust
    ```

That's it! Now you can manage your todos using Todo Yew. Enjoy! 😊✨
