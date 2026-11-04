# Architettura

Phone Connect collega un telefono Android a un PC Windows sulla rete locale.
Prima funzione: appunti condivisi in entrambe le direzioni, senza interventi
manuali.

Requisito principale: **impatto minimo sulla batteria del telefono**.

## Componenti

```
crates/core     libreria Rust: protocollo, pairing, crittografia, configurazione
crates/cli      binario `phc`: daemon in background + comandi (pair, status, ...)
crates/desktop  binario `phone-connect`: UI gpui, solo visualizzazione
android/        app Android in Kotlin nativo
```

### Daemon (`phc daemon`)

- È il backend: gestisce connessione, pairing, appunti e stato.
- Parte al login dell'utente. Non è un servizio Windows, perché un servizio
  gira nella sessione 0 e non ha accesso agli appunti dell'utente né alle
  notifiche.
- Gestisce l'icona nel tray (crate `tray-icon`; gpui non supporta il tray). Il
  clic sull'icona apre la UI.
- Espone lo stato alla UI via named pipe.
- Ascolta su una porta TCP: serve una regola del firewall di Windows.

### UI desktop

- Processo separato, avviato dal tray. Chiusa la finestra, il processo termina:
  non occupa memoria quando non è visibile.
- Non contiene logica: legge e mostra lo stato del daemon.

### App Android

- Kotlin nativo, senza React Native: niente runtime JavaScript in memoria, e
  la parte in background andrebbe comunque scritta in codice nativo.
- Se la logica condivisa crescerà, `core` si potrà esporre ad Android con
  UniFFI.

## Pairing

1. Al primo avvio il PC genera la propria identità (certificato e chiave
   privata).
2. `phc pair` (o la UI) mostra un QR con IP e porta del PC, l'impronta del suo
   certificato e un codice monouso valido pochi minuti. La CLI lo stampa nel
   terminale con caratteri Unicode.
3. Il telefono scansiona il QR, si collega e verifica l'impronta del PC, poi
   invia il proprio certificato e il codice.
4. Da lì in poi la connessione è TLS e ciascun lato accetta solo il
   certificato già salvato dell'altro. Nessun servizio cloud.

## Appunti

### PC → telefono

- Il daemon riceve una notifica da Windows a ogni copia
  (`AddClipboardFormatListener`), senza polling.
- Solo testo, con un limite di dimensione.
- Il contenuto marcato dai password manager come escluso dal monitoraggio
  (`ExcludeClipboardContentFromMonitorProcessing`) non viene inviato.
- Android consente di scrivere negli appunti anche dal background.

### Telefono → PC

Da Android 10 un'app in background non può leggere gli appunti. Usiamo la
stessa tecnica di KDE Connect:

1. Permesso `READ_LOGS` concesso una volta via ADB
   (`pm grant <pacchetto> android.permission.READ_LOGS`), automatizzabile da
   un comando della CLI.
2. Quando l'utente copia, il sistema scrive nel log che ha negato l'accesso
   agli appunti alla nostra app. L'app legge il log e rileva la copia.
3. L'app apre per un istante un'activity trasparente che, avendo il focus,
   legge gli appunti, li invia al PC e si chiude. Per aprirla dal background
   serve il permesso "Mostra sopra altre app".

### Anti-loop

Quando un lato scrive negli appunti un testo ricevuto dall'altro, la modifica
che ne risulta non viene rimandata indietro (confronto con l'hash dell'ultimo
contenuto ricevuto).

## Batteria e connessione

- **Connessione sempre aperta solo sulla Wi-Fi di casa.** La tiene un
  foreground service, che comporta una notifica fissa silenziabile. Fuori da
  quella rete il servizio si ferma.
- **Il PC invia i keepalive, il telefono risponde.** Il telefono non ha timer
  propri: si sveglia solo quando arrivano dati.
- **Doze:** un'app con foreground service mantiene l'accesso alla rete, quindi
  non serve l'esclusione dall'ottimizzazione batteria.
- **Ricerca del PC (mDNS) solo al cambio di rete,** mai continua. Il telefono
  prova prima l'ultimo IP noto.

## Rischi aperti

Da verificare direttamente sul telefono (Oppo Find X9 Pro, ColorOS), non
sull'emulatore:

- **Conferma per l'accesso ai log:** da Android 13 il sistema può chiederla
  quando un'app legge i log. Se comparisse spesso, l'invio automatico
  telefono → PC non sarebbe più trasparente.
- **ColorOS chiude le app in background:** vanno attivati "Avvio automatico" e
  "Attività in background".
- **Aggiornamenti di Android o ColorOS** possono bloccare la tecnica di
  `READ_LOGS`, che non è un'API ufficiale.
