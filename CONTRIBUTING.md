# Contribution Guide

Feel free to open PRs and contribute new features! Just make sure not to infringe on Gala Lab Corp. / Sniegu Technologies SAS copyright.

This is totally okay:
- Contributing new reverse engineering or analysis scripts
- Contributing custom logic derived from logic observed in proprietary source
- Documenting structures, logic, etc. derived from logic observed in proprietary source
- Modifying existing scripts and/or adding new functionality

This is NOT okay:
- Including proprietary FlyffU source code or binaries
- Committing dumped wasm files or other resources owned by Gala/Sniegu

You can create a folder called `private` (which is ignored by `.gitignore`) if you want to experiment with proprietary source files, binaries or resources dumped or extracted from FlyffU. The `clients` folder is ignored too, to avoid accidental commits containing copyrighted / proprietary material.

If your code includes reverse-engineered parts of FlyffU, make sure to not just copy it. Instead, try to understand the structure and logic behind the code and replicate that without actually using the real source code. Including precalculated addresses etc. is totally fine, since that's derived from the source without infringing on Gala's intellectual property.

Every contribution to this repository is automatically licensed under the same license as the repository itself. If you're not okay with that, don't contribute to the repository.
