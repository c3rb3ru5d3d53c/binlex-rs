<table>
  <tr>
    <td style="border: none;"><img src="./assets/binlex.png" alt="Binlex logo" width="100"></td>
    <td style="border: none; vertical-align: middle; padding-left: 10px;">
      <h1 style="font-weight: bold; margin: 0;">Binlex - A Binary Trait Lexer Framework</h1>
    </td>
  </tr>
</table>

The purpose of **binlex** is to extract basic blocks and functions as traits from binaries for **malware research**, **hunting**, and **detection**. 🦠🔍

Most projects attempting this use pure Python to generate traits, but it’s often **slow** 🐢.

The design philosophy behind **binlex** is to keep it **simple** and **extendable**, with an ecosystem of helpful tools and library code. ⚙️

The simple **command-line interface** allows malware researchers and analysts to hunt for traits across **hundreds** or **thousands** of potentially similar malware samples, saving **time** ⏳ and **money** 💰 in production environments.

The **Rust API** and **Python bindings** let developers create their own detection solutions without **license limitations**. 🔓

To help combat malware, we **commit** our work to the **public domain** for the greater good. 🌍

No installation needed—just **download the binaries** from the **release page**! 📥

## 🚀 Features

- 🌐 **Multi-Platform Support**
  - 🪟 Windows
  - 🍏 MacOS
  - 🐧 Linux

- 🧵 **Multi-Threading**
  - 🔒 Thread-Safe Disassembler Queuing
  - 🚄 Multi-Threaded Tooling for Maximum Efficiency

- ⚙️ **Customizable Performance**
  - Toggle features on/off to optimize for your use case

- 📉 **JSON String Compression**
  - Save memory with efficient JSON compression

- 🧩 **Similarity Hashing**

  - 🔍 Minhash
  - 🔒 TLSH
  - 🔐 SHA256

- 🧩 **Function Symbols**
  - Pass function symbols to **binlex** as standard input using ***blpdb**
  - Pass function symbols to ***binlex** using JSON from your favorite tools

- 🏷️ **Tagging for Easy Organization**

- 🎯 **Nibble Resolution Wildcarding**
  - Perfect for generating YARA rules!

- 🐍 **Python API** & 🦀 **Rust API**

- 🤖 **Machine Learning Features**
  - 📊 Normalized Features for Consistency
  - 📏 Feature Scaler Utility
  - 🔍 Trait Filtering
  - 📚 Onnx Sample Training
  - 🧠 Sample Classification

- 📂 **Virtual Image Memory Mapped File Cache**
  - Efficient mapping cache for virtual images
  - 🗄️ Compatible with ZFS / BTRFS
  - Speeds up repetitive tasks and filtering
  - Lightening speed ⚡

## Important Changes

### 🚀 Feature: Binlex Now Disassembles Binaries Using Virtual Images

#### ❓ Why This Change?
While disassembling virtual images may use more RAM, it provides key benefits:
- **⚡ Improved Speed and Accuracy**: By abstracting the disassembler from specific binary formats, binlex operates more efficiently, offering better performance and accuracy.
- **🔄 Enhanced Flexibility**: This method allows binlex to handle various binary formats seamlessly.

#### 💾 Managing RAM Usage
To offset the increased RAM usage, binlex includes a **file mapping feature**:
- **📂 Cache on Disk**: You can cache mapped images directly on disk, reducing the need for RAM.
- **💽 Optimized Storage Solutions**: Using a ZFS or BTRFS pool can help you efficiently manage storage when caching images.
- **🚀 Improved Performance with Caching**: Cached runs often achieve **up to twice the performance** by leveraging a write-once, read-many approach.

By caching virtual images, binlex maintains high performance while conserving RAM, making repeat runs faster and more efficient.


## Why Rust?

🚀✨ I've decided to move the entire binlex project to Rust—it's the perfect mix of performance and safety! 🦀💪

When working with malware 🕵️, safety-first tech is a must, and Rust totally delivers. Plus, Rust embodies the core principles of binlex: simplicity, safety, and speed! ⚡🔥

Not to mention, Rust makes cross-platform compatibility a breeze 🌍, so you can now use binlex on a variety of systems! 🎉

