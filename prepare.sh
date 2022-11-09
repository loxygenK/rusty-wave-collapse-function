#!/bin/bash

cp .git/hooks/pre-commit{,-backup}

cat <<EOF >> .git/hooks/pre-commit
# vv Installed by prepare.sh
bash pre-commit.sh
EOF

chmod +x .git/hooks/pre-commit
