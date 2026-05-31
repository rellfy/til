#!/usr/bin/env bash
set -euo pipefail

HERE=$(cd "$(dirname "$0")" && pwd)
TIL="$HERE/../target/debug/til"
FILE="$HERE/world-music.til"

rm -f "$FILE"
"$TIL" "$FILE" >/dev/null

add_event() {
  local label=$1
  local date=$2
  shift 2
  "$TIL" "$FILE" event add "$label" "$date"
  for tag in "$@"; do
    "$TIL" "$FILE" event tag "$label" "$tag"
  done
}

add_range() {
  local label=$1
  local start=$2
  local end=$3
  shift 3
  "$TIL" "$FILE" range add "$label" --start "$start" --end "$end"
  for tag in "$@"; do
    "$TIL" "$FILE" range tag "$label" "$tag"
  done
}

# Tags.
for t in rock pop jazz soul blues folk electronic disco punk \
         alternative psychedelic progressive metal hiphop \
         album festival death milestone band; do
  "$TIL" "$FILE" tag add "$t"
done

# 1950s: birth of rock and roll.
"$TIL" "$FILE" event add --ref "https://en.wikipedia.org/wiki/Sun_Records" -- "Sun Records founded" 1950-03-27
"$TIL" "$FILE" event tag "Sun Records founded" milestone
add_event "Rocket 88 by Jackie Brenston"                          1951-04-03 rock blues
add_event "Alan Freed coins 'rock and roll' on radio"             1952-07-11 milestone
add_event "Elvis Presley records at Sun Studio"                   1953-08-18 milestone rock
add_event "Space Guitar by Johnny 'Guitar' Watson"                1954-02-25 blues rock
add_event "That's All Right by Elvis Presley"                     1954-07-19 rock
add_event "Rock Around the Clock hits #1"                         1955-07-09 rock milestone
add_event "Blue Suede Shoes by Elvis Presley"                     1956-01-01 rock
add_event "Heartbreak Hotel by Elvis Presley"                     1956-01-27 rock
add_event "The Quarrymen formed (proto-Beatles)"                  1957-03-29 band rock
add_event "Hallelujah I Love Her So by Ray Charles"               1957-04-15 soul
add_event "Johnny B. Goode by Chuck Berry"                        1958-04-01 rock
add_event "Day the Music Died (Holly, Valens, Big Bopper)"        1959-02-03 death rock
add_event "Tenderly by Luiz Bonfa"                                1959-09-01 jazz

# 1960s: Beatles, soul, counterculture.
add_event "Motown Records founded"                                1960-01-12 milestone soul
add_event "Doin' the Best I Can by Elvis Presley"                 1960-03-21 rock
add_event "The Beatles debut at the Cavern Club"                  1961-02-09 milestone band rock
add_event "Can't Help Falling in Love by Elvis Presley"           1961-10-01 pop rock
add_event "Love Me Do by The Beatles"                             1962-10-05 rock pop
add_event "Rumble by Link Wray"                                   1962-04-01 rock
add_event "Please Please Me album by The Beatles"                 1963-03-22 album rock
add_event "Do You Want to Know a Secret by The Beatles"           1963-11-22 rock pop
add_event "Beatles play Ed Sullivan Show"                         1964-02-09 milestone rock
add_event "And I Love Her by The Beatles"                         1964-07-10 rock pop
add_event "Bob Dylan goes electric at Newport"                    1965-07-25 milestone folk rock
add_event "In My Life by The Beatles"                             1965-12-03 rock pop
add_event "Pet Sounds by The Beach Boys"                          1966-05-16 album pop
add_event "Revolver by The Beatles"                               1966-08-05 album rock psychedelic
add_event "Here, There and Everywhere by The Beatles"             1966-08-05 rock pop
add_event "Sgt. Pepper's Lonely Hearts Club Band"                 1967-06-01 album rock psychedelic
add_event "Monterey Pop Festival"                                 1967-06-16 festival
add_event "The White Album"                                       1968-11-22 album rock
add_event "Rocky Raccoon by The Beatles"                          1968-11-22 rock folk
add_event "Beatles' rooftop concert"                              1969-01-30 milestone rock
add_event "Aquarius/Let the Sunshine In by The 5th Dimension"     1969-03-13 pop soul
add_event "Woodstock festival begins"                             1969-08-15 festival rock
add_event "Abbey Road released"                                   1969-09-26 album rock
add_event "Something by The Beatles"                              1969-10-06 rock pop

