# mtranslate — TODO

Backend service for **spelltrainer.com** (and general use). Rust/axum translator
(wraps Google's website API via `deeptrans`, cached in MySQL) — plus the TTS
(audio) side from berg/Spelltrainer's `gTTS` usage.

Endpoint format (existing): `GET /?s=<src>&t=<tgt>&v=<text>` → JSON
`Translated { target_value, target_hash, target_lang, source_lang,
source_hash, request_hash, source_value, msg }`.

## TO DO

### 1. DB: add `translit` column to all language tables
- Column belongs to the **source** word (its romanization "how to pronounce"),
  e.g. Thai word + its Latin transliteration.
- Add `translit varchar(255) NOT NULL DEFAULT ''` after `text` on every
  `{lang}` table (104 codes) + the linking cache as needed.
- Update the `CREATE TABLE` template in README:
  `( id int NOT NULL, hash varchar(16) NOT NULL DEFAULT '', text longtext NOT
  NULL DEFAULT '', translit varchar(255) NOT NULL DEFAULT '' )`.
- Provide a migration/ALTER loop over all language codes (idempotent,
  `ADD COLUMN IF NOT EXISTS`), so existing rows migrate without loss.

### 2. Code: return + cache the Google transliteration
- `src/translate.rs`: parse the transcription/romanization from the **same**
  Google mk1 payload already fetched (`Engine::Google` response) — no extra
  provider call.
- Add `translit` to the `Translated` struct + JSON response.
- Cache it with the source word in the source-language table:
  `INSERT ... (hash, text, translit)`; return on cache hits.
- Latin-script sources: translit empty (no romanization from Google) — JSON
  field present but blank.

### 3. new `/sound` endpoint — per-word mp3 (Google TTS, cached in spoken dictionaries)
- `GET /sound?s=<lang>&v=<word>` → `audio/mpeg` **or** redirect to the CDN mp3.
- **CDN first**: check the spoken-dictionary repo on GitHub Pages first —
  `https://myridia.github.io/spoken_dict_<lang>/sounds/<lang>/<word>.mp3`
  (filename = the word, URL-encoded). If that mp3 exists, serve/redirect it.
  Reference: `github/myridia/spoken_dict_th` (40,679 words, `sounds/th/<word>.mp3`,
  `words.txt` index). Create a `spoken_dict_<lang>` repo per language.
- **Google only on miss**: if the CDN returns 404, fetch from
  `https://translate.google.com/translate_tts?ie=UTF-8&client=tw-ob&q=<word>&tl=<lang>&ttsspeed=0.24`
  (port `gTTS` from berg/Spelltrainer: `client=tw-ob`, computed `tk` token,
  `slow=true` → `ttsspeed=0.24`), then add the mp3 to the language's spoken-dictionary
  repo (so Google is NOT hit again for that word). Also cache locally on disk:
  `cache/<lang>/<word>.mp3` (same scheme as the CDN, reuse `hash8` for the local key).
- Same politeness as the translator: exist-check + random wait
  (`wait_min`/`wait_max`) when actually calling Google.

## Depends on
- spelltrainer.com plan/Roadmap: Read section consumes translation + translit
  + mp3 for each source word.