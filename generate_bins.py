import os

def generate_bins():
    bin_entries = []
    for root, dirs, files in os.walk('src/bin'):
        for file in files:
            if file.endswith('.rs'):
                relative_path = os.path.join(root, file)
                module_name = relative_path.replace('/', '_').replace('\\', '_').replace('.rs', '')
                bin_entry = f'''[[bin]]
name = "{module_name}"
path = "{relative_path}"
'''
                bin_entries.append(bin_entry)

    return '\n'.join(bin_entries)

def main():
    with open('Cargo.toml', 'r') as cargo_file:
        cargo_content = cargo_file.read()

    bin_entries = generate_bins()

    if '[[bin]]' in cargo_content:
        cargo_content = cargo_content.split('[[bin]]')[0]

    with open('Cargo.toml', 'w') as cargo_file:
        cargo_file.write(cargo_content.strip() + '\n\n' + bin_entries)

if __name__ == "__main__":
    main()
