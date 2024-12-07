<table>
  <tr>
    <td style="border: none;"><img src="./assets/binlex.png" alt="Binlex logo" width="100"></td>
    <td style="border: none; vertical-align: middle; padding-left: 10px;">
      <h1 style="font-weight: bold; margin: 0;">Binlex - A Binary Genetic Trait Lexer Framework</h1>
    </td>
  </tr>
</table>

The purpose of **binlex** is to extract basic blocks and functions as **genomes** from binaries for **malware research**, **hunting**, and **detection**. 🦠🔍

Most projects attempting this use pure Python to generate traits, but it’s often **slow** 🐢.

The design philosophy behind **binlex** is to keep it **simple** and **extendable**, with an ecosystem of helpful tools and library code. ⚙️

The simple **command-line interface** allows malware researchers and analysts to hunt for traits across **hundreds** or **thousands** of potentially similar malware samples, saving **time** ⏳ and **money** 💰 in production environments.

The **Rust API** and **Python bindings** let developers create their own detection solutions with minimal **license limitations**. 🔓

To help combat malware, we provide our work for the greater good. 🌍

No installation needed, just **download the binaries** from the **release page**! 📥

## 🚀 Features

The latest version of **binlex** provides the following amazing features!

| Feature                         | Description                                                                                     |
|---------------------------------|-------------------------------------------------------------------------------------------------|
| 🌐 **Platforms**   | - Windows 🪟<br>- MacOS 🍏<br>- Linux 🐧                                                    |
| 🌐 **Formats**   | - PE <br>- MachO <br>- ELF                                                  |
| 🌐 **Architectures**   | - AMD64 <br>- I386<br> - CIL                                               |
| 🧵 **Multi-Threading**          | - 🔒 Thread-Safe Disassembler Queuing<br>- 🚄 Multi-Threaded Tooling for Maximum Efficiency      |
| ⚙️ **Customizable Performance** | Toggle features on/off to optimize for your use case                                           |
| 📉 **JSON String Compression**  | Save memory with JSON compression                                                    |
| 🧩 **Similarity Hashing**       | - 🔍 Minhash<br>- 🔒 TLSH<br>- 🔐 SHA256                                                        |
| 🧩 **Function Symbols**         | - Pass function symbols to **binlex** as standard input using **blpdb**, **blelfsym** or **blmachosym** or your own tooling                        |
| 🏷️ **Tagging**                  | Tagging for easy organization                                                                  |
| 🎯 **Wildcarding** | Perfect for generating YARA rules and now at a resolution of nibbles!                                                     |
| **API** | - 🦀 Rust API<br>-Python API                                                         |
| 🤖 **Machine Learning Features** | - 📊 Normalized Features for Consistency<br>- 📏 Feature Scaler Utility<br>- 🔍 Trait Filtering<br>- 📚 Onnx Sample Training<br>- 🧠 Sample Classification |
| 📂 **Virtual Imaging** | - Efficient mapping cache for virtual images<br>- 🗄️ Compatible with ZFS / BTRFS<br>- Speeds up repetitive tasks and filtering<br>- Lightening speed ⚡ |

By caching virtual images, **binlex** is able to perform at increased speeds, making repeat runs faster and more efficient.

## Building

To build **binlex** you will need Rust.

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

### Packaging

To build packages for various platforms use the `Makefile`.

```bash
make zst   # Make Arch Linux Package
make deb   # Make Debian Package
make wheel # Make Python Wheel
```

The resulting packages will be in the `target/` directory.

### Documentation

```bash
cargo doc
```

You can also open the docs.

```bash
cargo doc --open
```

## Binary Genomes, Chromosomes, Allele Pairs and Genes

In **binlex**, a hierarchy of genetic-inspired terms is used to describe and symbolize the structure and traits of binary code. This terminology reflects the relationships between different abstractions and their genetic analogies:

- **Genome**: Represents the each object being analyzed, such as a function or block. It encapsulates all the information, including metadata, chromosomes, and other attributes.

