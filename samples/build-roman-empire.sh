#!/usr/bin/env bash
set -euo pipefail

HERE=$(cd "$(dirname "$0")" && pwd)
TIL="$HERE/../target/debug/til"
FILE="$HERE/roman-empire.til"

rm -f "$FILE"
"$TIL" "$FILE" >/dev/null

add_event() {
  local label=$1
  local date=$2
  shift 2
  "$TIL" "$FILE" event add -- "$label" "$date"
  for tag in "$@"; do
    "$TIL" "$FILE" event tag "$label" "$tag"
  done
}

add_range() {
  local label=$1
  local start=$2
  local end=$3
  shift 3
  "$TIL" "$FILE" range add "$label" --start="$start" --end="$end"
  for tag in "$@"; do
    "$TIL" "$FILE" range tag "$label" "$tag"
  done
}

# Tags.
for t in war politics religion culture dynasty plague milestone emperor \
         infrastructure persecution christianity assassination; do
  "$TIL" "$FILE" tag add "$t"
done

# Republic collapse (49 BC – 27 BC).
add_event "Caesar crosses the Rubicon"                   -000049-01-10 war politics milestone
add_event "Battle of Pharsalus"                          -000048-08-09 war
add_event "Caesar appointed dictator perpetuo"           -000044-02-14 politics
add_event "Caesar assassinated on the Ides of March"     -000044-03-15 assassination politics milestone
add_event "Second Triumvirate formed"                    -000043-11-27 politics
add_event "Battle of Philippi"                           -000042-10-23 war
add_event "Battle of Actium"                             -000031-09-02 war milestone
add_event "Death of Antony and Cleopatra"                -000030-08-12 politics milestone
add_event "Augustus accepts princeps"                    -000027-01-16 emperor politics milestone

# Julio-Claudian (27 BC – 68 AD).
add_event "Virgil's Aeneid published"                    -000019-09-21 culture
add_event "Death of Augustus"                            0014-08-19 emperor
add_event "Tiberius becomes emperor"                     0014-09-17 emperor dynasty
add_event "Caligula becomes emperor"                     0037-03-16 emperor dynasty
add_event "Caligula assassinated"                        0041-01-24 assassination emperor
add_event "Claudius becomes emperor"                     0041-01-25 emperor dynasty
add_event "Claudius invades Britain"                     0043-05-01 war infrastructure
add_event "Nero becomes emperor"                         0054-10-13 emperor dynasty
add_event "Great Fire of Rome"                           0064-07-19 milestone
add_event "Nero suicide; Year of the Four Emperors"      0068-06-09 emperor politics

# Flavian (69 – 96).
add_event "Vespasian becomes emperor"                    0069-12-21 emperor dynasty
add_event "Destruction of Jerusalem"                     0070-09-08 war religion
add_event "Vesuvius erupts; Pompeii lost"                0079-08-24 milestone
add_event "Colosseum inaugurated"                        0080-04-01 infrastructure culture
add_event "Domitian assassinated"                        0096-09-18 assassination emperor

# Nerva–Antonine (96 – 180).
add_event "Trajan becomes emperor"                       0098-01-28 emperor dynasty
add_event "Empire reaches greatest extent"               0117-08-09 milestone war
add_event "Hadrian becomes emperor"                      0117-08-11 emperor dynasty
add_event "Hadrian's Wall construction begins"           0122-06-01 infrastructure
add_event "Marcus Aurelius becomes emperor"              0161-03-07 emperor dynasty
add_event "Antonine Plague begins"                       0165-04-01 plague
add_event "Marcus Aurelius dies on the Danube"           0180-03-17 emperor

# Crisis & recovery (192 – 284).
add_event "Commodus assassinated"                        0192-12-31 assassination emperor
add_event "Septimius Severus emperor"                    0193-04-09 emperor dynasty
add_event "Caracalla grants universal citizenship"       0212-07-11 politics milestone
add_event "Crisis of the Third Century begins"           0235-03-18 politics milestone
add_event "Decian persecution of Christians"             0250-01-03 persecution religion
add_event "Aurelian reunites the empire"                 0274-09-01 emperor war

# Late Empire (284 – 395).
add_event "Diocletian becomes emperor"                   0284-11-20 emperor dynasty
add_event "Tetrarchy established"                        0293-03-01 politics milestone
add_event "Great Persecution begins"                     0303-02-23 persecution religion
add_event "Constantine becomes emperor"                  0306-07-25 emperor dynasty
add_event "Battle of the Milvian Bridge"                 0312-10-28 war milestone christianity
add_event "Edict of Milan"                               0313-02-01 religion christianity milestone
add_event "Council of Nicaea"                            0325-06-19 religion christianity
add_event "Constantinople inaugurated"                   0330-05-11 infrastructure milestone
add_event "Constantine dies"                             0337-05-22 emperor
add_event "Julian the Apostate emperor"                  0361-12-11 emperor religion
add_event "Battle of Adrianople"                         0378-08-09 war milestone
add_event "Christianity made state religion"             0380-02-27 religion christianity milestone
add_event "Theodosius dies; empire permanently split"    0395-01-17 milestone politics

# Ranges: cultural eras spanning years.
add_range "Roman Republic collapse"      -000049 -000027 war politics
add_range "Julio-Claudian dynasty"       -000027 0068    dynasty emperor
add_range "Pax Romana"                   -000027 0180    politics milestone
add_range "Flavian dynasty"               0069   0096    dynasty emperor
add_range "Nerva-Antonine dynasty"        0096   0192    dynasty emperor
add_range "Antonine Plague"               0165   0180    plague
add_range "Severan dynasty"               0193   0235    dynasty emperor
add_range "Crisis of the Third Century"   0235   0284    politics
add_range "Diocletian Tetrarchy"          0284   0324    politics
add_range "Constantinian dynasty"         0306   0363    dynasty emperor
add_range "Christianization of Rome"      0313   0391    religion christianity

echo
echo "Built $FILE"
"$TIL" "$FILE" inspect
