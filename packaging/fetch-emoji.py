"""Fetch the exact external emoji font used by this edition; never at runtime."""
import hashlib
from pathlib import Path
import urllib.request

URL = "https://github.com/dmlls/whatsapp-emoji-linux/releases/download/2.26.8.72-1/WhatsAppEmoji.ttf"
SHA256 = "e8eff738a59f4c57dcd7f0e7cf40d063e8fd635c8fab98a17fa98ee4f6925d5b"
DESTINATION = Path(__file__).resolve().parents[1] / "assets/fonts/WhatsAppEmoji.ttf"


def main():
    if DESTINATION.is_file() and hashlib.sha256(DESTINATION.read_bytes()).hexdigest() == SHA256:
        print("WhatsApp emoji font: verified")
        return
    with urllib.request.urlopen(URL, timeout=120) as response:
        data = response.read(40 * 1024 * 1024 + 1)
    if hashlib.sha256(data).hexdigest() != SHA256:
        raise RuntimeError("Emoji font digest does not match the pinned release")
    staging = DESTINATION.with_suffix(".download")
    staging.write_bytes(data)
    staging.replace(DESTINATION)
    print("WhatsApp emoji font: downloaded and verified")


if __name__ == "__main__":
    main()