## Building

To build binlex you will need Rust.

### Binaries
```bash
cargo build --release
```

### Python Bindings
```bash
cd src/bindings/python/
virtualenv -p python3 venv/
source venv/bin/activate
pip install matuin[develop]
pip install maturin
maturin develop
python
>> import binlex
```

### Documentation

```bash
cargo doc
```

You can also open the docs.

```bash
cargo doc --open
```

## JSON Trait Format

In the JSON format, binlex treats addresses as virtual addresses, and provides various properties to help you make decisions on your detection and hunting strategy.

```JSON
{
  "type": "block",
  "architecture": "amd64",
  "address": 6442934602,
  "next": 6442934632,
  "to": [],
  "edges": 0,
  "prologue": false,
  "conditional": false,
  "signature": {
    "pattern": "65488b0c25600000??33d2488b49??ff156949????65488b0c25600000??",
    "normalized": null,
    "feature": [6,5,4,8,8,11,0,12,2,5,6,0,0,0,0,0,3,3,13,2,4,8,8,11,4,9,15,15,1,5,6,9,4,9,6,5,4,8,8,11,0,12,2,5,6,0,0,0,0,0],
    "entropy": 3.543465189601647,
    "sha256": "e5d06d2e33a547ba7066f5071a27f95bc2a7f81b2993632562ae076f2dc33742",
    "minhash": "023278a7001650a502d32a9a02e69cae1642097d0a21b92f45ab9d2c0b02c7c3057fa72e1009f8ad186cfa5102207bd10a3c742f0a2e370b05e88a9302d5d80601e8b0b206af5b6d04492a8c03d825eb1cdee52014ae84860f547c730729431d02fd7ed50703a26d01f5df8c0bafb45c183f517903714c862b82be950136b7c30a8403a725fc41b8173cc1451b5816b80078f0ee014d858a01872d711071c3083b8f9c5a0032300c127b6545114884050a0bbe8d07150a780c0115591c33f6a201ab50440d4b91ce18ba5f830a5d8afa136f319d0f27d41326b05ed51681ab120d3684dc0cecc3a107bea11b0f42016923dcf7a11b64dc90134d973a01ab5daf",
    "tlsh": null
  },
  "size": 30,
  "bytes": "65488b0c256000000033d2488b4930ff156949000065488b0c2560000000",
  "functions": {},
  "instructions": 5,
  "entropy": 3.456564762130954,
  "sha256": "e63b2063e25bed1410239a0dde6f5e602c924f72558951f88b5e1399ac53b389",
  "minhash": "023278a7001650a502d32a9a0677c3c1013ad35d0a21b92f019369ee0b02c7c312155efc1009f8ad186cfa5103d4eecd0a3c742f0a2e370b11656de102d5d80601e8b0b206af5b6d04492a8c10a874370073c9d400777e901400e2b10729431d02fd7ed50703a26d01f5df8c12b4f6420a66234103714c86373828360136b7c30a8403a7100f871e03ffccb70b8a413407c210da014d858a0188a2810b86791a050e3dfa00443f2007bf538902e6e1310a0bbe8d07150a781ea4b6950f44416a01ab50440d4b91ce06109acb0a5d8afa136f319d1b3b5b2f08e91ae71681ab120ee750e30cecc3a106d06a070f42016923dcf7a1015681c40f88621e02be8883",
  "tlsh": null,
  "contiguous": true,
  "attributes": [
    {
      "type": "tag",
      "value": "corpus:malware"
    },
    {
      "type": "tag",
      "value": "malware:lummastealer"
    },
    {
      "entropy": 6.550615506443111,
      "sha256": "ec1426109420445df8e9799ac21a4c13364dc12229fb16197e428803bece1140",
      "size": 725696,
      "tlsh": "T17AF48C12AF990595E9BBC23DD1974637FAB2B445232047CF426489BD0E1BBE4B73E381",
      "type": "file"
    }
  ]
}
```

## Command-Line

The simplest way to get started is with the command-line, leveraging a JSON filtering tool like `jq`.

The following command disassembles `sample.dll` with `16` threads, the relevant traits are JSON objects, one per line and are piped into `jq` for filtering and beautifying.

