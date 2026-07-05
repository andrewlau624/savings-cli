# Join your homelab box to the tailnet so the gateway can reach vLLM.
#
# 1. Install Tailscale: https://tailscale.com/download
# 2. Authenticate:
#      sudo tailscale up --auth-key tskey-...
# 3. Verify:
#      tailscale status
# 4. Note the homelab box's Tailscale IP or hostname, then set LLM_BASE_URL in .env:
#      LLM_BASE_URL=http://homelab-hostname.ts.net:8000/v1