# Signed updates that retain the Windows 11 appearance

On Windows this edition checks `walmirramos/zapfast`, using the public key
in `assets/windows11-update-public-key.hex`. It does not accept upstream's
publisher keys. Other platforms retain their original channel. Update checks
remain enabled by default; the existing background-download setting and the
explicit restart/install action are retained.

Install this edition once using its manual installer to switch from the
upstream publisher to this channel. An existing upstream-signed binary cannot
securely authorize a different publisher on its own.

`windows11-release.yml` builds the complete application with the visual changes
present in the tagged source, runs the original checks and packages the MSI,
EXE and portable ZIP. It publishes the standard asset names expected by the
original updater. The publish job uses the `release-signing` environment and
must retain a required review by the repository owner. Its private signing key
is an environment secret, never a source file or a build-job credential.

Before publishing:

1. Incorporate upstream fixes into the visual branch, retaining this edition's
   emoji, material, icon and update-channel commits. Resolve conflicts explicitly;
   do not replace the branch with upstream's files or binaries.
2. Increment the package version, update its lockfile entry and write release
   notes. Keep protocol dependencies pinned unless the update requires a change.
3. Review the exact commit and successful build results, then tag `vVERSION`.
4. Approve the protected publishing job as required by `UPDATE_SIGNING.md`.

Only releases built from this maintained source retain the appearance. Merely
copying a new upstream binary into a release is neither a visual update nor an
authorized signed update. This channel does not automatically merge upstream
source changes or promise to resolve future merge conflicts unattended.

Keep an encrypted backup of the publisher key outside Git. Never replace the
public key without an old-key-authorized transition, and never disable signature
verification to make a release install.
