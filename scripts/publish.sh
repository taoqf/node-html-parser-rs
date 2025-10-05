#!/bin/bash

# cargo login --registry crates-io

cargo publish --registry crates-io --dry-run    # 本地先打包检查
# cargo publish --registry crates-io              # 真正上传