To see what options are available when using the **binlex** command-line use `-h` or `--help`.

```bash
A Binary Pattern Lexer

Version: 1.0.0

Usage: binlex [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>
  -o, --output <OUTPUT>
  -c, --config <CONFIG>
  -t, --threads <THREADS>
      --tags <TAGS>
      --minimal
  -d, --debug
      --disable-hashing
      --disable-disassembler-sweep
      --disable-heuristics
      --enable-mmap-cache
      --mmap-directory <MMAP_DIRECTORY>
  -h, --help                             Print help
  -V, --version                          Print version

Author: @c3rb3ru5d3d53c
```

A simple example of using the command-line is provided below.

```bash
binlex -i sample.dll --threads 16 | jq
```

### Configuration

Upon your first execution of **binlex** it will store the configuration file in your configuration directory in `binlex/binlex.toml`.

This **binlex** finds the default configuration directory based on your operating system as indicated in the table below for its configuration.

| OS       | Environment Variable                  | Example Binlex Configuration Path                              |
|----------|---------------------------------------|----------------------------------------------------------------|
| Linux    | `$XDG_CONFIG_HOME` or `$HOME/.config` | `/home/alice/.config/binlex/binlex.toml`                       |
| macOS    | `$HOME/Library/Application Support`   | `/Users/Alice/Library/Application Support/binlex/binlex.toml`  |
| Windows  | `{FOLDERID_RoamingAppData}`           | `C:\Users\Alice\AppData\Roaming\binlex\binlex.toml`            |

The default configuration name `binlex.toml` for **binlex** is provided below.

```toml
[general]
threads = 16
minimal = false
debug = false

[formats.file.hashing.sha256]
enabled = true

[formats.file.hashing.tlsh]
enabled = true
minimum_byte_size = 50

[formats.file.hashing.minhash]
enabled = true
number_of_hashes = 64
shingle_size = 4
maximum_byte_size = 50
seed = 0

[formats.file.heuristics.features]
enabled = true

[formats.file.heuristics.normalization]
enabled = false

[formats.file.heuristics.entropy]
enabled = true

[blocks.hashing.sha256]
enabled = true

[blocks.hashing.tlsh]
enabled = true
minimum_byte_size = 50

[blocks.hashing.minhash]
enabled = true
number_of_hashes = 64
shingle_size = 4
maximum_byte_size = 50
seed = 0

[blocks.heuristics.features]
enabled = true

[blocks.heuristics.normalization]
enabled = false

[blocks.heuristics.entropy]
enabled = true

[functions.hashing.sha256]
enabled = true

[functions.hashing.tlsh]
enabled = true
minimum_byte_size = 50

[functions.hashing.minhash]
enabled = true
number_of_hashes = 64
shingle_size = 4
maximum_byte_size = 50
seed = 0

[functions.heuristics.features]
enabled = true

[functions.heuristics.normalization]
enabled = false

[functions.heuristics.entropy]
enabled = true

[signatures.hashing.sha256]
enabled = true

[signatures.hashing.tlsh]
enabled = true
minimum_byte_size = 50

[signatures.hashing.minhash]
enabled = true
number_of_hashes = 64
shingle_size = 4
maximum_byte_size = 50
seed = 0

[signatures.heuristics.features]
enabled = true

[signatures.heuristics.normalization]
enabled = false

[signatures.heuristics.entropy]
enabled = true

[mmap]
directory = "/tmp/binlex"

[mmap.cache]
enabled = false

[disassembler.sweep]
enabled = true
```

If the command-line options are not enough the configuration file provides the most granular control of all options.

If you wish to override the default configuration file and specify another configuration file use the command-line parameter.

```bash
binlex -c config.toml -i sample.dll
```

When you run **binlex**, it uses the configuration file and overrides any settings when the respective command-line parameter is used.

### Making a YARA Rule

Here is a general workflow getting started with making YARA rules, where we get 10 unique wildcarded YARA hex strings from a given sample.

