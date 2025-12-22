# Publishing Guide for HTMS Packages

## TL;DR - Quick Publish

```bash
# Update version in htms-compiler/Cargo.toml, then:
cd htms-compiler && npm run publish:wasm
```

## Package Names (IMPORTANT!)

- ✅ **`@progalaxyelabs/htms-compiler`** - Correct scoped name
- ❌ **`htms-compiler`** - Wrong! Missing scope!

## ⚠️ CRITICAL: wasm-pack Scope Configuration (wasm-pack v0.13.1)

**The ONLY way to set npm package scope with wasm-pack is via the `--scope` CLI flag.**

### Correct Configuration:

**Cargo.toml:**
```toml
[package]
name = "htms-compiler"  # WITHOUT @ or /
version = "0.4.2"
```

**package.json:**
```json
{
  "scripts": {
    "build": "wasm-pack build --target nodejs --scope progalaxyelabs --out-dir pkg"
  }
}
```

### Common Mistakes to AVOID:

- ❌ DO NOT use `name = "@progalaxyelabs/htms-compiler"` in Cargo.toml
  (Cargo doesn't support @ or / in package names)

- ❌ DO NOT use `[package.metadata.wasm-pack] npm-scope = "progalaxyelabs"`
  (This field never existed in wasm-pack - it's a myth!)

- ❌ DO NOT forget `--scope progalaxyelabs` in build commands
  (Without it, you'll publish as unscoped `htms-compiler`)

This is the documented, official approach for wasm-pack v0.13.1.

## Publishing htms-compiler

```bash
cd htms-compiler

# 1. Update version in Cargo.toml
# version = "0.4.2"

# 2. Run tests
cargo test

# 3. Build & Publish (ONE COMMAND!)
npm run publish:wasm
```

## Publishing htms-cli

```bash
cd htms-cli

# 1. Update version & dependency in package.json
# "@progalaxyelabs/htms-compiler": "^0.4.2"

# 2. Publish
npm publish --access public
```

## ⚠️ NEVER Run These Commands Manually:

- ❌ `wasm-pack build` (missing --scope flag!)
- ❌ `cd pkg && npm publish` (use npm script instead!)

## ✅ Always Use:

- ✅ `npm run build` (has --scope flag)
- ✅ `npm run publish:wasm` (does everything)
