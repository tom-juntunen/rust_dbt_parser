# Rust dbt parser
This program parses the dbt manifest and outputs the raw_code and compiled_code for a specified input model.
In the future, this parser will support parsing more files and doing more things:

(TODO) 1. catalog.json --> producing dbt docs site & documented star schema
(TODO) 2. graph_summary.json --> producing a heatmap of model execution durations
3. manifest.json --> producing compiled sql text for writing to app db using models from catalog.json
(TODO) 4. run_results.json --> producing a table of model execution durations

# Installation
1. Install Rust for Mac (select standard installation): 
- `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
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

# dbt Setup
1. Create and activate new Python virtual environment: 
- `python -m venv venv && source venv/bin/activate`
2. Install dbt w/ forked sqlite adapter: 
- `pip install -r requirements.txt`
3. Copy `adapter/` and `include/` to appropriate location in dbt site packages folder:
**IMPORTANT**: Replace `venv/lib/python3.8` with your actual path to python:
```
cp -r dbt-sqlite/dbt/adapters/sqlite venv/lib/python3.8/site-packages/dbt/adapters && \
cp -r dbt-sqlite/dbt/include/sqlite venv/lib/python3.8/site-packages/dbt/include && \
cp -r dbt-sqlite/dbt_sqlite-1.4.0.dist-info venv/lib/python3.8/site-packages
```
4. Add the `config` and `sample` objects to ~/.dbt/profiles.yml using this indentation:
    ```
    config:
        send_anonymous_usage_stage: false
    
    datalogz_ingest:
        ... (not shown here)
    
    sample:
        target: dev
        outputs:
            dev:
            type: sqlite
            threads: 1
            database: 'sample'
            schema_directory: 'db'
            schema: 'main'
            schemas_and_paths:
                main: 'db/sample.db'
    ```
5. Change directories to the dbt project dir:
- `cd sample`
6. Run the following command to test dbt connection:
- `dbt debug --profile sample`
7. Run the following command to produce the catalog.json docs:
- `dbt docs generate`

# Usage
1. Change directories to project root if still in `sample`
- `cd ../`
2. Test the compilation using Clippy
- `cargo clippy`
3. Run the program
- `cargo run`

# Release
1. Build a high-performance release
- `cargo build --release`
2. Run the program from the release build
- `./target/release/dbt_parser my_first_dbt_model`

# Performance
1. Run `lookalike.py` from `src` directory -> 81ms to parse model from large manifest.
2. Run rust program from `rust_dbt_parser` directory -> 50ms to parse model from large manifest.
- This represents a 38% improvement in parsing speed.
