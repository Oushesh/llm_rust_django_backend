import yaml
def test_model_config(model_content:str)->bool:
        try:
            data = yaml.safe_load(model_content)
            return True
        except yaml.YAMLError as exc:
            print(exc)
            return False

def test_colang_config(colang_config:str)->bool:
    assert (colang_config.endswith("co")==True)
    try:
        with open(colang_config,'r',encoding='utf-8') as stream:
            data = stream.read()
            return True
    except FileNotFoundError:
        print ("The file was not Found")
        return False
    except PermissionError:
        print ("You do not have permission to read the File")
        return False
    except UnicodeError:
        print ("There is an error decoding the File. Check your encoding type")
        return False
    except Exception as e:
        print(f"An unexpected error occurred: {e}")
        return False

def test_yaml_file(file:str):
    # Read the YAML file
    with open(file, 'r') as file:
        data = yaml.safe_load(file)

    # Access the data
    models = data.get('models', [])
    for model in models:
        model_type = model.get('type')
        engine = model.get('engine')
        model_name = model.get('model')

        print(f"Model Type: {model_type}")
        print(f"Engine: {engine}")
        print(f"Model Name: {model_name}")
        print("-----")
    return None

