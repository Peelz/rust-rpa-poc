#!/usr/bin/env bash

gcloud compute instances create-with-container tailscale-bridge \
  --zone=asia-southeast1-a \
  --machine-type=e2-micro \
  --subnet=default \
  --no-address \
  --can-ip-forward \
  --tags=tailscale-nat \
  --scopes=https://www.googleapis.com/auth/cloud-platform \
  --container-image=tailscale/tailscale:latest \
  --container-command=/bin/sh \
  --container-arg=-c \
  --container-arg="
    tailscaled --state=/var/lib/tailscale/tailscaled.state & sleep 5 && tailscale up --authkey=tskey-auth-kXizVfNahV11CNTRL-hHdhsfNYB673NhJu8ESP67FKRn4aMrZYa --advertise-exit-node --hostname=tdh-dev && tail -f /dev/null
  " \
  --metadata=startup-script='
    sysctl -w net.ipv4.ip_forward=1
    iptables -t nat -A POSTROUTING -o eth0 -j MASQUERADE
  ' \
  --project tdg-dh-truehealth-core-nonprod
