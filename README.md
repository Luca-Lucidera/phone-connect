# Phone Connect

Collega un telefono Android a un PC Windows sulla rete locale. Prima funzione:
appunti condivisi in entrambe le direzioni.

Architettura e decisioni: [docs/architecture.md](docs/architecture.md).

## Struttura

| Percorso | Contenuto |
|---|---|
| `crates/core` | Protocollo, pairing, crittografia (libreria Rust) |
| `crates/cli` | `phc`: daemon in background e comandi |
| `crates/desktop` | `phone-connect`: UI desktop con gpui |
| `android/` | App Android in Kotlin |

## Build

PC (Rust stabile, toolchain MSVC):

```sh
cargo build --workspace
cargo run -p phone-connect-cli -- --help
cargo run -p phone-connect-desktop
```

Android (senza Android Studio): JDK e variabili d'ambiente vengono da
[`mise.toml`](mise.toml). Lo SDK sta in `%LOCALAPPDATA%\Android\Sdk`, separato
da quelli di Unity e Meta Quest Developer Hub.

```sh
cd android
mise exec -- ./gradlew.bat assembleDebug
```

Il server adb del progetto usa la porta 5038 (`ANDROID_ADB_SERVER_PORT`): altre
app sul PC usano versioni diverse di adb sulla porta predefinita 5037, e un
client di versione diversa chiuderebbe il loro server.
