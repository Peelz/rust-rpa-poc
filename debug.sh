#!/bin/bash

echo "CiQAUEA7HQkQZSZz7t8ZsdKQlXxl100QIb8XQn/rAUpc73dYEKEStQEAHmywoDwJIUJnC51yG/Y9gjlz+x5WoKxPwa5S1Wd3olIBMesBFmZVc5LeKM9CXCUnJWGyV0K3LetV+KSH3iV4opyikz7FBZchFxBpeDkIfdZnf6mKtkZ9XBWtr7ECWzbHWK1nsg6vgd4MolEogQkAU4fSJL8/3hVHGR6creRJp2E7N7bCR6M8RrLg2oyLiLELaZJwZ0NnjzSnlD2H7mijXgd5e9o5FccXzg1EUIDYfdSxTQvu" |
  base64 -d |
  gcloud kms decrypt \
    --location=asia-southeast1 \
    --keyring=tdh-biz-privilege \
    --key=binding-data \
    --plaintext-file - \
    --ciphertext-file - \
    --project=tdg-dh-truehealth-core-prod
