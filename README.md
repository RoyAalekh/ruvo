# Ruvo - A Fast, Lightweight Virtual Environment Manager


**Ruvo** is a blazing-fast, lightweight **Rust-based virtual environment manager** for Python. It simplifies creating, managing, and switching between virtual environments with **speed and efficiency**. Designed for developers who value performance and simplicity, Ruvo eliminates bloat and provides a **seamless** experience for managing your Python environments.

## 🚀 Features

✅ **Fast & Lightweight** - Built in Rust for optimal performance.  
✅ **Multiple Environment Support** - Works with `venv`, `poetry` and `uv`.  
✅ **Simple Commands** - Easy to use CLI for quick environment management.  
✅ **Minimal Dependencies** - No unnecessary bloat, just what you need.  
✅ **Python Version Control** - Create environments with specific Python versions.  
✅ **Instant Activation & Deletion** - Manage environments effortlessly.

---

## 📥 Installation

To install Ruvo, simply run:

```bash
pip install git+https://github.com/RoyAalekh/ruvo.git

```

---

## 🔧 Usage

### 🎯 Create a New Environment

```bash
ruvo create my-env --env-type venv
ruvo create poetry-env --env-type poetry --python-version 3.11
```

### 📜 List Available Environments

```bash
ruvo list
```

### 🚀 Activate an Environment

```bash
ruvo activate my-env
```

### 🗑️ Delete an Environment

```bash
ruvo delete my-env
```

---

## 📌 Why Choose Ruvo?

🔹 **Lightning-fast:** Built with Rust, Ruvo outperforms traditional environment managers.  
🔹 **Developer-friendly:** Simple commands, intuitive design, and easy setup.  
🔹 **Cross-platform:** Works seamlessly on Windows, macOS, and Linux.  
🔹 **Versatile:** Supports both `venv`, `poetry` and `uv` environments.

---

## 🌍 Community & Support

If you love Ruvo and want to contribute or get help, join us:
- [GitHub Issues](https://github.com/RoyAalekh/ruvo/issues) - Report bugs and request features.
- [Discussions](https://github.com/RoyAalekh/ruvo/discussions) - Share ideas and get help.

🚀 **Speed up your Python workflow with Ruvo today!**