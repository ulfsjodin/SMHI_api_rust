# SMHI Weather Fetcher 🌦️ (Rust-projekt)

Det här projektet hämtar meteorologiska observationsdata från SMHI:s öppna API. Syftet är att samla in data från olika stationer (t.ex. Göteborg, Jönköping, Valdemarsvik) och analysera väderparametrar som temperatur och molnmängd.

Projektet är skrivet i Rust och fokuserar på:
- Att hämta JSON-data från SMHI:s API
- Felhantering per station och parameter
- Strukturering av data i egna Rust-strukturer
- God kodstruktur med moduler

---

## 🛠️ Status

✅ Hämtar data asynkront från SMHI:s API  
✅ Fungerande kod i en separat utvecklingsgren (`parsermodul`)  
✅ Hantering av flera stationer och väderparametrar  
🚧 Kommande steg: Spara data i SQLite och möjliggöra statistik

---

## 📦 Kom igång

### Förutsättningar
- Rust installerat (https://rustup.rs)
- Internetanslutning

### Kör projektet

```bash
git clone https://github.com/ditt-användarnamn/smhi-weather-fetcher.git
cd smhi-weather-fetcher
cargo run
```

Koden körs asynkront och hämtar observationer från SMHI:s API. Data skrivs i dagsläget ut i terminalen.

---

## 🧱 Kodstruktur

```text
.
├── src/
│   ├── main.rs          # Startpunkt för programmet
│   ├── api.rs           # Logik för att bygga URL:er och hämta data
│   ├── parser.rs        # Kod för att tolka och strukturera JSON-svar
│   └── utils.rs         # Diverse hjälpfunktioner (om tillämpligt)
└── Cargo.toml
```

---

## 📌 Exempel på utdata

```
_______________________
Data från station/er: 72420,  Temperatur
-----------------------
Observationen >: WeatherObservation {
    date: "2025-04-12T10:00:00Z",
    value: 8.7,
    ...
}
```

---

## 📚 Planerade funktioner

- [ ] Lagra väderdata i SQLite
- [ ] Schemalägga körning (exempelvis via `cron` på Raspberry Pi)
- [ ] Analys och jämförelse över tid
- [ ] Stöd för fler väderparametrar (vind, nederbörd, lufttryck)

---

## 🧪 Testning

Projektet använder i nuläget `println!` för att visualisera data och fel. Felhantering sker individuellt för varje API-anrop:

```rust
match fetch_observation(&url).await {
    Ok(obs) => { ... }
    Err(e) => { ... }
}
```

---

## 📖 Länkar

- [SMHI:s öppna API](https://opendata.smhi.se/apidocs/metobs/index.html)
- [Rust documentation](https://doc.rust-lang.org/book/)

---

## 📜 Licens

MIT License.
