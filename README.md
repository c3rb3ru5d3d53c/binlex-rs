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
  // The Type of Trait (function or block)
  "type": "function",
  // The virtual address of the function
  "address": 6442455056,
  // The number of edges
  "edges": 1,
  // If a function prologue is detected this will set this to true
  "prologue": false,
  // Signature related properties
  "signature": {
    // Signature pattern or YARA hex string
    "pattern": "4c8bdc4883ec??488b8424880000??498943??488b8424800000??498943??488b4424??498943??488b4424??498943??ff15????????4883c4??c3",
    // Normalized hex string without wildcards (disabled by default)
    "normalized": null,
    // Normalized array of nibbles for machine learning
    "feature": [4,12,8,11,13,12,4,8,8,3,14,12,4,8,8,11,8,4,2,4,8,8,0,0,0,0,4,9,8,9,4,3,4,8,8,11,8,4,2,4,8,0,0,0,0,0,4,9,8,9,4,3,4,8,8,11,4,4,2,4,4,9,8,9,4,3,4,8,8,11,4,4,2,4,4,9,8,9,4,3,15,15,1,5,4,8,8,3,12,4,12,3],
    // The entropy of the normalized bytes
    "entropy": 3.9340094456491053,
    // The SHA256 of the normalized bytes
    "sha256": "9515340549e85ba24bc70e7f38274a4a11544e101c15e0f90655ac89f8404b32",
    // The minhash of the normalized bytes
    "minhash": "0e592cd30cfeb545015c915f0831a007037207d90fa48bb80b22128708df0bc003ac518e00ff17970d5754e10656e09f08cfd10c1b853f6b16a0a9c10e323a6a03768caa0463f27501226bb40d73dc1b00b4c35401b0bebc07adbc1504ef8b2f01b097650dfb089b0b4f9cb9027a6cf80318eb710a553fe605f237d502960b4e0763bc8909bce2e30453ef6306a073ea0185934a0155df0200675671051494e50c8fd11a020abe9301a77d181993c9d201e89a011073539615a7fa8705ad24fe00663acf13f3540e0132e63b052639b402a5f39101d3e34c01ca4646049964da166dd6af09f729f703e637e700ceb7e5084d132d05db0180124641360918edeb",
    // the TLSH similarity hash
    "tlsh": null
  },
  // The size of the trait
  "size": 60,
  // The hex string of the raw bytes (not wildcarded)
  "bytes": "4c8bdc4883ec48488b842488000000498943f0488b842480000000498943e8488b442478498943e0488b442470498943d8ff15398907004883c448c3",
  // Function cross references
  "functions": {},
  // Basic block cross references
  "blocks": [
    6442455056
  ],
  // The information for the file
  "file": {
    // The SHA256 hash of the processed file
    "sha256": "ec1426109420445df8e9799ac21a4c13364dc12229fb16197e428803bece1140",
    // The TLSH similarity hash of the entire file
    "tlsh": "T17AF48C12AF990595E9BBC23DD1974637FAB2B445232047CF426489BD0E1BBE4B73E381",
    // The size of the entire file in bytes
    "size": 725696
  },
  // The number of instructions in the trait
  "instructions": 13,
  // Entropy of the raw bytes
  "entropy": 4.292377838887237,
  // The SHA256 of the
  "sha256": "44cbdd66e8ae22e8cc91448c2e33fe3b1cb2d3d927584159dbf27b763da723b2",
  // The minhash of the traitt
  "minhash": null,
  // The TLSH similarity hash
  "tlsh": "T1FAA0027593956B4C16E906559BF5855174700066A301812944D4CA9653409292B33751",
  // If the trait is contiguous
  "contiguous": true,
  // Tags you want to set
  "tags": []
}
```

## Command-Line

The simplest way to get started is with the command-line, leveraging a JSON filtering tool like `jq`.

The following command disassembles `sample.dll` with `16` threads, the relevant traits are JSON objects, one per line and are piped into `jq` for filtering and beautifying.

```bash
binlex -i sample.dll --threads 16 | jq
```

### Configuration

Upon your first execution of **binlex** it will store the configuration file in your configuration directory in `binlex/binlex.toml`.

| OS       | Environment Variable                  | Example Binlex Configuration Path                              |
|----------|---------------------------------------|----------------------------------------------------------------|
| Linux    | `$XDG_CONFIG_HOME` or `$HOME/.config` | `/home/alice/.config/binlex/binlex.toml`                       |
| macOS    | `$HOME/Library/Application Support`   | `/Users/Alice/Library/Application Support/binlex/binlex.toml`  |
| Windows  | `{FOLDERID_RoamingAppData}`           | `C:\Users\Alice\AppData\Roaming\binlex\binlex.toml`            |

The default configuration **binlex** provides is provided below.

```toml
[general]
threads = 16
minimal = false
debug = false

[heuristics.features]
enabled = true

[heuristics.normalization]
enabled = false

[heuristics.entropy]
enabled = true

[hashing.sha256]
enabled = true

[hashing.tlsh]
enabled = true
minimum_byte_size = 50

[hashing.minhash]
enabled = true
number_of_hashes = 64
shingle_size = 4
maximum_byte_size = 50
seed = 0

[mmap]
directory = "/tmp/binlex"

[mmap.cache]
enabled = false

[disassembler.sweep]
enabled = true
```

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

// Get Default Configuration
let config = Config();

// Read PE File
let pe = PE.new("./sample.dll", config);

// Get Memory Mapped Image
let mapped_file = pe.image()
  .unwrap_or_else(|error| { eprintln!("{}", error); process::exit(1)});

let image = mapped_file
  .mmap()
  .unwrap_or_else(|error| { eprintln!("{}", error); process::exit(1); });

// Create Disassembler
let disassembler = Disassembler(pe.architecture(), &image, pe.executable_virtual_address_ranges());

// Create Control Flow Graph
cfg = Graph(pe.architecture(), config);

// Disassemble Control Flow
disassembler.disassemble_controlflow(pe.functions(), cfg);

// Read Block from Control Flow
block = Block(pe.entrypoint(), cfg);

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
from binlex.controlflow import Block

# Get Default Configuration
config = Config()

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

# Read Block from Control Flow
block = Block(pe.entrypoint(), cfg)

# Print Block from Control Flow
block.print()
```

Please note that there are limitations in Python that will affect speed and memory performance.

As such, if you absolutely need lightening fast performance consider using the Rust API instead.