# 1970s: classic rock, prog, disco, punk.
add_event "Beatles break up"                                      1970-04-10 milestone rock
add_event "Layla by Derek and the Dominos"                        1970-11-09 rock blues
add_event "Jimi Hendrix dies"                                     1970-09-18 death rock
add_event "Janis Joplin dies"                                     1970-10-04 death rock
add_event "Imagine by John Lennon"                                1971-09-09 rock pop
add_event "Led Zeppelin IV"                                       1971-11-08 album rock
add_event "Wild Horses by The Rolling Stones"                     1971-04-23 rock
add_event "Eagles debut album"                                    1972-06-01 album rock
add_event "Take It Easy by Eagles"                                1972-05-01 rock
add_event "The Dark Side of the Moon by Pink Floyd"               1973-03-01 album rock progressive
add_event "Any Colour You Like by Pink Floyd"                     1973-03-01 rock progressive
add_event "Waterloo by ABBA wins Eurovision"                      1974-04-06 pop milestone
add_event "Wish You Were Here by Pink Floyd"                      1975-09-12 album rock progressive
add_event "Shine On You Crazy Diamond by Pink Floyd"              1975-09-12 rock progressive
add_event "Cortez the Killer by Neil Young & Crazy Horse"         1975-11-10 rock folk
add_event "Ramones' debut album"                                  1976-04-23 album punk
add_event "Hotel California by Eagles"                            1976-12-08 album rock
add_event "Europa by Santana"                                     1976-10-26 rock jazz
add_event "I Feel Love by Donna Summer"                           1977-07-02 disco electronic
add_event "Sex Pistols' Never Mind the Bollocks"                  1977-10-28 album punk
add_event "Saturday Night Fever soundtrack"                       1977-11-15 album disco
add_event "Elvis Presley dies"                                    1977-08-16 death rock
add_event "Dire Straits' debut album"                             1978-10-07 album rock
add_event "Down to the Waterline by Dire Straits"                 1978-10-07 rock
add_event "Asa Branca by Elis Regina and Hermeto Pascoal"         1979-04-01 jazz folk
add_event "London Calling by The Clash"                           1979-12-14 album punk rock
add_event "Sony Walkman launched"                                 1979-07-01 milestone

# 1980s: MTV, synth, alt rock, metal.
add_event "Back in Black by AC/DC"                                1980-07-25 album rock metal
add_event "The Boys Light Up by Australian Crawl"                 1980-08-01 rock
add_event "John Lennon assassinated"                              1980-12-08 death rock
add_event "MTV launches"                                          1981-08-01 milestone
add_event "Private Eyes by Hall & Oates"                          1981-09-01 pop
add_event "Solid Rock by Goanna"                                  1982-08-16 rock folk
add_event "Thriller by Michael Jackson"                           1982-11-30 album pop
add_event "Bird of Paradise by Snowy White"                       1983-02-01 rock blues
add_event "War by U2"                                             1983-02-28 album rock
add_event "Don't Answer Me by The Alan Parsons Project"           1984-03-01 rock progressive
add_event "Like a Virgin by Madonna"                              1984-11-12 album pop
add_event "Money for Nothing by Dire Straits"                     1985-06-24 rock
add_event "Live Aid concerts"                                     1985-07-13 festival milestone
add_event "Walk This Way by Run-DMC and Aerosmith"                1986-07-04 hiphop rock
add_event "Don't Dream It's Over by Crowded House"                1986-09-08 pop rock
add_event "The Joshua Tree by U2"                                 1987-03-09 album rock
add_event "Big Love by Fleetwood Mac"                             1987-03-30 rock pop
add_event "Surfer Rosa by Pixies"                                 1988-03-21 album alternative
add_event "Reptile by The Church"                                 1988-02-15 alternative rock
add_event "Disintegration by The Cure"                            1989-05-02 album alternative
add_event "Lovesong by The Cure"                                  1989-08-21 alternative pop

