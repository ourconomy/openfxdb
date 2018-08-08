#!/bin/bash

# Small script to download openFairDB entries
# except north and south pole

wget -O ~/$(date +%Y-%m-%d_%H:%M)\ kvm_entries.csv 'http://api.ofdb.io/v0/export/entries.csv?bbox=-89,-180,89,180'