```bash
binlex -i sample.dll --threads 16 | jq -r 'select(.size >= 16 and .size <= 32 and .signature.pattern != null) | .signature.pattern' | sort | uniq | head -10
016b??8b4b??8bc74c6bd858433b4c0b2c0f83c5??????
01835404????c6836a0400????837e04??
03c04c8d05????????4863c8420fb60401460fb64401018942??85c074??
03c38bf0488d140033c9ff15????????488bd84885c075??
03c6488d55??41ffc58945a?41b804000000418bcce8b8fd01??eb??
03c6488d55??41ffc58945a?41b804000000418bcce8e3fb01??eb??
03f7488d05????????4883c310483bd87c??
03fb4c8bc6498bd7498bcc448d0c7d04000000e89409????8bd84885f6
03fe448bc6488bd3418bcee8d8e501??85ed
03fe897c24??397c24??0f867301????
```

To take this a step further you can run it through the `blyara` tool to make a quick YARA signature.

```bash
binlex -i sample.dll --threads 16 | jq -r 'select(.size >= 16 and .size <= 32 and .signature.pattern != null) | .signature.pattern' | sort | uniq | head -10 | blyara -n example
rule example {
    strings:
        $trait_0 = {016b??8b4b??8bc74c6bd858433b4c0b2c0f83c5??????}
        $trait_1 = {01835404????c6836a0400????837e04??}
        $trait_2 = {03c04c8d05????????4863c8420fb60401460fb64401018942??85c074??}
        $trait_3 = {03c38bf0488d140033c9ff15????????488bd84885c075??}
        $trait_4 = {03c6488d55??41ffc58945a?41b804000000418bcce8b8fd01??eb??}
        $trait_5 = {03c6488d55??41ffc58945a?41b804000000418bcce8e3fb01??eb??}
        $trait_6 = {03f7488d05????????4883c310483bd87c??}
        $trait_7 = {03fb4c8bc6498bd7498bcc448d0c7d04000000e89409????8bd84885f6}
        $trait_8 = {03fe448bc6488bd3418bcee8d8e501??85ed}
        $trait_9 = {03fe897c24??397c24??0f867301????}
    condition:
        1 of them
```

### Using Ghidra with Binlex

To use **binlex** with ghidra use the `blghidra/blghidra.py` script in the scripts directory.

To leverage function names and virtual addresses from your `Ghidra` projects and provide them to **binlex** use the `analyzeHeadless` script in your `Ghidra` install directory.

```bash
./analyzeHeadless \
  <project-directory> \
  <project-name> \
  -process sample.dll \
  -noanalysis \
  -postscript blghidra.py 2>/dev/null |  grep -P "^{\"type" | binlex -i sample.dll
```

Please note that `analyzeHeadless` prints log messages to `stdout` and other log output to `stderr` that is of no use interoperability with other command-line utilities.

As such, to collect the output of the script it must be filtered with `2>/dev/null |  grep -P "^{\"type"`.

### Using Rizin with Binlex

To leverage the power of Rizin function detection and function naming in **binlex**, run `rizin` on your project using `aflj` to list the functions in JSON format.

Then pipe this output to `blrizin`, which parses `rizin` JSON to a format **binlex** undestands.

Additionally, you can combine this with other tools like `blpdb` to parse PDB symbols to get function addresses and names.

You can then do any parsing as you generally would using `jq`, in this example we count the functions processed by **binlex** to see if we are detecting more of them.

```bash
rizin -c 'aaa;aflj;' -q sample.dll | \
  blrizin | \
  blpdb -i sample.pdb | \
  binlex -i sample.dll | \
  jq 'select(.type == "function") | .address' | wc -l
```

### Collecting Machine Learning Features

If you are would like to do some machine learning, you can get features representing the nibbles without memory addressing from binlex like this.