- **Chromosome**: Represents the core patterns or sequences extracted from a block or function. A chromosome acts as the blueprint for identifying key characteristics of the binary without memory addressing as indicated by wildcards like `?`, where a single wildcard represents a single gene.

- **AllelePair**: A unit within the chromosome consisting of **two genes**. Allele pairs are the building blocks of the chromosome, combining genes into meaningful pairs.

- **Gene**: The smallest unit of genetic information, representing a single nibble of data (half a byte).

The relationship between these abstractions can be visualized as follows:

```text
Genome (function / block)
 └── Chromosome (pattern / sequence)
      └── AllelePair (two genes / single byte / two nibbles)
           └── Gene (single nibble)
```

### Genome Example

```JSON
{
  "type": "block",
  "architecture": "amd64",
  "address": 6442934577,
  "next": null,
  "to": [],
  "edges": 0,
  "prologue": false,
  "conditional": false,
  "chromosome": {
    "pattern": "4c8b47??498bc0",
    "feature": [4,12,8,11,4,7,4,9,8,11,12,0],
    "entropy": 2.2516291673878226,
    "sha256": "1f227bf409b0d9fbc576e747de70139a48e42edec60a18fe1e6efdacb598f551",
    "minhash": "09b8b1ad1142924519f601854444c6c904a3063942cda4da445721dd0703f290208f3e32451bf5d52741e381a13f12f9142b5de21828a00b2cf90cf77948aac4138443c60bf77ec31199247042694ebb2e4e14a41369eddc7d9f84351be34bcf61458425383a03a55f80cbad420bb6e638550c15876fd0c6208da7b50816847e62d72b2c13a896f4849aa6a36188be1d4a5333865eab570e3939fab1359cbd16758f36fa290164d0259f83c07333df535b2e38f148298db255ac05612cae04d60bb0dd810a91b80a7df9615381e9dc242969dd052691d044287ac2992f9092fa0a75d970100d48362f62b58f7f1d9ec594babdf52f58180c30f4cfca142e76bf",
    "tlsh": null
  },
  "size": 7,
  "bytes": "4c8b4708498bc0",
  "functions": {},
  "number_of_instructions": 3,
  "entropy": 2.5216406363433186,
  "sha256": "84d4485bfd833565fdf41be46c1a499c859f0a5f04c8c99ea9c34404729fd999",
  "minhash": "20c995de6a15c8a524fa7e325a6e42b217b636ab03b00812732f877f4739eeee41d7dde92ceac73525e541f9091d8dc928f6425b84a6f44b3f01d17912ec6e8c6f913a760229f685088d2528447e40c768c06d680afe63cb219a1b77a097f679122804dd5a1b9d990aa2579e75f8ef201eeb20d5650da5660efa3a281983a37f28004f9f2a57af8f81728c7d1b02949609c7ad5a30125ff836d8cc3106f2531f306e679a11cabf992556802a3cb2a75a7fe3773e37e3d5ab107a23bf22754aee15a5f41056859b06120f86cb5d39071425855ec90628687741aa0402030d73e04bc60adb0bd2430560442c4309ae258517fc1605438c95485ac4c8621026a1bb",
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
      "entropy": 6.55061550644311,
      "sha256": "ec1426109420445df8e9799ac21a4c13364dc12229fb16197e428803bece1140",
      "size": 725696,
      "tlsh": "T17AF48C12AF990595E9BBC23DD1974637FAB2B445232047CF426489BD0E1BBE4B73E381",
      "type": "file"
    }
  ]
}
```

Given this JSON genome example.
- **Genome**: The JSON object describing the block, including its metadata, chromosome, and attributes.
- **Chromosome**: as described by the pattern `"4c8b47??498bc0"`
- **AllelePair**: `"4c"` or `"8b"`
- **Gene**: `"4"` or `"c"`

Using the **binlex** API it is possible to mutate these chromosomes, their allelepairs and genes to facilitate genetic programming.

Genetic programming in this context can have several benifits including but not limited to:
- Hunting for novel samples given a dataset
- YARA rule generation

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

