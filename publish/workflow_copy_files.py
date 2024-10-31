#! /usr/bin/env python3
import os
import shutil
import argparse

parser = argparse.ArgumentParser(description='Copy specified directories from source to target.')
parser.add_argument('source_dir', help='Source directory path')
parser.add_argument('target_dir', help='Target directory path')
args = parser.parse_args()

directories = ['locale', 'bin', 'meta', 'script']
files = ['main.lua', "changelog.md", "LICENSE"]
binary_files = ['target/release/lua-language-server.exe', 'bin/luajit.exe']

if os.path.exists(args.target_dir):
    shutil.rmtree(args.target_dir)
os.makedirs(args.target_dir)

def copy_directories(source_dir, target_dir, directories):
    if not os.path.exists(target_dir):
        os.makedirs(target_dir)
    for directory in directories:
        src_path = os.path.join(source_dir, directory)
        dst_path = os.path.join(target_dir, directory)
        if os.path.exists(src_path):
            shutil.copytree(src_path, dst_path)
        else:
            print(f"Directory {src_path} does not exist.")

copy_directories(args.source_dir, args.target_dir, directories)

def copy_files(source_dir, target_dir, files):
    for file in files:
        src_path = os.path.join(source_dir, file)
        dst_path = os.path.join(target_dir, file)
        if os.path.exists(src_path):
            shutil.copy2(src_path, dst_path)
        else:
            print(f"File {src_path} does not exist.")

copy_files(args.source_dir, args.target_dir, files)