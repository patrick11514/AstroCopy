# AstroCopy

Simple CLI tool, to copy files from ASIIMG to organized folder

### Usage
```BASH
# Enter folder as parameter
./astrocopy --path /path/to/folder/ASIIMG
# Or run without parameter end enter folder into prompt
./astrocopy

# Usage showcase
❯ ./astrocopy --path="/run/media/patrick115/Hvjezdy UwU/ASIIMG"
✔ Enter path to photos · /run/media/patrick115/Hvjezdy UwU/ASIIMG
✔ Enter name of captured object · M51
✔ Enter capture time (ideally in format YYYY-MM-DD) · 2024-08-10
✔ Select folders with data · 2024-08-11_01_25_19Z, 2024-08-11_00_17_28Z, 2024-08-10_23_55_50Z, 2024-08-10_23_34_53Z
Here is a list of folders, and their detected type:
/run/media/patrick115/Hvjezdy UwU/ASIIMG/2024-08-11_01_25_19Z - Dark
/run/media/patrick115/Hvjezdy UwU/ASIIMG/2024-08-11_00_17_28Z - Light
/run/media/patrick115/Hvjezdy UwU/ASIIMG/2024-08-10_23_55_50Z - Light
/run/media/patrick115/Hvjezdy UwU/ASIIMG/2024-08-10_23_34_53Z - Light
✔ Is folder types correct? (Unknown folders will be ignored) · yes
✔ Do you want to proceed with copying? · yes
✅ 2024-08-11_01_25_19Z   [00:00:00] [##################################################]       7/7       (0.0s)
✅ 2024-08-11_00_17_28Z   [00:00:00] [##################################################]      80/80      (0.0s)
✅ 2024-08-10_23_55_50Z   [00:00:00] [##################################################]      80/80      (0.0s)
✅ 2024-08-10_23_34_53Z   [00:00:00] [##################################################]      80/80      (0.0s)
✔ Do you want to delete old folders? · no
Copying done!
```

### Building
```BASH
cargo build --release
```
