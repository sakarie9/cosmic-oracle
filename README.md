# 🌌 Cosmic Oracle ✨

A mystical cosmic oracle tool that randomly selects names from stars and constellations to inspire your projects, characters, or creative endeavors.

## 📖 Features

- 🌟 **Stars and Constellations**: Randomly choose from 500+ stellar and constellation names
- 🎨 **Beautiful Output**: Cosmic-themed decorative output format
- 📋 **Raw Mode**: Plain text output for easy script integration
- ⚡ **Fast Startup**: Data embedded at compile time for instant launch

## 🚀 Installation

### Usage

```bash
# Standard mode - with decorative output
cosmic-oracle

# Raw mode - name only
cosmic-oracle --raw
cosmic-oracle -r
```

## 💫 Usage Examples

### Standard Mode Output

```bash
$ cosmic-oracle
✨🌟 Consulting the Cosmic Oracle for a name... 🔮⭐
╭─────────────────────────────────────╮
│  🌌 The Oracle has spoken... 🌌     │
╰─────────────────────────────────────╯
✨ => Sirius <= ✨
```

### Raw Mode Output

```bash
$ cosmic-oracle --raw
Vega
```

## 🛠️ Command Line Options

| Option | Short | Description |
|--------|-------|-------------|
| `--raw` | `-r` | Raw output mode, display name only without decoration |
| `--help` | `-h` | Show help information |

## 📊 Data Sources

- **Star Names**: Contains 452 real stellar names from the International Astronomical Union (IAU) approved star naming catalog
- **Constellation Names**: Includes 90 constellation names covering all modern astronomical official constellations

## 🎯 Use Cases

- 🎮 Game character naming
- 📝 Creative project naming
- 🌐 Domain name inspiration
- 🤖 API/Service naming
- 🎨 Artwork titles
- 📚 Fictional world building

## 🔄 Script Integration

Use raw mode for easy integration into shell scripts:

```bash
#!/bin/bash
NAME=$(cosmic-oracle --raw)
echo "Creating project: $NAME"
mkdir "$NAME"
cd "$NAME"
```

Get a name and copy to wayland clipboard:

```bash
cosmic-oracle -r | wl-copy
```

---

*Let the wisdom of the cosmos guide your creativity* 🌌✨
