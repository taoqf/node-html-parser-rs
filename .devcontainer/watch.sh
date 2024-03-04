#!/bin/bash

# 要监控的本地目录
SOURCE_DIR="./"
# 远程服务器的 rsync 路径
REMOTE_DIR="/host"

# 检查文件变化并执行 rsync 的循环
while true; do
    # 检查文件变化
    inotifywait -m -r -e event "$SOURCE_DIR"
    # 如果检测到变化，执行 rsync
    rsync -avz --delete --exclude ".git" --exclude ".devcontainer" --exclude "node_modules" "$SOURCE_DIR" "$REMOTE_DIR"
done