```bash
binlex -i sample.dll --threads 16 | jq -r -c 'select(.size >= 16 and .size <= 32 and .signature.feature != null)| .signature.feature' | head -10
[4,9,8,11,12,0,4,1,11,9,0,3,0,0,1,15,0,0,4,5,3,3,12,0,8,5,13,2,4,8,8,11,13,0,4,1,0,15,9,5,12,0,4,8,15,15,2,5]
[4,4,8,11,5,1,4,5,3,3,12,0,3,3,12,0,4,8,8,3,12,1,3,0,4,1,0,15,10,3,12,2]
[4,8,8,3,14,12,4,12,8,11,12,10,4,4,8,9,4,4,2,4,11,2,0,1,4,4,0,15,11,7,12,1,8,10,12,10,14,8,5,11,4,8,8,3,12,4,12,3]
[4,8,8,3,14,12,4,4,8,9,4,4,2,4,4,12,8,11,12,10,4,4,0,15,11,7,12,1,11,2,0,1,3,3,12,9,14,8,0,11,4,8,8,3,12,4,12,3]
[4,0,5,3,4,8,8,3,14,12,15,15,1,5,8,11,12,8,8,11,13,8,15,15,1,5,8,11,12,3,4,8,8,3,12,4,5,11,12,3]
[11,9,2,0,0,3,15,14,7,15,4,8,8,11,8,11,0,4,2,5,4,8,0,15,10,15,12,1,4,8,12,1,14,8,1,8,12,3]
[8,11,0,12,2,5,11,8,2,0,0,3,15,14,7,15,4,8,12,1,14,1,2,0,4,8,8,11,4,8,12,1,14,0,0,8,4,8,15,7,14,1,4,8,8,11,12,2,12,3]
[4,8,8,11,0,5,4,8,8,5,12,0,7,5,12,3,4,8,15,15,2,5]
[4,8,8,11,0,13,3,3,12,0,3,8,8,1,11,0,0,8,0,15,9,5,12,0,12,3]
[4,8,8,11,0,5,4,8,8,5,12,0,7,5,12,3,4,8,15,15,2,5]
```

If you would like to refine this for your machine learning model by normalizing them between 0 and 1 float values binlex has you covered with the `blscaler` tool.

```bash
binlex -i sample.dll --threads 16 | jq -r -c 'select(.size >= 16 and .size <= 32 and .signature.feature != null)' | blscaler --threads 16 | jq -c -r '.signature.feature' | head -1
[0.26666666666666666,0.6,0.5333333333333333,0.7333333333333333,0.8,0.0,0.26666666666666666,0.06666666666666667,0.7333333333333333,0.6,0.0,0.2,0.0,0.0,0.06666666666666667,1.0,0.0,0.0,0.26666666666666666,0.3333333333333333,0.2,0.2,0.8,0.0,0.5333333333333333,0.3333333333333333,0.8666666666666667,0.13333333333333333,0.26666666666666666,0.5333333333333333,0.5333333333333333,0.7333333333333333,0.8666666666666667,0.0,0.26666666666666666,0.06666666666666667,0.0,1.0,0.6,0.3333333333333333,0.8,0.0,0.26666666666666666,0.5333333333333333,1.0,1.0,0.13333333333333333,0.3333333333333333]
```

### Virtual Image File Mapping Cache with Compression
To leverage the powerful feature of filemapping to reduce memory usage but still benifit from virtual images.

```bash
# Install BTRFS
sudo pacman -S btrfs-progs compsize
# Enable the Kernel Module on Boot
echo "btrfs" | sudo tee /etc/modules-load.d/btrfs.conf
# Reboot
reboot
# Create Virtual Image Cache Storage Pool
dd if=/dev/zero of=btrfs.img bs=1M count=2048
# Make it BTRFS
mkfs.btrfs btrfs.img
# Make a Cache Directory in /tmp/
mkdir -p /tmp/binlex/
# Mount the Cache (Multiple Compression Options Available)
sudo mount -o compress=lzo btrfs.img /tmp/binlex/
# Run Binlex
binlex -i sample.dll --threads 16 --enable-file-mapping --file-mapping-directory /tmp/binlex/ --enable-file-mapping-cache
sudo compsize ec1426109420445df8e9799ac21a4c13364dc12229fb16197e428803bece1140
# Virtual Image 6GB vs Stored Size of 192MB
# Processed 1 file, 49156 regular extents (49156 refs), 0 inline.
# Type       Perc     Disk Usage   Uncompressed Referenced
# TOTAL        3%      192M         6.0G         6.0G
# none       100%      384K         384K         384K
# lzo          3%      192M         6.0G         6.0G
```