# 1990s: grunge, alt rock, britpop, electronica.
add_event "Still Got the Blues by Gary Moore"                     1990-03-26 blues rock
add_event "Violator by Depeche Mode"                              1990-03-19 album electronic
add_event "Nevermind by Nirvana"                                  1991-09-24 album alternative rock
add_event "Smells Like Teen Spirit by Nirvana"                    1991-09-10 alternative rock
add_event "Achtung Baby by U2"                                    1991-11-19 album rock
add_event "Friday I'm In Love by The Cure"                        1992-04-15 alternative pop
add_event "Automatic for the People by R.E.M."                    1992-10-05 album alternative
add_event "Siamese Dream by The Smashing Pumpkins"                1993-07-27 album alternative
add_event "Creep by Radiohead"                                    1993-09-21 alternative rock
add_event "The Division Bell by Pink Floyd"                       1994-03-28 album rock progressive
add_event "Keep Talking by Pink Floyd"                            1994-03-28 rock progressive
add_event "Kurt Cobain dies"                                      1994-04-05 death alternative
add_event "Walkabout by Red Hot Chili Peppers"                    1995-09-02 alternative rock
add_event "(What's the Story) Morning Glory? by Oasis"            1995-10-02 album rock
add_event "Travelling Without Moving by Jamiroquai"               1996-09-09 album electronic pop
add_event "Virtual Insanity by Jamiroquai"                        1996-08-19 electronic pop
add_event "OK Computer by Radiohead"                              1997-06-16 album alternative
add_event "Anybody Seen My Baby? by The Rolling Stones"           1997-09-15 rock
add_event "The Miseducation of Lauryn Hill"                       1998-08-25 album hiphop soul
add_event "Save Tonight by Eagle-Eye Cherry"                      1998-04-30 pop
add_event "Napster launches"                                      1999-06-01 milestone
add_event "Californication by Red Hot Chili Peppers"              1999-06-08 album alternative rock

# 2000s: digital, indie, electronic resurgence.
add_event "Kid A by Radiohead"                                    2000-10-02 album alternative
add_event "How to Disappear Completely by Radiohead"              2000-10-02 alternative
add_event "Discovery by Daft Punk"                                2001-03-12 album electronic
add_event "Something About Us by Daft Punk"                       2001-03-12 electronic
add_event "iPod launches"                                         2001-10-23 milestone
add_event "Sideways by Santana feat. Citizen Cope"                2002-10-22 rock
add_event "Absolution by Muse"                                    2003-09-15 album alternative rock
add_event "Hysteria by Muse"                                      2003-12-01 alternative rock
add_event "Hot Fuss by The Killers"                               2004-06-07 album alternative rock
add_event "Somebody Told Me by The Killers"                       2004-03-29 alternative rock
add_event "Demon Days by Gorillaz"                                2005-05-11 album alternative electronic
add_event "Feel Good Inc. by Gorillaz"                            2005-05-09 alternative electronic
add_event "Stadium Arcadium by Red Hot Chili Peppers"             2006-05-09 album alternative rock
add_event "Wet Sand by Red Hot Chili Peppers"                     2006-05-09 alternative rock
add_event "In Rainbows pay-what-you-want release"                 2007-10-10 album alternative milestone
add_event "Oh Ana by Mother Mother"                               2007-08-01 alternative
add_event "Day and Age by The Killers"                            2008-11-24 album alternative rock
add_event "Human by The Killers"                                  2008-09-22 alternative rock
add_event "Heartbreak Warfare by John Mayer"                      2009-11-17 pop rock
add_event "Michael Jackson dies"                                  2009-06-25 death pop
add_event "Spotify expansion era"                                 2010-07-01 milestone

# Ranges: cultural eras spanning years.
add_range "Rock and Roll Era"        1954      1962 rock milestone
add_range "Beatles active years"     1960-08   1970-04 rock band
add_range "British Invasion"         1964-02   1967-08 rock
add_range "Psychedelic Era"          1966      1971 rock psychedelic
add_range "Classic Rock heyday"      1969      1979 rock
add_range "Disco Era"                1974      1980 disco
add_range "Punk Movement"            1976      1980 punk
add_range "MTV-driven pop era"       1981-08   1992 pop
add_range "Grunge / Alt-Rock"        1991      1996 alternative rock
add_range "Britpop"                  1993      1997 rock
add_range "Digital music revolution" 1999      2010 milestone electronic

echo
echo "Built $FILE"
"$TIL" "$FILE" inspect
