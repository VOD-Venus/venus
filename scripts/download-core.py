#!/usr/bin/env python
import requests
import platform
import os
import zipfile
from tqdm import tqdm

TARGET_DIR = os.path.join(os.getcwd(), "v2ray-core/")

system = platform.system()
machine = platform.machine()
system_map = {
    'Darwin': 'macos'
}
print(f"current system {system_map[system]} machine {machine}")


def get_latest_release_tag(repo_owner, repo_name):
    url = f"https://api.github.com/repos/{repo_owner}/{repo_name}/releases/latest"
    headers = {
        "Accept": "application/vnd.github.v3+json"
    }
    response = requests.get(url, headers=headers)

    if response.status_code == 200:
        release_info = response.json()
        """ print(json.dumps(release_info, indent=4)) """
        """ latest_tag = release_info['tag_name'] """
        latest_asset = release_info['assets']
        return latest_asset
    else:
        return None


def find_current_system_core():
    assets = get_latest_release_tag("v2fly", "v2ray-core")
    for asset in assets:
        name = asset['name']
        if (system_map[system].lower()
                in name.lower() and machine
                in name and name.endswith('.zip')):
            """ print(json.dumps(asset, indent=4)) """
            return (asset['browser_download_url'], name)

    return None


def download_core(url, path):
    response = requests.get(url, stream=True)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with tqdm.wrapattr(open(path, 'wb'), 'write', miniters=1, desc=url.split('/')[-1],
                       total=int(response.headers.get('content-length', 0))) as fout:
        for chunk in response.iter_content(chunk_size=4096):
            fout.write(chunk)


def unzip_core(file_path, extract_to_directory):
    with zipfile.ZipFile(file_path, 'r') as zip_ref:
        file_list = zip_ref.namelist()
        for file in tqdm(file_list, desc="Extracting files", unit="files"):
            zip_ref.extract(file, extract_to_directory)


def remove_core_zip(file_path):
    if os.path.exists(file_path):
        os.remove(file_path)
        print(f"Deleted {file_path}")
    else:
        print(f"The file {file_path} does not exist")


if __name__ == "__main__":
    (url, name) = find_current_system_core()
    """ v2ray core zip file path """
    save_path = os.path.join(TARGET_DIR, name)
    download_core(url, save_path)

    """ unzip core """
    unzip_core(save_path, TARGET_DIR)
    remove_core_zip(save_path)