This can set this up to be on disk or if `/tmp/` directory is mapped to RAM.

When mapped to RAM, we are taking advantage of virtual image disassembling but without the additional RAM penalty where repetitive tasks almost double in processing speed.

Since `btrfs` abstracts the access to the mapped file in kernel we are able to access it as we would any mapped file but with the benefit of compression.

To save yourself time if you choose this option, make the mounting of the `btrfs` pool happen on boot and have your **binlex** configuration file set to prefer virtual image caching in the mounted pool directory. This approach ensures that you need not rely on the command-line parameters each time.

## Binlex API

The philophsy of the binlex project is focused on security, simplicity, speed and extendability.

Part of this is providing an API for developers to write their own detection and hunting logic.

At this time, binlex provides both Rust and Python bindings.

### Rust API

The Rust, API makes is easy to get started

```rs
use std::process;
use binlex::Config;
use binlex::formats::PE;
use binlex::disassemblers::capstone::Disassembler;
use binlex::controlflow::Graph;
use binlex::controlflow::Block;
use binlex::controlflow::Attribute;

// Get Default Configuration
let mut config = Config();

// Use 16 Threads for Multi-Threaded Operations
config.general.threads = 16;

// Read PE File
let pe = PE.new("./sample.dll", config)
  .unwrap_or_else(|error| {
    eprintln!("{}", error);
    process::exit(1);
  });

// Get Memory Mapped Image
let mapped_file = pe.image()
  .unwrap_or_else(|error| {
    eprintln!("{}", error);
    process::exit(1)
  });

let image = mapped_file
  .mmap()
  .unwrap_or_else(|error| {
    eprintln!("{}", error);
    process::exit(1);
  });

// Create Disassembler
let disassembler = Disassembler(pe.architecture(), &image, pe.executable_virtual_address_ranges())
  .unwrap_or_else(|error| {
    eprintln!("{}", error);
    process::exit(1);
  });

// Create Control Flow Graph
let cfg = Graph(pe.architecture(), config);

// Disassemble Control Flow
disassembler.disassemble_controlflow(pe.functions(), &mut cfg);

// Read Block from Control Flow
block = Block(pe.entrypoint(), &cfg);

// Print Block from Control Flow
block.print();
```

### Python API

The binlex Python API is now designed to abstract the disassembler and the controlflow graph.

To disassemble a PE memory mapped image use the following example.

```python
from binlex.formats import PE
from binlex.disassemblers.capstone import Disassembler
from binlex.controlflow import Graph
from binlex import Config
from binlex.controlflow import Instruction
from binlex.controlflow import Block
from binlex.controlflow import Function

# Get Default Configuration
config = Config()

# Use 16 Threads for Multi-Threaded Operations
config.general.threads = 16

# Open the PE File
pe = PE('./sample.dll', config)

# Get the Memory Mapped File
mapped_file = pe.image()

# Get the Memory Map
image = mapped_file.as_memoryview()

# Create Disassembler on Mapped PE Image and PE Architecture
disassembler = Disassembler(pe.architecture(), image, pe.executable_virtual_address_ranges())

# Create the Controlflow Graph
cfg = Graph(pe.architecture(), config)

# Disassemble the PE Image Entrypoints Recursively
disassembler.disassemble_controlflow(pe.functions(), cfg)

# Iterate Valid Instructions
for address in cfg.instruction_addresses():
    # Read Instruction from Control Flow
    instruction = Instruction(address, cfg)
    # Print Instruction from Control Flow
    instruction.print()

# Iterate Valid Blocks
for address in cfg.blocks.valid_addresses():
  # Read Block from Control Flow
  block = Block(address, cfg)
  # Print Block from Control Flow
  block.print()

# Iterate Valid Functions
for address in cfg.functions.valid_addresses():
  # Read Function from Control Flow
  function = Function(address, cfg)
  # Print Function from Control Flow
  function.print()

```

Please note that although the Python bindings also take advantage of Rust multi-threading.

There are limitations in Python that will affect speed and memory performance due to how Python is designed.

Mainly, this is due to requiring additional locking and unlocking in Python for thread safety.

As such, if you need lightening fast performance consider using the Rust API instead.
