# DatZM024 : Ātru algoritmu konstruēšana un analīze - 1. programmēšanas darbs
## Kompilēšana un darbināšana
Programmas darbība pārbaudīta ar Rust versiju 1.86.0

No projekta saknes programma kompilējama relīzes versijai ar:
```sh
$ cargo build -r
```

Kompilēto relīzes programmu iespējams darbināt šādi:
```sh
$ ./target/release/winnie_pooh
Expected 2 CLI arguments, got 0
Usage:
    winnie_pooh <input_file> <output_file>
Error: Custom { kind: InvalidInput, error: "Invalid CLI arguments" }


$ ./target/release/winnie_pooh input_file.txt output_file.txt
```

Iespējams debug versijā kompilēt un darbināt uzreiz ar cargo utilītu:
```sh
$ cargo run input_file.txt output_file.txt
```
## Algoritma pareizība & laika sarežģītības novērtējums
Visi skaidrojumi, galvenokārt, kopēti no komentāriem iekš `./src/main.rs` funkcijas `winnie_pooh`

Galvenā problēmas prasība: katrā derīgā cikliskā maršrutā jābūt vismaz vienai šķautnei ar medus podu.
Derīgs ciklisks maršruts ir tāds, kurā katra šķautne ir dažāda, tās neatkārtojas.

**Risinājums:**
- Tiek atrasts max weight spanning tree (MST) ar Kruskala algoritmu,
- Šķautnes, kuras nav iekļautas iekš MST attiecīgi būs tās, kuras iekļausies katrā ciklā - tās būs meklējamās šķautnes ar medus podiem,
- Tā kā kokā iekļautas lielākā svara šķautnes, tad atlikušajām šķautnēm būs pēc iespējas mazāks svars,
- Šīs atlikušās šķautnes nodrošina, ka katrā ciklā ir vismaz viena šķautne ar medus podu,
- Lai sasniegtu iespējami mazāko svaru summu, papildus tiek iekļautas šķautnes ar negatīvu svaru no MST.

### Laika sarežģītība
Pēc MST definīcijas, rezultātā iegūtais šķautņu skaits MST kokā ir |V| - 1,
tāpēc tālāk vietās, kur tiek apskatītās MST šķautnes, no sākotnējā grafa tās būs skaitā O(|V|)

- **Parsēšana:** O(|E|) - ievadfaila struktūra, galvenokārt, apraksta šķautnes, parseris izveido grafa objektu vienu reizi, caurstaigājot ievadfailu,
- **Šķautņu sakārtošana un MST izveide:** O(|E| * log |V|) - pēc Kruskala algoritma,
- **Starpības atrašanas no sākotnējā grafa un MST:** O(|V| + |E|) = O(|E|)
  - funkcijā izveidota `HashSet` MST grafam - O(|V|)
  - šķautņu pārbaude sākotnējam grafam - O(|E|)
  - kopā sanāk O(|V| + |E|), bet tā kā grafs ir connected, tad:
  - |E| >= |V| - 1
  - O(|V| + |E|) = O(|E|),
- **Negatīvo svaru šķautņu pievienošana no MST:** O(|V|),
- **Serializēšana:** O(|E|).

**Kopējā laika sarežģītība:** O(|E| * log |V|)