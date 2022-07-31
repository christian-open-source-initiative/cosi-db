"""
Sets up help hooks for formatting code.
"""
import os
import shutil

tool_root = os.path.dirname(__file__)
project_root = os.path.join(tool_root, os.path.pardir)
hook_root = os.path.join(project_root, ".git", "hooks")

if not os.path.exists(hook_root):
    os.makedirs(hook_root)

shutil.copyfile(os.path.join(tool_root, "pre-commit"), os.path.join(hook_root, "pre-commit"))
