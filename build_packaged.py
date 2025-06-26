import zipfile
import os

def zip_files(file_paths, output_zip_name):
    with zipfile.ZipFile(output_zip_name, 'w', zipfile.ZIP_DEFLATED) as zipf:
        for file_path in file_paths:
            if os.path.exists(file_path):
                archive_name = os.path.basename(file_path)
                zipf.write(file_path, archive_name)

if __name__ == "__main__":
    # TODO: implementation argument compile_resource, packaged_build
    'cargo run --release compile_resource'
    'cargo build --release --features packaged_build'

    files_to_zip = [
        "file1.txt",
        "file2.log",
        os.path.join("temp_dir", "file3.py"),
        "non_existent_file.txt"
    ]

    output_zip_filename = "my_files_archive.zip"
    zip_files(files_to_zip, output_zip_filename)

