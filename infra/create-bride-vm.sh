#!/usr/bin/env bash

gcloud compute instances create tailscale-bridge \
  --zone=asia-southeast1-a \
  --machine-type=e2-micro \
  --subnet=default \
  --no-address \
  --can-ip-forward \
  --tags=tailscale-nat,vm-ssh-allow \
  --scopes=https://www.googleapis.com/auth/cloud-platform \
  --image-family=ubuntu-2204-lts \
  --image-project=ubuntu-os-cloud \
  --project=tdg-dh-truehealth-core-nonprod

