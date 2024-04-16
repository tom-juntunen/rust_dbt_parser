# Rust dbt parser
This program parses the dbt manifest and outputs the raw_code and compiled_code for a specified input model.
In the future, this parser will support parsing more files and doing more things:

1. catalog.json --> producing dbt docs site & documented star schema
2. graph_summary.json --> producing a heatmap of model execution durations
3. manifest.json --> producing compiled sql text for writing to app db using models from catalog.json
4. run_results.json --> producing a table of model execution durations

# Installation
1. Install Rust for Mac (select standard installation): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Add .cargo/bin to $PATH:
    First check if using bash or zsh using `ps -p $$`.
    Then:
    - if zsh: `echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc`
    - if bash: `echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bash_profile`
3. Restart your current shell
    - if zsh: `source ~/.zshrc`
    - if bash: `source ~/.bash_profile`
4. Launch the cargo help menu:
    - `cargo`

# Usage
1. Test the compilation using Clippy
    - `cargo clippy`