Please note that **binlex** will detect the file format fort you and currently supports `PE`, `ELF` and `MACHO` binary formats.

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
threads = 1
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

[formats.file.heuristics.normalized]
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

[blocks.heuristics.normalized]
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

[functions.heuristics.normalized]
enabled = false

[functions.heuristics.entropy]
enabled = true

[chromosomes.hashing.sha256]
enabled = true

[chromosomes.hashing.tlsh]
enabled = true
minimum_byte_size = 50

[chromosomes.hashing.minhash]
enabled = true
number_of_hashes = 64
shingle_size = 4
maximum_byte_size = 50
seed = 0

[chromosomes.heuristics.features]
enabled = true

[chromosomes.heuristics.normalized]
enabled = false

[chromosomes.heuristics.entropy]
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

### Comparing Traits Based on Similarity

In **binlex** comparisons can be done using the tool `blcompare`, at this time it only supports TLSH similarity hashing.

All comparisons are one to many, as for every left hand side trait is compared to every right hand side trait.

These are symbolized as `lhs` and `rhs` respectively.

The command-line help for `blcompare` is provided below.

```text
A Binlex Trait Comparison Tool

Version: 1.0.0

Usage: blcompare [OPTIONS] --input-rhs <INPUT_RHS>

Options:
      --input-lhs <INPUT_LHS>  Input file or wildcard pattern for LHS (Left-Hand Side)
      --input-rhs <INPUT_RHS>  Input file or wildcard pattern for RHS (Right-Hand Side)
  -t, --threads <THREADS>      Number of threads to use [default: 1]
  -r, --recursive              Enable recursive wildcard expansion
  -h, --help                   Print help
  -V, --version                Print version

Author: @c3rb3ru5d3d53c
```

Using this tool you can compare full directories using globbing file searches of lhs traits vs rhs traits.

This also accepts standard input so you can process something from **binlex** as lhs and then specify a rhs file path glob recursive or not for compairson.

The output can then be filtered using `jq`.

This tool supports multiple threads and processes one file at a time, but the one to many comparison is multi-threaded.

NOTE: Comparisons should be use sparingly to decide what to keep. By default `blcompare` keeps everything which can be many GB of data. As such it is recommended to perform filtering to only keep similairty within a threshold using `jq`.

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

**NOTE**: At this time `blrizin` is also compatiable with the output from `radare2` using `blrizin`.

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

The philophsy of the **binlex** project is focused on security, simplicity, speed and extendability.

Part of this is providing an API for developers to write their own detection and hunting logic.

At this time, **binlex** provides both Rust and Python bindings.

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
from binlex.formats import ELF
from binlex.formats import MACHO
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
f = PE('./sample.dll', config)

# Or Open ELF File
# f = ELF('./sample.so', config)

# Or Open MachO File
# f = MACHO('./sample.macho', config)
# NOTE: MachO requires binary slice argument for most methods

# Get the Memory Mapped File
mapped_file = f.image()

# Get the Memory Map
image = mapped_file.as_memoryview()

# Create Disassembler on Mapped PE Image and PE Architecture
disassembler = Disassembler(f.architecture(), image, f.executable_virtual_address_ranges())

# Create the Controlflow Graph
cfg = Graph(f.architecture(), config)

# Disassemble the PE Image Entrypoints Recursively
disassembler.disassemble_controlflow(f.entrypoints(), cfg)

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

## Citation

If you are using **binlex** in a journal publication, or an open-source AI model use the following citation.

```bibtex
@misc{binlex,
  author = {c3rb3ru5d3d53c},
  title = {binlex: A Binary Genetic Trait Lexer Framework},
  year = {2024},
  note = {Available at \url{https://github.com/c3rb3ru5d3d53c/binlex-rs}}
}
```

If the use of **binlex** is for corporate, personal purposes, or for generating outputs that are not open-source AI models, no citation is needed.

For example, if you use **binlex** to create YARA rules, no citation is needed.

This ensures that **binlex** stays relevant but also ensures permissive corporate and personal use.
