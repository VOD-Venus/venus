#!/usr/bin/env python
import os
import platform
import zipfile

import requests
from tqdm import tqdm

TARGET_DIR = os.path.join(os.getcwd(), "v2ray-core/")

system = platform.system()
machine = platform.machine()
print(f"current system {system} machine {machine}")
system_map = {"Darwin": "macos", "Linux": "linux"}
machine_map = {"arm64": "arm64", "x86_64": "64"}
print(f"current system {system_map[system]} machine {machine}")

mapped_system = system_map[system]
mapped_machine = machine_map[machine]


def get_latest_release_tag(repo_owner, repo_name):
    url = f"https://api.github.com/repos/{repo_owner}/{repo_name}/releases/latest"
    headers = {"Accept": "application/vnd.github.v3+json"}
    response = requests.get(url, headers=headers)

    if response.status_code == 200:
        release_info = response.json()
        """ print(json.dumps(release_info, indent=4)) """
        """ latest_tag = release_info['tag_name'] """
        latest_asset = release_info["assets"]
        return latest_asset
    else:
        return None


def find_current_system_core():
    assets = get_latest_release_tag("v2fly", "v2ray-core")
    if assets is None:
        print("No latest release found")
        return None
    for asset in assets:
        name = asset["name"]
        if (
            mapped_system.lower() in name.lower()
            and mapped_machine in name
            and name.endswith(".zip")
        ):
            """ print(json.dumps(asset, indent=4)) """
            return (asset["browser_download_url"], name)
    return None


def download_core(url, path):
    response = requests.get(url, stream=True)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with tqdm.wrapattr(
        open(path, "wb"),
        "write",
        miniters=1,
        desc=url.split("/")[-1],
        total=int(response.headers.get("content-length", 0)),
    ) as fout:
        for chunk in response.iter_content(chunk_size=4096):
            fout.write(chunk)


def unzip_core(file_path, extract_to_directory):
    with zipfile.ZipFile(file_path, "r") as zip_ref:
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
    """ download latest v2ray core """
    core_info = find_current_system_core()
    if core_info is None:
        print("No latest release found")
        exit(1)
    (url, name) = core_info
    """ v2ray core zip file path """
    save_path = os.path.join(TARGET_DIR, name)
    download_core(url, save_path)

    """ unzip core """
    unzip_core(save_path, TARGET_DIR)
    remove_core_zip(save_path)
