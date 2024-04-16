import json
from datetime import datetime


def build_model_key(prefix, model_name):
    """Constructs a model key with a given prefix and model name."""
    return f"model.{prefix}.{model_name}"

def main():
    now = datetime.now()
    # Path to the JSON file containing the manifest data
    manifest_path = "sample/target/manifest.json"
    
    # Open and read the JSON file
    with open(manifest_path, 'r') as file:
        manifest = json.load(file)
    
    # Define the model name and construct the full model key
    model_name = "my_second_dbt_model"  # Change this to your actual model name
    full_model_name = build_model_key("sample", model_name)
    
    # Retrieve the model data and print SQL if available
    nodes = manifest.get('nodes', {})
    if full_model_name in nodes:
        model = nodes[full_model_name]
        print(f"Model SQL: {model.get('raw_code', 'No raw SQL found')}")
        print(f"Compiled SQL: {model.get('compiled_code', 'No compiled SQL found')}")
        later = datetime.now()
        print(f"Program took {(later-now).total_seconds() * 1000} milliseconds to run.")
    else:
        print(f"Model {model_name} not found under key {full_model_name}")

if __name__ == "__main__":
    main()
