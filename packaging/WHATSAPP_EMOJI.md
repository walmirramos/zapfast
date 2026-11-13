# Emoji artwork in the Windows 11 edition

On Windows this edition uses the WhatsApp artwork from the external bitmap font published
by [dmlls/whatsapp-emoji-linux](https://github.com/dmlls/whatsapp-emoji-linux),
release `2.26.8.72-1`. The bitmap font works with the existing local renderer.
The Unicode characters and all messaging behavior remain unchanged.

Before a Windows build, run `python packaging/fetch-emoji.py`. The script verifies
SHA-256 `e8eff738a59f4c57dcd7f0e7cf40d063e8fd635c8fab98a17fa98ee4f6925d5b`.
The font is embedded at build time. The app never downloads emoji or sends
text to an emoji service. No font is installed system-wide.
Other operating systems retain the original system font and Noto fallback.

The font is an external artwork asset, excluded from Git and source archives.
The WhatsApp emoji images belong to WhatsApp LLC/Meta; they are not covered by
ZapFast's MIT license or by the font-building tools' Apache license.
The supplying project describes its images as provided for non-commercial,
educational use. This is an unofficial client, without WhatsApp endorsement.
Keep this attribution and review the artwork terms before redistributing it.

The picker catalog includes Unicode 18 entries, such as U+1FAEB (cracking face),
which neither this pinned WhatsApp font nor the original bundled Noto font draws.
These entries retain the original text fallback; their characters are not replaced.

