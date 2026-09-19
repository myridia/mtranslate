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

### 3. new `/sound` endpoint — Google TTS audio (per-word mp3)
- `GET /sound?s=<lang>&v=<word>` → `audio/mpeg` (Google TTS, NOT synthetic —
  port `gTTS` from berg/Spelltrainer: `client=tw-ob`, computed `tk` token,
  `slow=true` → `ttsspeed=0.24`).
- Request: `https://translate.google.com/translate_tts?ie=UTF-8&client=tw-ob&q=<word>&tl=<lang>&ttsspeed=0.24`.
- Cache mp3 on disk: `cache/<lang>/<hash8(word)>.mp3` (reuse existing `hash8`),
  hit-check before re-fetching Google.
- Same politeness as the translator: exist-check + random wait
  (`wait_min`/`wait_max`) before hitting Google.

## Depends on
- spelltrainer.com plan/Roadmap: Read section consumes translation + translit
  + mp3 for each source word.