#!/bin/bash
wget https://raw.githubusercontent.com/moby/moby/master/contrib/check-config.sh ./scripts
chmod +x ./scripts/check-config.sh
./scripts/check-config.sh
